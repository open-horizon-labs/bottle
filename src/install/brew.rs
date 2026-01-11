use crate::error::{BottleError, Result};
use std::process::Command;

/// Install a formula using Homebrew
pub fn install(package: &str, _version: &str) -> Result<()> {
    // Note: Homebrew doesn't support pinned versions easily
    // This is a simplified implementation
    let status = Command::new("brew")
        .args(["install", package])
        .status()
        .map_err(|e| BottleError::InstallError {
            tool: package.to_string(),
            reason: format!("Failed to run brew: {}", e),
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: package.to_string(),
            reason: format!("brew install exited with code {}", status),
        })
    }
}

/// Check if Homebrew is available
pub fn is_available() -> bool {
    which::which("brew").is_ok()
}
