use crate::error::{BottleError, Result};
use console::style;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// Validate a bottle manifest (curator command)
pub fn run(bottle: &str) -> Result<()> {
    println!();
    println!("Validating {} bottle...", style(bottle).cyan());
    println!();

    let manifest_path = get_manifest_path(bottle)?;
    let contents = fs::read_to_string(&manifest_path)?;
    let manifest: Value = serde_json::from_str(&contents)?;

    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    // 1. Schema validation
    check_schema(&manifest, &mut errors);

    // 2. Tool definitions exist
    check_tool_definitions(&manifest, &mut errors);

    // 3. Version format
    check_version_format(&manifest, &mut warnings);

    // 4. No duplicate plugins (tools can't duplicate - JSON object keys are unique)
    check_duplicate_plugins(&manifest, &mut errors);

    // Report results
    if errors.is_empty() && warnings.is_empty() {
        println!("  {} Schema valid", style("✓").green());
        println!("  {} All tools have definitions", style("✓").green());
        println!("  {} Version formats valid", style("✓").green());
        println!("  {} No duplicates", style("✓").green());
        println!();
        println!("{} {} bottle is valid.", style("✓").green().bold(), bottle);
        println!();
        Ok(())
    } else {
        for err in &errors {
            println!("  {} {}", style("✗").red(), err);
        }
        for warn in &warnings {
            println!("  {} {}", style("!").yellow(), warn);
        }
        println!();
        if !errors.is_empty() {
            Err(BottleError::ValidationError(format!(
                "{} error(s) found in {} bottle",
                errors.len(),
                bottle
            )))
        } else {
            println!("{} {} bottle is valid (with warnings).", style("✓").green().bold(), bottle);
            println!();
            Ok(())
        }
    }
}

/// Get the path to a bottle manifest
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

/// Check required schema fields
fn check_schema(manifest: &Value, errors: &mut Vec<String>) {
    let required = ["name", "version", "description", "tools"];
    for field in required {
        if manifest.get(field).is_none() {
            errors.push(format!("Missing required field: {}", field));
        }
    }

    // tools must be an object
    if let Some(tools) = manifest.get("tools") {
        if !tools.is_object() {
            errors.push("'tools' must be an object".to_string());
        }
    }

    // plugins must be an array if present
    if let Some(plugins) = manifest.get("plugins") {
        if !plugins.is_array() {
            errors.push("'plugins' must be an array".to_string());
        }
    }
}

/// Check that each tool has a definition file
fn check_tool_definitions(manifest: &Value, errors: &mut Vec<String>) {
    let Some(tools) = manifest.get("tools").and_then(|t| t.as_object()) else {
        return;
    };

    for tool_name in tools.keys() {
        let def_path = PathBuf::from(format!("tools/{}.json", tool_name));
        if !def_path.exists() {
            errors.push(format!("Tool '{}' has no definition at tools/{}.json", tool_name, tool_name));
        }
    }
}

/// Check version formats look like semver
fn check_version_format(manifest: &Value, warnings: &mut Vec<String>) {
    let Some(tools) = manifest.get("tools").and_then(|t| t.as_object()) else {
        return;
    };

    for (tool_name, version) in tools {
        if let Some(v) = version.as_str() {
            if !looks_like_semver(v) {
                warnings.push(format!(
                    "Tool '{}' version '{}' doesn't look like semver (x.y.z)",
                    tool_name, v
                ));
            }
        }
    }
}

/// Simple semver check - must have at least x.y format
fn looks_like_semver(v: &str) -> bool {
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() < 2 {
        return false;
    }
    parts.iter().all(|p| p.parse::<u32>().is_ok())
}

/// Check for duplicate plugin entries
fn check_duplicate_plugins(manifest: &Value, errors: &mut Vec<String>) {
    if let Some(plugins) = manifest.get("plugins").and_then(|p| p.as_array()) {
        let mut seen = HashSet::new();
        for plugin in plugins {
            if let Some(name) = plugin.as_str() {
                if !seen.insert(name) {
                    errors.push(format!("Duplicate plugin: {}", name));
                }
            }
        }
    }
}
