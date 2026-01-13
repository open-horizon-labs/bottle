use crate::error::{BottleError, Result};
use chrono::Local;
use console::style;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Tag and publish a bottle update (curator command)
pub fn run(bottle: &str, message: Option<&str>) -> Result<()> {
    println!();
    println!("Releasing {} bottle...", style(bottle).cyan());
    println!();

    // 1. Run validation
    print!("  Validating manifest... ");
    validate_manifest(bottle)?;
    println!("{}", style("✓").green());

    // 2. Check git status is clean
    print!("  Checking git status... ");
    check_git_clean()?;
    println!("{}", style("✓").green());

    // 3. Bump manifest version to today's date
    let manifest_path = get_manifest_path(bottle)?;
    let (old_version, new_version) = bump_version(&manifest_path)?;
    println!(
        "  Bumped version: {} → {}",
        style(&old_version).dim(),
        style(&new_version).green()
    );

    // 4. Commit the version bump
    let commit_msg = format_commit_message(bottle, &new_version, message);
    git_commit(&manifest_path, &commit_msg)?;
    println!("  Committed: {}", style(&commit_msg).dim());

    // 5. Create git tag
    let tag_name = format!("{}-{}", bottle, new_version);
    git_tag(&tag_name, message)?;
    println!("  Tagged: {}", style(&tag_name).cyan());

    // 6. Push commit + tag
    git_push(&tag_name)?;
    println!("  Pushed to origin {}", style("✓").green());

    println!();
    println!(
        "{} Released {} {}",
        style("✓").green().bold(),
        style(bottle).cyan(),
        style(&new_version).green()
    );
    println!();

    Ok(())
}

/// Validate the manifest (mirrors validate command logic)
fn validate_manifest(bottle: &str) -> Result<()> {
    let manifest_path = get_manifest_path(bottle)?;
    let contents = fs::read_to_string(&manifest_path)?;
    let manifest: Value = serde_json::from_str(&contents)?;

    // Check required fields
    let required = ["name", "version", "description", "tools"];
    for field in required {
        if manifest.get(field).is_none() {
            return Err(BottleError::ValidationError(format!(
                "Missing required field: {}",
                field
            )));
        }
    }

    // Check tools is an object
    if let Some(tools) = manifest.get("tools") {
        if !tools.is_object() {
            return Err(BottleError::ValidationError(
                "'tools' must be an object".to_string(),
            ));
        }
    }

    // Check tool definitions exist
    if let Some(tools) = manifest.get("tools").and_then(|t| t.as_object()) {
        for tool_name in tools.keys() {
            let def_path = PathBuf::from(format!("tools/{}.json", tool_name));
            if !def_path.exists() {
                return Err(BottleError::ValidationError(format!(
                    "Tool '{}' has no definition at tools/{}.json",
                    tool_name, tool_name
                )));
            }
        }
    }

    Ok(())
}

/// Check that git working directory is clean
fn check_git_clean() -> Result<()> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()?;

    let status = String::from_utf8_lossy(&output.stdout);
    if !status.trim().is_empty() {
        let files: Vec<&str> = status.lines().collect();
        let mut msg = String::from("Working directory is not clean.\nUncommitted changes:\n");
        for file in files.iter().take(10) {
            msg.push_str(&format!("  {}\n", file));
        }
        if files.len() > 10 {
            msg.push_str(&format!("  ... and {} more\n", files.len() - 10));
        }
        msg.push_str("\nCommit your changes before releasing.");
        return Err(BottleError::Other(msg));
    }

    Ok(())
}

/// Bump the version field to today's date and return (old, new) versions
fn bump_version(manifest_path: &PathBuf) -> Result<(String, String)> {
    let contents = fs::read_to_string(manifest_path)?;
    let mut manifest: Value = serde_json::from_str(&contents)?;

    let old_version = manifest
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let new_version = Local::now().format("%Y.%m.%d").to_string();

    manifest["version"] = Value::String(new_version.clone());

    let updated = serde_json::to_string_pretty(&manifest)?;
    fs::write(manifest_path, updated + "\n")?;

    Ok((old_version, new_version))
}

/// Format the commit message
fn format_commit_message(bottle: &str, version: &str, message: Option<&str>) -> String {
    match message {
        Some(msg) => format!("chore: release {} {} - {}", bottle, version, msg),
        None => format!("chore: release {} {}", bottle, version),
    }
}

/// Commit the manifest change
fn git_commit(manifest_path: &PathBuf, message: &str) -> Result<()> {
    let path_str = manifest_path
        .to_str()
        .ok_or_else(|| BottleError::Other("Invalid path encoding".to_string()))?;

    // Stage the manifest
    let status = Command::new("git")
        .args(["add", path_str])
        .status()?;

    if !status.success() {
        return Err(BottleError::Other("Failed to stage manifest".to_string()));
    }

    // Commit
    let status = Command::new("git")
        .args(["commit", "-m", message])
        .status()?;

    if !status.success() {
        return Err(BottleError::Other("Failed to commit".to_string()));
    }

    Ok(())
}

/// Create a git tag
fn git_tag(tag_name: &str, message: Option<&str>) -> Result<()> {
    // Check if tag already exists
    let output = Command::new("git")
        .args(["tag", "-l", tag_name])
        .output()?;

    let existing = String::from_utf8_lossy(&output.stdout);
    if !existing.trim().is_empty() {
        return Err(BottleError::Other(format!(
            "Tag '{}' already exists. A release was already made today.\n\
            Either wait until tomorrow or delete the existing tag with:\n  \
            git tag -d {} && git push origin :refs/tags/{}",
            tag_name, tag_name, tag_name
        )));
    }

    // Create annotated tag
    let tag_msg = message.unwrap_or("Release");
    let status = Command::new("git")
        .args(["tag", "-a", tag_name, "-m", tag_msg])
        .status()?;

    if !status.success() {
        return Err(BottleError::Other(format!(
            "Failed to create tag '{}'",
            tag_name
        )));
    }

    Ok(())
}

/// Push commit and tag to origin
fn git_push(tag_name: &str) -> Result<()> {
    // Push the commit
    let output = Command::new("git")
        .args(["push", "origin"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(BottleError::Other(format!(
            "Failed to push commit.\n{}\n\n\
            The commit and tag were created locally. To retry:\n  \
            git push origin && git push origin {}",
            stderr.trim(),
            tag_name
        )));
    }

    // Push the tag
    let output = Command::new("git")
        .args(["push", "origin", tag_name])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(BottleError::Other(format!(
            "Failed to push tag '{}'.\n{}\n\n\
            The commit was pushed but the tag wasn't. To retry:\n  \
            git push origin {}",
            tag_name,
            stderr.trim(),
            tag_name
        )));
    }

    Ok(())
}

/// Get the path to a bottle manifest (local bottles/ directory)
fn get_manifest_path(bottle: &str) -> Result<PathBuf> {
    let local_path = PathBuf::from(format!("bottles/{}/manifest.json", bottle));
    if local_path.exists() {
        return Ok(local_path);
    }

    Err(BottleError::BottleNotFound(format!(
        "No local manifest found at bottles/{}/manifest.json. Run from bottle repo root.",
        bottle
    )))
}
