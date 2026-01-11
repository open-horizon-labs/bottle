pub mod brew;
pub mod cargo;
pub mod mcp;
pub mod plugin;

use crate::error::Result;
use crate::manifest::tool::{ToolDefinition, ToolType};

/// Install a tool using the appropriate method
pub fn install_tool(tool: &ToolDefinition, version: &str) -> Result<()> {
    match tool.tool_type {
        ToolType::Binary => {
            // Try cargo first, fall back to brew
            if which::which("cargo").is_ok() {
                cargo::install(&tool.package, version)
            } else if which::which("brew").is_ok() {
                brew::install(&tool.package, version)
            } else {
                Err(crate::error::BottleError::PrerequisitesNotMet(
                    "Neither cargo nor brew found. Install Rust or Homebrew.".into(),
                ))
            }
        }
        ToolType::Mcp => mcp::register(&tool.package, version),
    }
}

/// Check if a tool is installed
pub fn check_tool(tool: &ToolDefinition) -> bool {
    match tool.tool_type {
        ToolType::Binary => which::which(tool.binary_name()).is_ok(),
        ToolType::Mcp => mcp::is_registered(&tool.name),
    }
}
