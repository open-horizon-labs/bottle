use super::common::get_local_manifest_path;
use crate::error::{BottleError, Result};
use console::style;
use serde_json::Value;
use std::fs;

/// Bump a tool version in a bottle manifest (curator command)
pub fn run(bottle: &str, tool: &str, version: &str) -> Result<()> {
    // Find the manifest
    let manifest_path = get_local_manifest_path(bottle)?;

    // Read and parse
    let contents = fs::read_to_string(&manifest_path)?;
    let mut manifest: Value = serde_json::from_str(&contents)?;

    // Get tools object
    let tools = manifest
        .get_mut("tools")
        .and_then(|t| t.as_object_mut())
        .ok_or_else(|| BottleError::Other("Manifest has no tools section".to_string()))?;

    // Check tool exists and get old version
    let old_version = tools
        .get(tool)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| BottleError::ToolNotFound(tool.to_string()))?;

    // Update version
    tools.insert(tool.to_string(), Value::String(version.to_string()));

    // Write back with pretty formatting
    let updated = serde_json::to_string_pretty(&manifest)?;
    fs::write(&manifest_path, updated + "\n")?;

    // Report
    println!();
    println!(
        "{} Updated {} in {} bottle:",
        style("✓").green(),
        style(tool).cyan(),
        style(bottle).cyan()
    );
    println!(
        "  {} → {}",
        style(&old_version).dim(),
        style(version).green()
    );
    println!();
    println!("{}:", style("Next steps").bold());
    println!(
        "  {} - Verify manifest is valid",
        style("bottle validate").cyan()
    );
    println!("  {} - Tag and publish", style("bottle release").cyan());
    println!();

    Ok(())
}
