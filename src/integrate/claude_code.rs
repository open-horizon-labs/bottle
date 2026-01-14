//! Claude Code integration
//!
//! Installs/removes the bottle plugins for Claude Code.

use crate::error::{BottleError, Result};
use std::process::Command;

/// Marketplace repo (owner/repo format for GitHub)
const MARKETPLACE_REPO: &str = "open-horizon-labs/bottle";
/// Marketplace name (used in plugin install commands - must match marketplace.json "name")
const MARKETPLACE_NAME: &str = "open-horizon-labs";
/// Old marketplace name (for migration cleanup)
const OLD_MARKETPLACE_NAME: &str = "bottle";

/// All plugins to install (bottle + child plugins)
const ALL_PLUGINS: &[&str] = &["bottle", "ba", "superego", "wm", "oh-mcp", "miranda"];

/// Check if Claude Code is detected (has config directory)
pub fn is_detected() -> bool {
    dirs::home_dir()
        .map(|h| h.join(".claude").exists())
        .unwrap_or(false)
}

/// Check if ALL bottle plugins are installed in Claude Code
/// Reads ~/.claude/plugins/installed_plugins.json directly
pub fn is_installed() -> bool {
    let installed = get_installed_plugins();
    ALL_PLUGINS.iter().all(|plugin| {
        let key = format!("{}@{}", plugin, MARKETPLACE_NAME);
        installed.contains(&key)
    })
}

/// Get list of currently installed plugin keys from Claude Code
fn get_installed_plugins() -> Vec<String> {
    dirs::home_dir()
        .map(|h| h.join(".claude/plugins/installed_plugins.json"))
        .and_then(|path| std::fs::read_to_string(path).ok())
        .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).ok())
        .and_then(|json| {
            json.get("plugins")
                .and_then(|p| p.as_object())
                .map(|plugins| plugins.keys().cloned().collect())
        })
        .unwrap_or_default()
}

/// Get list of missing plugins (not currently installed)
#[allow(dead_code)]
pub fn get_missing_plugins() -> Vec<String> {
    let installed = get_installed_plugins();
    ALL_PLUGINS
        .iter()
        .filter(|plugin| {
            let key = format!("{}@{}", plugin, MARKETPLACE_NAME);
            !installed.contains(&key)
        })
        .map(|s| s.to_string())
        .collect()
}

/// Add the marketplace if not already added
fn ensure_marketplace() -> Result<()> {
    // Check if marketplace is already added
    let output = Command::new("claude")
        .args(["plugin", "marketplace", "list"])
        .output()
        .map_err(|e| BottleError::InstallError {
            tool: "claude_code integration".to_string(),
            reason: format!("Failed to list marketplaces: {}", e),
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains(MARKETPLACE_NAME) || stdout.contains(MARKETPLACE_REPO) {
        return Ok(());
    }

    // Add the marketplace (using owner/repo format)
    let status = Command::new("claude")
        .args(["plugin", "marketplace", "add", MARKETPLACE_REPO])
        .status()
        .map_err(|e| BottleError::InstallError {
            tool: "claude_code integration".to_string(),
            reason: format!("Failed to add marketplace: {}", e),
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: "claude_code integration".to_string(),
            reason: format!("Failed to add marketplace '{}'", MARKETPLACE_REPO),
        })
    }
}

/// Clean up old @bottle marketplace entries from installed_plugins.json and settings.json
/// This fixes the "Plugin not found in marketplace 'bottle'" error for users
/// who had plugins installed before the marketplace was renamed to 'open-horizon-labs'
pub fn cleanup_old_marketplace_entries() {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return,
    };

    // Clean up installed_plugins.json
    cleanup_installed_plugins(&home);

    // Clean up settings.json (enabledPlugins)
    cleanup_settings(&home);
}

/// Remove old @bottle entries from installed_plugins.json
fn cleanup_installed_plugins(home: &std::path::Path) {
    let path = home.join(".claude/plugins/installed_plugins.json");

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(j) => j,
        Err(_) => return,
    };

    if let Some(plugins) = json.get_mut("plugins").and_then(|p| p.as_object_mut()) {
        let old_keys: Vec<String> = plugins
            .keys()
            .filter(|k| k.ends_with(&format!("@{}", OLD_MARKETPLACE_NAME)))
            .cloned()
            .collect();

        if !old_keys.is_empty() {
            eprintln!(
                "Cleaning up old installed_plugins entries: {}",
                old_keys.join(", ")
            );
            for key in old_keys {
                plugins.remove(&key);
            }

            if let Ok(new_content) = serde_json::to_string_pretty(&json) {
                let _ = std::fs::write(&path, new_content);
            }
        }
    }
}

/// Remove old @bottle entries from settings.json enabledPlugins
fn cleanup_settings(home: &std::path::Path) {
    let path = home.join(".claude/settings.json");

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(j) => j,
        Err(_) => return,
    };

    if let Some(enabled) = json
        .get_mut("enabledPlugins")
        .and_then(|p| p.as_object_mut())
    {
        let old_keys: Vec<String> = enabled
            .keys()
            .filter(|k| k.ends_with(&format!("@{}", OLD_MARKETPLACE_NAME)))
            .cloned()
            .collect();

        if !old_keys.is_empty() {
            eprintln!(
                "Cleaning up old settings entries: {}",
                old_keys.join(", ")
            );
            for key in old_keys {
                enabled.remove(&key);
            }

            if let Ok(new_content) = serde_json::to_string_pretty(&json) {
                let _ = std::fs::write(&path, new_content);
            }
        }
    }
}

/// Install all bottle plugins for Claude Code
pub fn install() -> Result<()> {
    // Clean up old @bottle entries that cause "Plugin not found in marketplace 'bottle'" errors
    cleanup_old_marketplace_entries();

    // Ensure the marketplace is added
    ensure_marketplace()?;

    let mut failures: Vec<String> = Vec::new();

    for plugin in ALL_PLUGINS {
        let status = Command::new("claude")
            .args([
                "plugin",
                "install",
                &format!("{}@{}", plugin, MARKETPLACE_NAME),
            ])
            .status()
            .map_err(|e| BottleError::InstallError {
                tool: format!("claude_code integration ({})", plugin),
                reason: format!("Failed to run claude plugin install: {}", e),
            })?;

        if !status.success() {
            failures.push(plugin.to_string());
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: "claude_code integration".to_string(),
            reason: format!("Failed to install plugins: {}", failures.join(", ")),
        })
    }
}

/// Remove all bottle plugins from Claude Code
pub fn remove() -> Result<()> {
    let mut failures: Vec<String> = Vec::new();

    // Remove in reverse order (children first, then bottle)
    for plugin in ALL_PLUGINS.iter().rev() {
        let status = Command::new("claude")
            .args([
                "plugin",
                "uninstall",
                &format!("{}@{}", plugin, MARKETPLACE_NAME),
            ])
            .status()
            .map_err(|e| BottleError::UninstallError {
                tool: format!("claude_code integration ({})", plugin),
                reason: format!("Failed to run claude plugin uninstall: {}", e),
            })?;

        if !status.success() {
            // Don't fail on uninstall errors - plugin might not have been installed
            failures.push(plugin.to_string());
        }
    }

    // Warn about partial failures but don't error
    if !failures.is_empty() && failures.len() < ALL_PLUGINS.len() {
        eprintln!(
            "Note: Some plugins couldn't be removed (may not have been installed): {}",
            failures.join(", ")
        );
    }

    // Only fail if ALL plugins failed to uninstall
    if failures.len() == ALL_PLUGINS.len() {
        Err(BottleError::UninstallError {
            tool: "claude_code integration".to_string(),
            reason: "Failed to uninstall any plugins".to_string(),
        })
    } else {
        Ok(())
    }
}
