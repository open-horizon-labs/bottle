use crate::error::{BottleError, Result};
use std::process::Command;

/// Install a crate using cargo
pub fn install(package: &str, version: &str) -> Result<()> {
    let mut args = vec!["install".to_string()];

    // Support "latest" by omitting version (cargo will fetch latest)
    if version.is_empty() || version == "latest" {
        args.push(package.to_string());
    } else {
        args.push(format!("{}@{}", package, version));
    }

    let status =
        Command::new("cargo")
            .args(&args)
            .status()
            .map_err(|e| BottleError::InstallError {
                tool: package.to_string(),
                reason: format!("Failed to run cargo: {}", e),
            })?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: package.to_string(),
            reason: format!("cargo install exited with code {}", status),
        })
    }
}

/// Check if cargo is available
pub fn is_available() -> bool {
    which::which("cargo").is_ok()
}
