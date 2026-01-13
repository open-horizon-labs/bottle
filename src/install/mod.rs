pub mod brew;
pub mod cargo;
pub mod mcp;
pub mod plugin;

use crate::error::Result;
use crate::manifest::state::InstallMethod;
use crate::manifest::tool::{ToolDefinition, ToolType};

/// Install a tool using the appropriate method, returns the method actually used
pub fn install_tool(tool: &ToolDefinition, version: &str) -> Result<InstallMethod> {
    match tool.tool_type {
        ToolType::Binary => {
            // Try cargo first, fall back to brew
            if which::which("cargo").is_ok() {
                cargo::install(&tool.package, version)?;
                Ok(InstallMethod::Cargo)
            } else if which::which("brew").is_ok() {
                brew::install(&tool.package, version)?;
                Ok(InstallMethod::Brew)
            } else {
                Err(crate::error::BottleError::PrerequisitesNotMet(
                    "Neither cargo nor brew found. Install Rust or Homebrew.".into(),
                ))
            }
        }
        ToolType::Mcp => {
            mcp::register(&tool.name, &tool.package, version)?;
            Ok(InstallMethod::Mcp)
        }
    }
}

/// Check if a tool is installed
pub fn check_tool(tool: &ToolDefinition) -> bool {
    match tool.tool_type {
        ToolType::Binary => which::which(tool.binary_name()).is_ok(),
        ToolType::Mcp => mcp::is_registered(&tool.name),
    }
}
