//! Codex integration
//!
//! Installs/removes all Open Horizon Labs skills for Codex (bottle, ba, wm, superego).
//! Skills are fetched from GitHub at runtime to allow updates without new binary releases.

use crate::error::{BottleError, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

const GITHUB_RAW_BASE: &str =
    "https://raw.githubusercontent.com/open-horizon-labs/bottle/master/codex-skill";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Fetch a skill file from GitHub
fn fetch_skill(path: &str) -> Result<String> {
    let url = format!("{}/{}", GITHUB_RAW_BASE, path);
    let client = reqwest::blocking::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|e| BottleError::InstallError {
            tool: "codex integration".to_string(),
            reason: format!("Failed to create HTTP client: {}", e),
        })?;
    client
        .get(&url)
        .send()
        .and_then(|r| r.error_for_status())
        .and_then(|r| r.text())
        .map_err(|e| BottleError::InstallError {
            tool: "codex integration".to_string(),
            reason: format!("Failed to fetch {}: {}", path, e),
        })
}

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

/// Install all Open Horizon Labs skills for Codex
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

    // Fetch and install all skills from GitHub
    let bottle_skill = fetch_skill("SKILL.md")?;
    let ba_skill = fetch_skill("ba/SKILL.md")?;
    let wm_skill = fetch_skill("wm/SKILL.md")?;
    let sg_skill = fetch_skill("sg/SKILL.md")?;
    let agents_snippet = fetch_skill("AGENTS.md.snippet")?;

    install_skill(&skills_path, "bottle", &bottle_skill)?;
    install_skill(&skills_path, "ba", &ba_skill)?;
    install_skill(&skills_path, "wm", &wm_skill)?;
    install_skill(&skills_path, "sg", &sg_skill)?;

    // Install AGENTS.md.snippet in bottle skill directory
    let bottle_path = skills_path.join("bottle");
    fs::write(bottle_path.join("AGENTS.md.snippet"), &agents_snippet).map_err(|e| {
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

/// Remove all Open Horizon Labs skills from Codex
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
