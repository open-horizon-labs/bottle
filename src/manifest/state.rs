use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// User state - tracks installed bottle and tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleState {
    pub bottle: String,
    pub bottle_version: String,
    pub installed_at: DateTime<Utc>,
    pub tools: HashMap<String, ToolState>,
    pub mode: Mode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolState {
    pub version: String,
    pub installed_at: DateTime<Utc>,
    pub method: InstallMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Managed,
    Ejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstallMethod {
    Cargo,
    Brew,
    Mcp,
}

impl BottleState {
    /// Get the path to the state file
    pub fn state_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".bottle").join("state.json"))
    }

    /// Load state from disk
    pub fn load() -> Option<Self> {
        let path = Self::state_path()?;
        let contents = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&contents).ok()
    }

    /// Save state to disk
    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::state_path().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Could not determine home directory")
        })?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(path, contents)
    }

    /// Check if user is in managed mode
    pub fn is_managed(&self) -> bool {
        matches!(self.mode, Mode::Managed)
    }
}
