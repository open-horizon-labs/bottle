//! Codex integration
//!
//! Installs/removes the bottle skill for Codex.

use crate::error::{BottleError, Result};
use std::fs;
use std::path::PathBuf;

/// Skill directory name
const SKILL_DIR: &str = "bottle";

/// Get the Codex skills directory path
fn skills_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".codex").join("skills"))
}

/// Get the bottle skill path
fn skill_path() -> Option<PathBuf> {
    skills_dir().map(|s| s.join(SKILL_DIR))
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
    skill_path()
        .map(|p| p.exists())
        .unwrap_or(false)
}

/// Install the bottle skill for Codex
/// AIDEV-NOTE: Creates a minimal SKILL.md that invokes the bottle CLI.
/// The actual skill content will be enhanced as part of kk-c0jl task.
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

    let bottle_skill_path = skills_path.join(SKILL_DIR);

    // Create skill directory
    fs::create_dir_all(&bottle_skill_path).map_err(|e| BottleError::InstallError {
        tool: "codex integration".to_string(),
        reason: format!("Failed to create bottle skill directory: {}", e),
    })?;

    // Write SKILL.md
    let skill_content = r#"# Bottle

Manage the Cloud Atlas AI tool stack.

## Commands

All commands invoke the `bottle` CLI. Ensure bottle is installed.

### $bottle status

Show current bottle state.

```bash
bottle status
```

### $bottle install <name>

Install a bottle (stable, edge, or bespoke).

```bash
bottle install stable
bottle install edge
```

### $bottle update

Update to the latest bottle snapshot.

```bash
bottle update
```

### $bottle switch <name>

Switch to a different bottle.

```bash
bottle switch edge
```

### $bottle list

List available bottles.

```bash
bottle list
```

### $bottle eject

Exit bottle management, keep tools.

```bash
bottle eject
```

### $bottle integrate <platform>

Add a platform integration.

```bash
bottle integrate opencode
bottle integrate codex
bottle integrate claude_code
```

### $bottle integrate --remove <platform>

Remove a platform integration.

```bash
bottle integrate --remove codex
```

### $bottle integrate --list

Show available and installed integrations.

```bash
bottle integrate --list
```
"#;

    fs::write(bottle_skill_path.join("SKILL.md"), skill_content).map_err(|e| {
        BottleError::InstallError {
            tool: "codex integration".to_string(),
            reason: format!("Failed to write SKILL.md: {}", e),
        }
    })?;

    Ok(())
}

/// Remove the bottle skill from Codex
pub fn remove() -> Result<()> {
    let Some(path) = skill_path() else {
        return Ok(()); // No path, nothing to remove
    };

    if !path.exists() {
        return Ok(()); // Already removed
    }

    fs::remove_dir_all(&path).map_err(|e| BottleError::InstallError {
        tool: "codex integration".to_string(),
        reason: format!("Failed to remove bottle skill: {}", e),
    })?;

    Ok(())
}
