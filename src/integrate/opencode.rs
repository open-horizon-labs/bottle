//! OpenCode integration
//!
//! Adds/removes the bottle ecosystem plugin to opencode.json.
//!
//! AIDEV-NOTE: Config file resolution is cwd-first, then home directory.
//! This means `bottle integrate opencode` modifies the local project's
//! opencode.json if it exists, otherwise the global ~/.opencode.json.
//! This matches how opencode itself resolves config files.

use crate::error::{BottleError, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Default NPM packages for OpenCode integration (fallback if no manifest)
const DEFAULT_PACKAGES: &[&str] = &[
    "@cloud-atlas-ai/bottle",
    "ba-opencode",
    "wm-opencode",
    "superego-opencode",
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

/// Get the path to opencode.json (cwd first, then global config)
/// AIDEV-NOTE: OpenCode uses XDG-style paths (~/.config/) on all platforms,
/// not the native config directories. Do NOT use dirs::config_dir() here.
fn get_config_path() -> Option<PathBuf> {
    // Check current directory first (project config)
    let cwd_config = PathBuf::from("opencode.json");
    if cwd_config.exists() {
        return Some(cwd_config);
    }

    // Check global config at ~/.config/opencode/opencode.json (XDG style)
    dirs::home_dir()
        .map(|h| h.join(".config").join("opencode").join("opencode.json"))
        .filter(|p| p.exists())
}

/// Get the default config path for creating new config
/// AIDEV-NOTE: Always use XDG-style ~/.config/opencode/ for OpenCode
fn default_config_path() -> PathBuf {
    dirs::home_dir()
        .map(|h| h.join(".config").join("opencode").join("opencode.json"))
        .unwrap_or_else(|| PathBuf::from("opencode.json"))
}

/// Check if the bottle plugin are installed in OpenCode config
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

    // Check if main bottle package is in plugin array (with or without version)
    config
        .get("plugin")
        .and_then(|p| p.as_array())
        .map(|plugin| {
            plugin.iter().any(|p| {
                p.as_str()
                    .map(|s| s.starts_with(DEFAULT_PACKAGES[0]))
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

/// Install the bottle ecosystem plugin into OpenCode config
/// If opencode_plugins map is provided, use versioned package references (e.g., "ba-opencode@0.2.1")
pub fn install(opencode_plugins: Option<&HashMap<String, String>>) -> Result<()> {
    let config_path = get_config_path().unwrap_or_else(default_config_path);

    // Read existing config or create new one
    let mut config: Value = if config_path.exists() {
        let contents = fs::read_to_string(&config_path).map_err(|e| BottleError::InstallError {
            tool: "opencode integration".to_string(),
            reason: format!("Failed to read opencode.json: {}", e),
        })?;
        serde_json::from_str(&contents).map_err(|e| {
            BottleError::InstallError {
                tool: "opencode integration".to_string(),
                reason: format!("Failed to parse opencode.json: {}", e),
            }
        })?
    } else {
        // Create new config with schema
        json!({
            "$schema": "https://opencode.ai/config.json"
        })
    };

    // Get or create plugin array
    let plugin = config
        .as_object_mut()
        .ok_or_else(|| BottleError::InstallError {
            tool: "opencode integration".to_string(),
            reason: "opencode.json is not an object".to_string(),
        })?
        .entry("plugin")
        .or_insert_with(|| json!([]));

    let plugin_array = plugin.as_array_mut().ok_or_else(|| BottleError::InstallError {
        tool: "opencode integration".to_string(),
        reason: "plugin field is not an array".to_string(),
    })?;

    // Build package list: use versioned if manifest provided, otherwise defaults
    let packages: Vec<String> = if let Some(plugins) = opencode_plugins {
        plugins
            .iter()
            .map(|(name, version)| format!("{}@{}", name, version))
            .collect()
    } else {
        DEFAULT_PACKAGES.iter().map(|s| s.to_string()).collect()
    };

    // Add all ecosystem packages (idempotent - skip if package name already present)
    for package in &packages {
        let package_name = package.split('@').next().unwrap_or(package);
        // Remove any existing entry for this package (to update version)
        plugin_array.retain(|p| {
            p.as_str()
                .map(|s| !s.starts_with(package_name))
                .unwrap_or(true)
        });
        plugin_array.push(json!(package));
    }

    // Write back
    let updated = serde_json::to_string_pretty(&config).map_err(|e| BottleError::InstallError {
        tool: "opencode integration".to_string(),
        reason: format!("Failed to serialize config: {}", e),
    })?;

    // Create parent directory if needed (for new config files)
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| BottleError::InstallError {
            tool: "opencode integration".to_string(),
            reason: format!("Failed to create config directory: {}", e),
        })?;
    }

    fs::write(&config_path, updated).map_err(|e| BottleError::InstallError {
        tool: "opencode integration".to_string(),
        reason: format!("Failed to write opencode.json: {}", e),
    })?;

    Ok(())
}

/// Remove the bottle ecosystem plugin from OpenCode config
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

    // Get plugin array
    let Some(plugin) = config.get_mut("plugin").and_then(|p| p.as_array_mut()) else {
        return Ok(()); // No plugin array, nothing to remove
    };

    // Remove all ecosystem packages
    plugin.retain(|p| {
        p.as_str()
            .map(|s| !DEFAULT_PACKAGES.contains(&s))
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
