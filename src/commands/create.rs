use crate::error::{BottleError, Result};
use crate::fetch::fetch_bottle_manifest;
use crate::manifest::bottle::BottleManifest;
use crate::ui;
use chrono::Utc;
use console::style;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

/// Get the path to the bespoke bottles directory
fn bespoke_bottles_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".bottle").join("bottles"))
}

/// Get the path to a specific bespoke bottle
fn bespoke_bottle_path(name: &str) -> Option<PathBuf> {
    bespoke_bottles_dir().map(|d| d.join(name))
}

/// Check if a bespoke bottle already exists locally
fn bespoke_bottle_exists(name: &str) -> bool {
    bespoke_bottle_path(name)
        .map(|p| p.join("manifest.json").exists())
        .unwrap_or(false)
}

/// Load a bespoke bottle manifest from local storage
fn load_bespoke_manifest(name: &str) -> Result<BottleManifest> {
    let path = bespoke_bottle_path(name)
        .ok_or_else(|| BottleError::Other("Could not determine home directory".to_string()))?
        .join("manifest.json");

    let contents = std::fs::read_to_string(&path).map_err(|_| {
        BottleError::BottleNotFound(format!("bespoke bottle '{}' not found", name))
    })?;

    serde_json::from_str(&contents).map_err(|e| BottleError::ParseError(e))
}

/// Create a new bespoke bottle
pub fn run(name: &str, from: Option<&str>) -> Result<()> {
    // Validate bottle name
    if name.is_empty() {
        return Err(BottleError::ValidationError(
            "Bottle name cannot be empty".to_string(),
        ));
    }

    // Check for invalid characters in name
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(BottleError::ValidationError(
            "Bottle name can only contain alphanumeric characters, hyphens, and underscores"
                .to_string(),
        ));
    }

    // Check if bottle already exists
    if bespoke_bottle_exists(name) {
        return Err(BottleError::ValidationError(format!(
            "Bespoke bottle '{}' already exists",
            name
        )));
    }

    // Create the bottle directory
    let bottle_dir = bespoke_bottle_path(name)
        .ok_or_else(|| BottleError::Other("Could not determine home directory".to_string()))?;

    std::fs::create_dir_all(&bottle_dir)?;

    // Create manifest
    let manifest = if let Some(source) = from {
        // Copy from source bottle
        let spinner = ui::spinner(&format!("Fetching manifest from '{}'...", source));

        // Try fetching as a curated bottle first, then as a bespoke bottle
        let source_manifest = fetch_bottle_manifest(source).or_else(|_| {
            // Try loading as bespoke bottle
            load_bespoke_manifest(source)
        });

        spinner.finish_and_clear();

        let source_manifest = source_manifest.map_err(|_| {
            BottleError::BottleNotFound(format!(
                "Source bottle '{}' not found (checked curated and bespoke)",
                source
            ))
        })?;

        // Create new manifest based on source
        BottleManifest {
            name: name.to_string(),
            version: Utc::now().format("%Y.%m.%d").to_string(),
            description: format!("Custom bottle based on {}", source),
            tools: source_manifest.tools,
            plugins: source_manifest.plugins,
            prerequisites: source_manifest.prerequisites,
            opencode_plugins: source_manifest.opencode_plugins,
        }
    } else {
        // Create template
        BottleManifest {
            name: name.to_string(),
            version: Utc::now().format("%Y.%m.%d").to_string(),
            description: "My custom tool versions".to_string(),
            tools: HashMap::new(),
            plugins: Vec::new(),
            prerequisites: HashMap::new(),
            opencode_plugins: HashMap::new(),
        }
    };

    // Write manifest
    let manifest_path = bottle_dir.join("manifest.json");
    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    std::fs::write(&manifest_path, &manifest_json)?;

    // Show success message
    ui::print_success(&format!("Created bespoke bottle: {}", name));
    println!();
    println!(
        "Location: {}",
        style(manifest_path.display().to_string()).cyan()
    );
    println!();

    // Try to open in editor
    if let Ok(editor) = std::env::var("EDITOR") {
        println!(
            "Opening manifest in {}...",
            style(&editor).cyan()
        );
        let _ = Command::new(&editor).arg(&manifest_path).status();
        println!();
    }

    // Show next steps
    println!("Edit the manifest to pin your desired versions, then:");
    println!("  {} {}", style("bottle install").cyan(), name);
    println!();
    println!(
        "{}",
        style("Note: Bespoke bottles are user-maintained. You're responsible").dim()
    );
    println!(
        "{}",
        style("for keeping the manifest compatible with future bottle versions.").dim()
    );

    Ok(())
}
