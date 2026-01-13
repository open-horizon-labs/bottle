//! OpenCode integration
//!
//! Adds/removes the bottle ecosystem plugins to opencode.json.
//!
//! AIDEV-NOTE: Config file resolution is cwd-first, then home directory.
//! This means `bottle integrate opencode` modifies the local project's
//! opencode.json if it exists, otherwise the global ~/.opencode.json.
//! This matches how opencode itself resolves config files.

use crate::error::{BottleError, Result};
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

/// NPM packages for the OpenCode integration - full ecosystem
const PACKAGES: &[&str] = &[
    "@cloud-atlas-ai/bottle",
    "@cloud-atlas-ai/ba-opencode",
    "@cloud-atlas-ai/wm-opencode",
    "@cloud-atlas-ai/superego-opencode",
];

/// Check if OpenCode is detected (has ~/.opencode/ directory or opencode binary)
pub fn is_detected() -> bool {
    // Check for ~/.opencode/ directory
    dirs::home_dir()
        .map(|h| h.join(".opencode").exists())
        .unwrap_or(false)
        // Or check if opencode binary is installed
        || std::process::Command::new("which")
            .arg("opencode")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
}

/// Get the path to opencode.json (cwd first, then home)
fn get_config_path() -> Option<PathBuf> {
    // Check current directory first
    let cwd_config = PathBuf::from("opencode.json");
    if cwd_config.exists() {
        return Some(cwd_config);
    }

    // Check home directory
    dirs::home_dir()
        .map(|h| h.join("opencode.json"))
        .filter(|p| p.exists())
}

/// Check if the bottle plugins are installed in OpenCode config
#[allow(dead_code)]
pub fn is_installed() -> bool {
    let Some(config_path) = get_config_path() else {
        return false;
    };

    let Ok(contents) = fs::read_to_string(&config_path) else {
        return false;
    };

    let Ok(config): std::result::Result<Value, _> = serde_json::from_str(&contents) else {
        return false;
    };

    // Check if main bottle package is in plugins array
    config
        .get("plugins")
        .and_then(|p| p.as_array())
        .map(|plugins| plugins.iter().any(|p| p.as_str() == Some(PACKAGES[0])))
        .unwrap_or(false)
}

/// Install the bottle ecosystem plugins into OpenCode config
pub fn install() -> Result<()> {
    let config_path = get_config_path().ok_or_else(|| BottleError::InstallError {
        tool: "opencode integration".to_string(),
        reason: "No opencode.json found in current directory or home".to_string(),
    })?;

    // Read existing config
    let contents = fs::read_to_string(&config_path).map_err(|e| BottleError::InstallError {
        tool: "opencode integration".to_string(),
        reason: format!("Failed to read opencode.json: {}", e),
    })?;

    let mut config: Value = serde_json::from_str(&contents).map_err(|e| {
        BottleError::InstallError {
            tool: "opencode integration".to_string(),
            reason: format!("Failed to parse opencode.json: {}", e),
        }
    })?;

    // Get or create plugins array
    let plugins = config
        .as_object_mut()
        .ok_or_else(|| BottleError::InstallError {
            tool: "opencode integration".to_string(),
            reason: "opencode.json is not an object".to_string(),
        })?
        .entry("plugins")
        .or_insert_with(|| json!([]));

    let plugins_array = plugins.as_array_mut().ok_or_else(|| BottleError::InstallError {
        tool: "opencode integration".to_string(),
        reason: "plugins field is not an array".to_string(),
    })?;

    // Add all ecosystem packages (idempotent - skip already installed)
    for package in PACKAGES {
        if !plugins_array.iter().any(|p| p.as_str() == Some(*package)) {
            plugins_array.push(json!(*package));
        }
    }

    // Write back
    let updated = serde_json::to_string_pretty(&config).map_err(|e| BottleError::InstallError {
        tool: "opencode integration".to_string(),
        reason: format!("Failed to serialize config: {}", e),
    })?;

    fs::write(&config_path, updated).map_err(|e| BottleError::InstallError {
        tool: "opencode integration".to_string(),
        reason: format!("Failed to write opencode.json: {}", e),
    })?;

    Ok(())
}

/// Remove the bottle ecosystem plugins from OpenCode config
pub fn remove() -> Result<()> {
    let Some(config_path) = get_config_path() else {
        return Ok(()); // No config, nothing to remove
    };

    // Read existing config
    let contents = fs::read_to_string(&config_path).map_err(|e| BottleError::InstallError {
        tool: "opencode integration".to_string(),
        reason: format!("Failed to read opencode.json: {}", e),
    })?;

    let mut config: Value = serde_json::from_str(&contents).map_err(|e| {
        BottleError::InstallError {
            tool: "opencode integration".to_string(),
            reason: format!("Failed to parse opencode.json: {}", e),
        }
    })?;

    // Get plugins array
    let Some(plugins) = config.get_mut("plugins").and_then(|p| p.as_array_mut()) else {
        return Ok(()); // No plugins array, nothing to remove
    };

    // Remove all ecosystem packages
    plugins.retain(|p| {
        p.as_str()
            .map(|s| !PACKAGES.contains(&s))
            .unwrap_or(true)
    });

    // Write back
    let updated = serde_json::to_string_pretty(&config).map_err(|e| BottleError::InstallError {
        tool: "opencode integration".to_string(),
        reason: format!("Failed to serialize config: {}", e),
    })?;

    fs::write(&config_path, updated).map_err(|e| BottleError::InstallError {
        tool: "opencode integration".to_string(),
        reason: format!("Failed to write opencode.json: {}", e),
    })?;

    Ok(())
}
