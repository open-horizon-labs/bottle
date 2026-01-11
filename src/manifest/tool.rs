use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tool definition - how to install a specific tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub binary: Option<String>,
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    pub registry: String,
    pub package: String,
    pub install: HashMap<String, String>,
    pub check: String,
    pub homepage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    Binary,
    Mcp,
}

impl ToolDefinition {
    /// Get the binary name (defaults to package name if not specified)
    pub fn binary_name(&self) -> &str {
        self.binary.as_deref().unwrap_or(&self.name)
    }
}
