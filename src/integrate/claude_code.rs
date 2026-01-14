//! Claude Code integration
//!
//! Installs/removes the bottle plugins for Claude Code.

use crate::error::{BottleError, Result};
use std::process::Command;

/// Marketplace source (owner/repo format for GitHub)
const MARKETPLACE_SOURCE: &str = "open-horizon-labs/bottle";
/// Marketplace name (used in plugin install commands - must match marketplace.json "name")
const MARKETPLACE: &str = "open-horizon-labs";

/// All plugins to install (bottle + child plugins)
const ALL_PLUGINS: &[&str] = &["bottle", "ba", "superego", "wm", "oh-mcp", "miranda"];

/// Check if Claude Code is detected (has config directory)
pub fn is_detected() -> bool {
    dirs::home_dir()
        .map(|h| h.join(".claude").exists())
        .unwrap_or(false)
}

/// Check if ALL bottle plugins are installed in Claude Code
pub fn is_installed() -> bool {
    Command::new("claude")
        .args(["plugin", "list"])
        .output()
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Check if ALL plugins are installed (not just bottle)
            ALL_PLUGINS.iter().all(|plugin| {
                stdout.contains(&format!("{}@{}", plugin, MARKETPLACE))
            })
        })
        .unwrap_or(false)
}

/// Get list of missing plugins (not currently installed)
pub fn get_missing_plugins() -> Vec<String> {
    Command::new("claude")
        .args(["plugin", "list"])
        .output()
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            ALL_PLUGINS
                .iter()
                .filter(|plugin| !stdout.contains(&format!("{}@{}", plugin, MARKETPLACE)))
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_else(|_| ALL_PLUGINS.iter().map(|s| s.to_string()).collect())
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
    if stdout.contains(MARKETPLACE) || stdout.contains(MARKETPLACE_SOURCE) {
        return Ok(());
    }

    // Add the marketplace (using owner/repo format)
    let status = Command::new("claude")
        .args(["plugin", "marketplace", "add", MARKETPLACE_SOURCE])
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
            reason: format!("Failed to add marketplace '{}'", MARKETPLACE_SOURCE),
        })
    }
}

/// Install all bottle plugins for Claude Code
pub fn install() -> Result<()> {
    // First, ensure the marketplace is added
    ensure_marketplace()?;

    let mut failures: Vec<String> = Vec::new();

    for plugin in ALL_PLUGINS {
        let status = Command::new("claude")
            .args([
                "plugin",
                "install",
                &format!("{}@{}", plugin, MARKETPLACE),
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
                &format!("{}@{}", plugin, MARKETPLACE),
            ])
            .status()
            .map_err(|e| BottleError::InstallError {
                tool: format!("claude_code integration ({})", plugin),
                reason: format!("Failed to run claude plugin uninstall: {}", e),
            })?;

        if !status.success() {
            // Don't fail on uninstall errors - plugin might not have been installed
            failures.push(plugin.to_string());
        }
    }

    // Only fail if ALL plugins failed to uninstall
    if failures.len() == ALL_PLUGINS.len() {
        Err(BottleError::InstallError {
            tool: "claude_code integration".to_string(),
            reason: "Failed to uninstall any plugins".to_string(),
        })
    } else {
        Ok(())
    }
}
