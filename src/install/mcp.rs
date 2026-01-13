use crate::error::{BottleError, Result};
use std::process::Command;

/// Register an MCP server with Claude
/// `name` is the MCP server name (e.g., "oh-mcp")
/// `package` is the npm package (e.g., "@cloud-atlas-ai/oh-mcp-server")
pub fn register(name: &str, package: &str, version: &str) -> Result<()> {
    let status = Command::new("claude")
        .args([
            "mcp",
            "add",
            name,
            "-s",
            "user",
            "--",
            "npx",
            "-y",
            &format!("{}@{}", package, version),
        ])
        .status()
        .map_err(|e| BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("Failed to run claude mcp add: {}", e),
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("claude mcp add exited with code {}", status),
        })
    }
}

/// Check if an MCP server is registered
pub fn is_registered(name: &str) -> bool {
    Command::new("claude")
        .args(["mcp", "list"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains(name))
        .unwrap_or(false)
}

/// Unregister an MCP server
pub fn unregister(name: &str) -> Result<()> {
    let status = Command::new("claude")
        .args(["mcp", "remove", name])
        .status()
        .map_err(|e| BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("Failed to run claude mcp remove: {}", e),
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("claude mcp remove exited with code {}", status),
        })
    }
}
