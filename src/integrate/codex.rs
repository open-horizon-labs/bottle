//! Codex integration
//!
//! Installs/removes all Cloud Atlas AI skills for Codex (bottle, ba, wm, superego).
//! Skill content is embedded at compile time from codex-skill/ directory.

use crate::error::{BottleError, Result};
use std::fs;
use std::path::{Path, PathBuf};

// Embed skill files at compile time - codex-skill/ is the source of truth
const BOTTLE_SKILL: &str = include_str!("../../codex-skill/SKILL.md");
const BA_SKILL: &str = include_str!("../../codex-skill/ba/SKILL.md");
const WM_SKILL: &str = include_str!("../../codex-skill/wm/SKILL.md");
const SG_SKILL: &str = include_str!("../../codex-skill/sg/SKILL.md");
const AGENTS_SNIPPET: &str = include_str!("../../codex-skill/AGENTS.md.snippet");

/// Get the Codex skills directory path
fn skills_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".codex").join("skills"))
}

/// Check if Codex is detected (has config directory)
pub fn is_detected() -> bool {
    dirs::home_dir()
        .map(|h| h.join(".codex").exists())
        .unwrap_or(false)
}

/// Check if the bottle skill is installed in Codex
#[allow(dead_code)]
pub fn is_installed() -> bool {
    skills_dir()
        .map(|s| s.join("bottle").exists())
        .unwrap_or(false)
}

/// Install all Cloud Atlas AI skills for Codex
pub fn install() -> Result<()> {
    let skills_path = skills_dir().ok_or_else(|| BottleError::InstallError {
        tool: "codex integration".to_string(),
        reason: "Could not determine Codex skills directory".to_string(),
    })?;

    // Create skills directory if needed
    fs::create_dir_all(&skills_path).map_err(|e| BottleError::InstallError {
        tool: "codex integration".to_string(),
        reason: format!("Failed to create skills directory: {}", e),
    })?;

    // Install all skills
    install_skill(&skills_path, "bottle", BOTTLE_SKILL)?;
    install_skill(&skills_path, "ba", BA_SKILL)?;
    install_skill(&skills_path, "wm", WM_SKILL)?;
    install_skill(&skills_path, "sg", SG_SKILL)?;

    // Install AGENTS.md.snippet in bottle skill directory
    let bottle_path = skills_path.join("bottle");
    fs::write(bottle_path.join("AGENTS.md.snippet"), AGENTS_SNIPPET).map_err(|e| {
        BottleError::InstallError {
            tool: "codex integration".to_string(),
            reason: format!("Failed to write AGENTS.md.snippet: {}", e),
        }
    })?;

    Ok(())
}

/// Install a single skill
fn install_skill(skills_path: &Path, name: &str, content: &str) -> Result<()> {
    let skill_path = skills_path.join(name);

    // Create skill directory
    fs::create_dir_all(&skill_path).map_err(|e| BottleError::InstallError {
        tool: "codex integration".to_string(),
        reason: format!("Failed to create {} skill directory: {}", name, e),
    })?;

    // Write SKILL.md
    fs::write(skill_path.join("SKILL.md"), content).map_err(|e| BottleError::InstallError {
        tool: "codex integration".to_string(),
        reason: format!("Failed to write {}/SKILL.md: {}", name, e),
    })?;

    Ok(())
}

/// Remove all Cloud Atlas AI skills from Codex
pub fn remove() -> Result<()> {
    let Some(skills_path) = skills_dir() else {
        return Ok(()); // No path, nothing to remove
    };

    // Remove all skills
    for name in &["bottle", "ba", "wm", "sg"] {
        let skill_path = skills_path.join(name);
        if skill_path.exists() {
            fs::remove_dir_all(&skill_path).map_err(|e| BottleError::InstallError {
                tool: "codex integration".to_string(),
                reason: format!("Failed to remove {} skill: {}", name, e),
            })?;
        }
    }

    Ok(())
}
