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
    /// Platform integrations (Claude Code, OpenCode, Codex)
    /// AIDEV-NOTE: Optional for backwards compatibility with existing state files
    #[serde(default)]
    pub integrations: HashMap<String, IntegrationState>,
    /// Custom tools installed via bespoke bottles
    /// AIDEV-NOTE: Optional for backwards compatibility with existing state files
    #[serde(default)]
    pub custom_tools: HashMap<String, CustomToolState>,
}

/// State for a platform integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationState {
    pub installed_at: DateTime<Utc>,
}

/// State for a custom tool installed via bespoke bottle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomToolState {
    pub version: String,
    pub installed_at: DateTime<Utc>,
    pub method: CustomInstallMethod,
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

/// Install method for custom tools (more options than curated tools)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomInstallMethod {
    Brew,
    Cargo,
    Npm,
    Binary,
}

impl BottleState {
    /// Get the base bottle directory
    pub fn bottle_dir() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".bottle"))
    }

    /// Get the path to the active bottle pointer file
    pub fn active_path() -> Option<PathBuf> {
        Self::bottle_dir().map(|d| d.join("active"))
    }

    /// Get the directory for a specific bottle
    pub fn bottle_path(bottle: &str) -> Option<PathBuf> {
        Self::bottle_dir().map(|d| d.join("bottles").join(bottle))
    }

    /// Get the path to a bottle's state file
    pub fn state_path(bottle: &str) -> Option<PathBuf> {
        Self::bottle_path(bottle).map(|d| d.join("state.json"))
    }

    /// Get the path to a bottle's AGENTS.md snippet
    pub fn snippet_path(bottle: &str) -> Option<PathBuf> {
        Self::bottle_path(bottle).map(|d| d.join("agents-md-snippet"))
    }

    /// Get the active bottle name
    pub fn active_bottle() -> Option<String> {
        let path = Self::active_path()?;
        std::fs::read_to_string(path).ok().map(|s| s.trim().to_string())
    }

    /// Set the active bottle
    pub fn set_active(bottle: &str) -> std::io::Result<()> {
        let path = Self::active_path().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Could not determine home directory")
        })?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, bottle)
    }

    /// Load state for the active bottle
    pub fn load() -> Option<Self> {
        let bottle = Self::active_bottle()?;
        Self::load_for(&bottle)
    }

    /// Load state for a specific bottle
    pub fn load_for(bottle: &str) -> Option<Self> {
        let path = Self::state_path(bottle)?;
        let contents = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&contents).ok()
    }

    /// Save state to disk (uses the bottle name from self)
    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::state_path(&self.bottle).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Could not determine home directory")
        })?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, &contents)?;

        // Also set as active bottle
        Self::set_active(&self.bottle)?;

        Ok(())
    }

    /// Save AGENTS.md snippet for this bottle
    pub fn save_snippet(&self, content: &str) -> std::io::Result<()> {
        let path = Self::snippet_path(&self.bottle).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Could not determine home directory")
        })?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, content)
    }

    /// Load AGENTS.md snippet for the active bottle
    pub fn load_snippet() -> Option<String> {
        let bottle = Self::active_bottle()?;
        Self::load_snippet_for(&bottle)
    }

    /// Load AGENTS.md snippet for a specific bottle
    pub fn load_snippet_for(bottle: &str) -> Option<String> {
        let path = Self::snippet_path(bottle)?;
        std::fs::read_to_string(path).ok()
    }

    /// Check if user is in managed mode
    pub fn is_managed(&self) -> bool {
        matches!(self.mode, Mode::Managed)
    }
}
