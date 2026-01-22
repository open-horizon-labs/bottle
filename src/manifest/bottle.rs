use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MCP server definition for bespoke bottles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerDef {
    /// Command to run (e.g., "npx", "node")
    pub command: String,
    /// Arguments to pass to the command
    #[serde(default)]
    pub args: Vec<String>,
    /// Environment variables (supports ${VAR} syntax for required vars)
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Scope: "user" or "project" (default: "user")
    #[serde(default = "default_scope")]
    pub scope: String,
}

fn default_scope() -> String {
    "user".to_string()
}

/// AGENTS.md section to inject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsMdSection {
    /// Section heading (e.g., "## Design Review Protocol")
    pub heading: String,
    /// Section content (markdown)
    pub content: String,
}

/// AGENTS.md configuration for bespoke bottles
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentsMdConfig {
    /// Inline sections to inject
    #[serde(default)]
    pub sections: Vec<AgentsMdSection>,
    /// URL to fetch additional snippet content from
    #[serde(default)]
    pub snippets_url: Option<String>,
}

/// Custom tool installation methods
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomToolInstall {
    /// Homebrew formula (e.g., "internal-tap/cli")
    #[serde(default)]
    pub brew: Option<String>,
    /// Cargo crate name
    #[serde(default)]
    pub cargo: Option<String>,
    /// npm package name
    #[serde(default)]
    pub npm: Option<String>,
    /// Direct binary URL (supports {arch} placeholder)
    #[serde(default)]
    pub binary_url: Option<String>,
}

/// Custom tool definition for bespoke bottles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomToolDef {
    /// Installation methods (tried in order: brew -> cargo -> npm -> binary)
    pub install: CustomToolInstall,
    /// Version to install
    pub version: String,
    /// Command to verify installation (e.g., "cli --version")
    #[serde(default)]
    pub verify: Option<String>,
}

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
    /// MCP servers to register (bespoke bottles)
    #[serde(default)]
    pub mcp_servers: HashMap<String, McpServerDef>,
    /// AGENTS.md configuration (bespoke bottles)
    #[serde(default)]
    pub agents_md: Option<AgentsMdConfig>,
    /// Custom tools to install (bespoke bottles)
    #[serde(default)]
    pub custom_tools: HashMap<String, CustomToolDef>,
}
