pub mod claude_code;
pub mod codex;
pub mod opencode;

use crate::error::Result;
use std::collections::HashMap;
use std::fmt;

/// Supported platform integrations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    ClaudeCode,
    OpenCode,
    Codex,
}

impl Platform {
    /// Get the state key for this platform (snake_case)
    pub fn key(&self) -> &'static str {
        match self {
            Platform::ClaudeCode => "claude_code",
            Platform::OpenCode => "opencode",
            Platform::Codex => "codex",
        }
    }

    /// Get display name for this platform
    pub fn display_name(&self) -> &'static str {
        match self {
            Platform::ClaudeCode => "Claude Code",
            Platform::OpenCode => "OpenCode",
            Platform::Codex => "Codex",
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Detection result for a platform
#[derive(Debug)]
pub struct DetectionResult {
    pub platform: Platform,
    pub detected: bool,
    pub detection_hint: String,
}

/// Detect which platforms are available on this system
pub fn detect_platforms() -> Vec<DetectionResult> {
    vec![
        DetectionResult {
            platform: Platform::ClaudeCode,
            detected: claude_code::is_detected(),
            detection_hint: "~/.claude/".to_string(),
        },
        DetectionResult {
            platform: Platform::OpenCode,
            detected: opencode::is_detected(),
            detection_hint: "opencode.json".to_string(),
        },
        DetectionResult {
            platform: Platform::Codex,
            detected: codex::is_detected(),
            detection_hint: "~/.codex/".to_string(),
        },
    ]
}

/// Install an integration for a platform
/// For OpenCode, pass the opencode_plugins map from the manifest for versioned installs
pub fn install(platform: Platform, opencode_plugins: Option<&HashMap<String, String>>) -> Result<()> {
    match platform {
        Platform::ClaudeCode => claude_code::install(),
        Platform::OpenCode => opencode::install(opencode_plugins),
        Platform::Codex => codex::install(),
    }
}

/// Remove an integration for a platform
pub fn remove(platform: Platform) -> Result<()> {
    match platform {
        Platform::ClaudeCode => claude_code::remove(),
        Platform::OpenCode => opencode::remove(),
        Platform::Codex => codex::remove(),
    }
}

/// Check if an integration is currently installed (filesystem check)
/// AIDEV-NOTE: Kept for future use in `bottle status` to verify state matches reality
#[allow(dead_code)]
pub fn is_installed(platform: Platform) -> bool {
    match platform {
        Platform::ClaudeCode => claude_code::is_installed(),
        Platform::OpenCode => opencode::is_installed(),
        Platform::Codex => codex::is_installed(),
    }
}
