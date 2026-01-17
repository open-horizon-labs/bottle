use crate::error::{BottleError, Result};
use std::process::Command;

/// Install a Claude Code plugin
pub fn install(plugin: &str, marketplace: &str) -> Result<()> {
    let status = Command::new("claude")
        .args(["plugin", "install", &format!("{}@{}", plugin, marketplace)])
        .status()
        .map_err(|e| BottleError::InstallError {
            tool: plugin.to_string(),
            reason: format!("Failed to run claude plugin install: {}", e),
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: plugin.to_string(),
            reason: format!("claude plugin install exited with code {}", status),
        })
    }
}

/// Uninstall a Claude Code plugin
pub fn uninstall(plugin: &str, marketplace: &str) -> Result<()> {
    let status = Command::new("claude")
        .args(["plugin", "uninstall", &format!("{}@{}", plugin, marketplace)])
        .status()
        .map_err(|e| BottleError::InstallError {
            tool: plugin.to_string(),
            reason: format!("Failed to run claude plugin uninstall: {}", e),
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: plugin.to_string(),
            reason: format!("claude plugin uninstall exited with code {}", status),
        })
    }
}

/// Update Claude Code plugins from a marketplace
pub fn update_marketplace(marketplace: &str) -> Result<()> {
    let status = Command::new("claude")
        .args(["plugin", "marketplace", "update", marketplace])
        .status()
        .map_err(|e| BottleError::Other(format!("Failed to run claude plugin marketplace update: {}", e)))?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::Other(format!(
            "claude plugin marketplace update exited with code {}",
            status
        )))
    }
}

/// Update a single Claude Code plugin from a marketplace
#[allow(dead_code)]
pub fn update_plugin(plugin: &str, marketplace: &str) -> Result<bool> {
    let output = Command::new("claude")
        .args(["plugin", "update", &format!("{}@{}", plugin, marketplace)])
        .output()
        .map_err(|e| BottleError::Other(format!("Failed to run claude plugin update: {}", e)))?;

    if output.status.success() {
        // Check if it was actually updated vs already at latest
        // Combine stdout and stderr, check case-insensitively
        let mut text = String::new();
        text.push_str(&String::from_utf8_lossy(&output.stdout));
        text.push_str(&String::from_utf8_lossy(&output.stderr));
        let text = text.to_lowercase();
        let updated = text.contains("updated from") || text.contains("updated to");
        let up_to_date = text.contains("up to date") || text.contains("already at");
        Ok(updated && !up_to_date)
    } else {
        Err(BottleError::Other(format!(
            "claude plugin update exited with code {}",
            output.status
        )))
    }
}
