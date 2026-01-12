use crate::error::{BottleError, Result};
use crate::fetch::fetch_bottle_manifest;
use crate::manifest::bottle::BottleManifest;
use std::fs;

/// Marketplace identifier for plugins
pub const MARKETPLACE: &str = "cloud-atlas-ai/bottle";

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
