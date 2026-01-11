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
