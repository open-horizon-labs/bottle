use crate::error::{BottleError, Result};
use crate::fetch::fetch_bottle_manifest;
use crate::manifest::bottle::BottleManifest;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

/// Marketplace identifier for plugins
pub const MARKETPLACE: &str = "open-horizon-labs";

/// Fetch manifest from bespoke location or GitHub
pub fn fetch_or_load_manifest(bottle: &str) -> Result<BottleManifest> {
    // Check bespoke first (~/.bottle/bottles/<name>/)
    if let Some(home) = dirs::home_dir() {
        let bespoke_path = home
            .join(".bottle")
            .join("bottles")
            .join(bottle)
            .join("manifest.json");

        if bespoke_path.exists() {
            let contents = fs::read_to_string(&bespoke_path)
                .map_err(|e| BottleError::Other(format!("Failed to read bespoke manifest: {}", e)))?;
            return serde_json::from_str(&contents).map_err(BottleError::ParseError);
        }
    }

    // Fall back to curated (fetch from GitHub)
    fetch_bottle_manifest(bottle)
}

/// Check that required prerequisites are available
pub fn check_prerequisites(manifest: &BottleManifest) -> Result<()> {
    let mut missing = Vec::new();

    if manifest.prerequisites.contains_key("cargo") && !crate::install::cargo::is_available() {
        missing.push("cargo (install Rust: https://rustup.rs)");
    }

    if manifest.prerequisites.contains_key("node") && which::which("node").is_err() {
        missing.push("node (install Node.js: https://nodejs.org)");
    }

    if !missing.is_empty() {
        return Err(BottleError::PrerequisitesNotMet(missing.join(", ")));
    }

    Ok(())
}

/// Get the path to a local bottle manifest (curator command helper).
///
/// This function is used by curator commands (upgrade, validate, release) that
/// operate on local bottle manifests in the repository's bottles/ directory.
///
/// # Security
/// Validates the bottle name to prevent path traversal attacks by rejecting
/// names containing path separators or parent directory references.
pub fn get_local_manifest_path(bottle: &str) -> Result<PathBuf> {
    // Reject bottle names with path separators or traversal attempts
    if bottle.contains('/') || bottle.contains('\\') || bottle.contains("..") {
        return Err(BottleError::Other(format!(
            "Invalid bottle name '{}': must not contain path separators or '..'",
            bottle
        )));
    }

    // Reject empty or whitespace-only names
    if bottle.trim().is_empty() {
        return Err(BottleError::Other(
            "Invalid bottle name: cannot be empty".to_string(),
        ));
    }

    // Construct path using PathBuf for proper path handling
    let local_path = PathBuf::from("bottles").join(bottle).join("manifest.json");

    if local_path.exists() {
        return Ok(local_path);
    }

    Err(BottleError::BottleNotFound(format!(
        "No local manifest found at bottles/{}/manifest.json. Run from bottle repo root.",
        bottle
    )))
}

/// Build AGENTS.md snippet content from a manifest.
/// Returns None if no agents_md config or empty sections/no URL.
/// Returns the snippet content on success, or an error if URL fetch fails.
pub fn build_agents_md_snippet(manifest: &BottleManifest) -> Result<Option<String>> {
    let Some(agents_config) = &manifest.agents_md else {
        return Ok(None);
    };

    // Skip if no sections and no snippets_url
    if agents_config.sections.is_empty() && agents_config.snippets_url.is_none() {
        return Ok(None);
    }

    // Build snippet content
    let mut snippet = String::new();
    snippet.push_str(&format!("<!-- Bottle snippet for {} -->\n\n", manifest.name));

    // Add inline sections
    for section in &agents_config.sections {
        snippet.push_str(&section.heading);
        snippet.push_str("\n\n");
        snippet.push_str(&section.content);
        snippet.push_str("\n\n");
    }

    // Fetch snippets from URL if provided
    if let Some(url) = &agents_config.snippets_url {
        let content = fetch_snippets_url(url)?;
        snippet.push_str(&content);
        snippet.push_str("\n");
    }

    Ok(Some(snippet))
}

/// Fetch content from a snippets URL (HTTPS only)
fn fetch_snippets_url(url: &str) -> Result<String> {
    // Enforce HTTPS for security (prevents MITM injection of malicious instructions)
    if !url.starts_with("https://") {
        return Err(BottleError::Other(format!(
            "Snippets URL must use HTTPS: {}",
            url
        )));
    }

    // Build client with timeout
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| BottleError::Other(format!("Failed to create HTTP client: {}", e)))?;

    let response = client.get(url).send().map_err(|e| {
        BottleError::Other(format!("Failed to fetch {}: {}", url, e))
    })?;

    if !response.status().is_success() {
        return Err(BottleError::Other(format!(
            "Failed to fetch {}: HTTP {}",
            url,
            response.status()
        )));
    }

    response
        .text()
        .map_err(|e| BottleError::Other(format!("Failed to read response from {}: {}", url, e)))
}
