use crate::error::{BottleError, Result};
use crate::manifest::bottle::BottleManifest;
use crate::manifest::tool::ToolDefinition;

const GITHUB_RAW_BASE: &str =
    "https://raw.githubusercontent.com/cloud-atlas-ai/bottle/main";

/// Fetch a bottle manifest from GitHub
pub fn fetch_bottle_manifest(bottle: &str) -> Result<BottleManifest> {
    let url = format!("{}/bottles/{}/manifest.json", GITHUB_RAW_BASE, bottle);
    let response = reqwest::blocking::get(&url)?;

    if response.status() == 404 {
        return Err(BottleError::BottleNotFound(bottle.to_string()));
    }

    let manifest: BottleManifest = response.json()?;
    Ok(manifest)
}

/// Fetch a tool definition from GitHub
pub fn fetch_tool_definition(tool: &str) -> Result<ToolDefinition> {
    let url = format!("{}/tools/{}.json", GITHUB_RAW_BASE, tool);
    let response = reqwest::blocking::get(&url)?;

    if response.status() == 404 {
        return Err(BottleError::ToolNotFound(tool.to_string()));
    }

    let definition: ToolDefinition = response.json()?;
    Ok(definition)
}

/// List available bottles
pub fn list_available_bottles() -> Result<Vec<String>> {
    // For now, return hardcoded list
    // In the future, could fetch from an index file
    Ok(vec![
        "stable".to_string(),
        "edge".to_string(),
        "minimal".to_string(),
    ])
}
