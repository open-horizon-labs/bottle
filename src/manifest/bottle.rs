use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Bottle manifest - defines a curated snapshot of tool versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub tools: HashMap<String, String>,
    pub plugins: Vec<String>,
    #[serde(default)]
    pub prerequisites: HashMap<String, String>,
    /// OpenCode plugin versions (package name -> version)
    #[serde(default)]
    pub opencode_plugins: HashMap<String, String>,
}

impl BottleManifest {
    /// Get the list of tool names
    pub fn tool_names(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }

    /// Get the version of a specific tool
    pub fn tool_version(&self, tool: &str) -> Option<&str> {
        self.tools.get(tool).map(|s| s.as_str())
    }
}
