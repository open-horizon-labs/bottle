use super::common::get_local_manifest_path;
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

    let manifest_path = get_local_manifest_path(bottle)?;
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

    // 5. MCP servers validation (bespoke)
    check_mcp_servers(&manifest, &mut errors, &mut warnings);

    // 6. AGENTS.md config validation (bespoke)
    check_agents_md(&manifest, &mut errors);

    // 7. Custom tools validation (bespoke)
    check_custom_tools(&manifest, &mut errors, &mut warnings);

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
            println!(
                "{} {} bottle is valid (with warnings).",
                style("✓").green().bold(),
                bottle
            );
            println!();
            Ok(())
        }
    }
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
            errors.push(format!(
                "Tool '{}' has no definition at tools/{}.json",
                tool_name, tool_name
            ));
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

/// Check MCP server definitions (bespoke bottles)
fn check_mcp_servers(manifest: &Value, errors: &mut Vec<String>, warnings: &mut Vec<String>) {
    let Some(mcp_servers) = manifest.get("mcp_servers") else {
        return;
    };

    if !mcp_servers.is_object() {
        errors.push("'mcp_servers' must be an object".to_string());
        return;
    }

    let servers = mcp_servers.as_object().unwrap();
    for (name, server) in servers {
        // command is required
        if server.get("command").and_then(|c| c.as_str()).is_none() {
            errors.push(format!(
                "MCP server '{}' missing required 'command' field",
                name
            ));
        }

        // scope must be "user" or "project" if present
        if let Some(scope) = server.get("scope").and_then(|s| s.as_str()) {
            if scope != "user" && scope != "project" {
                errors.push(format!(
                    "MCP server '{}' has invalid scope '{}' (must be 'user' or 'project')",
                    name, scope
                ));
            }
        }

        // args must be an array if present
        if let Some(args) = server.get("args") {
            if !args.is_array() {
                errors.push(format!("MCP server '{}' 'args' must be an array", name));
            }
        }

        // env must be an object if present
        if let Some(env) = server.get("env") {
            if !env.is_object() {
                errors.push(format!("MCP server '{}' 'env' must be an object", name));
            } else {
                // Warn about env vars that need to be set
                for (key, value) in env.as_object().unwrap() {
                    if let Some(v) = value.as_str() {
                        if v.contains("${") {
                            warnings.push(format!(
                                "MCP server '{}' env var '{}' requires runtime env var",
                                name, key
                            ));
                        }
                    }
                }
            }
        }
    }
}

/// Check AGENTS.md configuration (bespoke bottles)
fn check_agents_md(manifest: &Value, errors: &mut Vec<String>) {
    let Some(agents_md) = manifest.get("agents_md") else {
        return;
    };

    if !agents_md.is_object() {
        errors.push("'agents_md' must be an object".to_string());
        return;
    }

    // sections must be an array if present
    if let Some(sections) = agents_md.get("sections") {
        if !sections.is_array() {
            errors.push("'agents_md.sections' must be an array".to_string());
        } else {
            for (i, section) in sections.as_array().unwrap().iter().enumerate() {
                if section.get("heading").and_then(|h| h.as_str()).is_none() {
                    errors.push(format!("agents_md.sections[{}] missing 'heading'", i));
                }
                if section.get("content").and_then(|c| c.as_str()).is_none() {
                    errors.push(format!("agents_md.sections[{}] missing 'content'", i));
                }
            }
        }
    }

    // snippets_url must be a string if present
    if let Some(url) = agents_md.get("snippets_url") {
        if !url.is_string() {
            errors.push("'agents_md.snippets_url' must be a string".to_string());
        }
    }
}

/// Check custom tool definitions (bespoke bottles)
fn check_custom_tools(manifest: &Value, errors: &mut Vec<String>, warnings: &mut Vec<String>) {
    let Some(custom_tools) = manifest.get("custom_tools") else {
        return;
    };

    if !custom_tools.is_object() {
        errors.push("'custom_tools' must be an object".to_string());
        return;
    }

    let tools = custom_tools.as_object().unwrap();
    for (name, tool) in tools {
        // install is required
        let Some(install) = tool.get("install") else {
            errors.push(format!(
                "Custom tool '{}' missing required 'install' field",
                name
            ));
            continue;
        };

        if !install.is_object() {
            errors.push(format!(
                "Custom tool '{}' 'install' must be an object",
                name
            ));
            continue;
        }

        // At least one install method should be present
        let has_brew = install.get("brew").is_some();
        let has_cargo = install.get("cargo").is_some();
        let has_npm = install.get("npm").is_some();
        let has_binary = install.get("binary_url").is_some();

        if !has_brew && !has_cargo && !has_npm && !has_binary {
            errors.push(format!(
                "Custom tool '{}' has no installation method (need brew, cargo, npm, or binary_url)",
                name
            ));
        }

        // version is required
        if tool.get("version").and_then(|v| v.as_str()).is_none() {
            errors.push(format!(
                "Custom tool '{}' missing required 'version' field",
                name
            ));
        } else if let Some(v) = tool.get("version").and_then(|v| v.as_str()) {
            if !looks_like_semver(v) && v != "latest" {
                warnings.push(format!(
                    "Custom tool '{}' version '{}' doesn't look like semver",
                    name, v
                ));
            }
        }

        // verify must be a string if present
        if let Some(verify) = tool.get("verify") {
            if !verify.is_string() {
                errors.push(format!("Custom tool '{}' 'verify' must be a string", name));
            }
        }
    }
}
