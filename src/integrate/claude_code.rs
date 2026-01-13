//! Claude Code integration
//!
//! Installs/removes the bottle plugin for Claude Code.

use crate::error::{BottleError, Result};
use std::process::Command;

/// Marketplace and plugin name for Claude Code integration
const MARKETPLACE: &str = "cloud-atlas-ai/bottle";
const PLUGIN: &str = "bottle";

/// Check if Claude Code is detected (has config directory)
pub fn is_detected() -> bool {
    dirs::home_dir()
        .map(|h| h.join(".claude").exists())
        .unwrap_or(false)
}

/// Check if the bottle plugin is installed in Claude Code
#[allow(dead_code)]
pub fn is_installed() -> bool {
    Command::new("claude")
        .args(["plugin", "list"])
        .output()
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // AIDEV-NOTE: This checks for the plugin name in the list output.
            // The exact format depends on claude CLI output. May need adjustment.
            stdout.contains(PLUGIN) && stdout.contains(MARKETPLACE)
        })
        .unwrap_or(false)
}

/// Install the bottle plugin for Claude Code
pub fn install() -> Result<()> {
    let status = Command::new("claude")
        .args([
            "plugin",
            "install",
            &format!("{}@{}", PLUGIN, MARKETPLACE),
        ])
        .status()
        .map_err(|e| BottleError::InstallError {
            tool: "claude_code integration".to_string(),
            reason: format!("Failed to run claude plugin install: {}", e),
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: "claude_code integration".to_string(),
            reason: format!("claude plugin install exited with code {}", status),
        })
    }
}

/// Remove the bottle plugin from Claude Code
pub fn remove() -> Result<()> {
    let status = Command::new("claude")
        .args([
            "plugin",
            "uninstall",
            &format!("{}@{}", PLUGIN, MARKETPLACE),
        ])
        .status()
        .map_err(|e| BottleError::InstallError {
            tool: "claude_code integration".to_string(),
            reason: format!("Failed to run claude plugin uninstall: {}", e),
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: "claude_code integration".to_string(),
            reason: format!("claude plugin uninstall exited with code {}", status),
        })
    }
}
