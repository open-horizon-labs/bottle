use crate::error::{BottleError, Result};
use crate::manifest::bottle::McpServerDef;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::process::Command;

/// Pattern for matching ${VAR} environment variable references
static ENV_VAR_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\{([^}]+)\}").unwrap());

/// Validate environment variables in a bespoke MCP server definition.
/// Returns an error if any ${VAR} pattern references an unset env var.
pub fn validate_env_vars(name: &str, server: &McpServerDef) -> Result<()> {
    let pattern = &*ENV_VAR_PATTERN;
    let mut missing: Vec<String> = Vec::new();

    // Check env values for ${VAR} patterns
    for (key, value) in &server.env {
        for cap in pattern.captures_iter(value) {
            let var_name = &cap[1];
            if std::env::var(var_name).is_err() {
                missing.push(format!("{}={} (needs ${})", key, value, var_name));
            }
        }
    }

    // Also check args for ${VAR} patterns
    for arg in &server.args {
        for cap in pattern.captures_iter(arg) {
            let var_name = &cap[1];
            if std::env::var(var_name).is_err() {
                missing.push(format!("arg '{}' (needs ${})", arg, var_name));
            }
        }
    }

    if missing.is_empty() {
        Ok(())
    } else {
        Err(BottleError::ValidationError(format!(
            "MCP server '{}' requires environment variables that are not set:\n  {}\n\nSet these environment variables and try again.",
            name,
            missing.join("\n  ")
        )))
    }
}

/// Expand ${VAR} patterns in a string using environment variables.
/// Logs a warning if a variable is not set (substitutes empty string).
fn expand_env_vars(s: &str) -> String {
    ENV_VAR_PATTERN
        .replace_all(s, |caps: &regex::Captures| {
            let var_name = &caps[1];
            match std::env::var(var_name) {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Warning: environment variable ${} is not set", var_name);
                    String::new()
                }
            }
        })
        .to_string()
}

/// Register a bespoke MCP server with Claude Code.
/// Supports custom commands, args, env vars, and scope.
/// Note: Caller should validate env vars first using validate_env_vars().
pub fn register_bespoke(name: &str, server: &McpServerDef) -> Result<()> {
    // Build command args with -e KEY=VALUE flags before "--"
    // Format: claude mcp add <name> -s <scope> [-e KEY=VALUE]... -- <command> [args]...
    let mut args: Vec<String> = vec![
        "mcp".to_string(),
        "add".to_string(),
        name.to_string(),
        "-s".to_string(),
        server.scope.clone(),
    ];

    // Add -e KEY=VALUE flags for each environment variable (before "--")
    for (key, value) in &server.env {
        args.push("-e".to_string());
        args.push(format!("{}={}", key, expand_env_vars(value)));
    }

    // Add separator and command
    args.push("--".to_string());
    args.push(server.command.clone());

    // Expand env vars in server args and add them
    for arg in &server.args {
        args.push(expand_env_vars(arg));
    }

    // Build and run the command
    let status =
        Command::new("claude")
            .args(&args)
            .status()
            .map_err(|e| BottleError::InstallError {
                tool: name.to_string(),
                reason: format!("Failed to run claude mcp add: {}", e),
            })?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("claude mcp add exited with code {}", status),
        })
    }
}

/// Register bespoke MCP servers with OpenCode by writing to opencode.json
/// Note: Caller should validate env vars first using validate_env_vars().
pub fn register_bespoke_opencode(servers: &HashMap<String, McpServerDef>) -> Result<()> {
    use serde_json::{json, Value};
    use std::fs;
    use std::path::PathBuf;

    // Get config path (cwd first, then global)
    let config_path = {
        let cwd_config = PathBuf::from("opencode.json");
        if cwd_config.exists() {
            cwd_config
        } else {
            dirs::home_dir()
                .map(|h| h.join(".config").join("opencode").join("opencode.json"))
                .unwrap_or_else(|| PathBuf::from("opencode.json"))
        }
    };

    // Read existing config or create new one
    let mut config: Value = if config_path.exists() {
        let contents = fs::read_to_string(&config_path).map_err(|e| BottleError::InstallError {
            tool: "opencode mcp".to_string(),
            reason: format!("Failed to read opencode.json: {}", e),
        })?;
        serde_json::from_str(&contents).map_err(|e| BottleError::InstallError {
            tool: "opencode mcp".to_string(),
            reason: format!("Failed to parse {}: {}", config_path.display(), e),
        })?
    } else {
        json!({
            "$schema": "https://opencode.ai/config.json"
        })
    };

    // Get or create mcp object (OpenCode's key for MCP servers)
    let mcp_servers = config
        .as_object_mut()
        .ok_or_else(|| BottleError::InstallError {
            tool: "opencode mcp".to_string(),
            reason: "opencode.json is not an object".to_string(),
        })?
        .entry("mcp")
        .or_insert_with(|| json!({}));

    let mcp_obj = mcp_servers
        .as_object_mut()
        .ok_or_else(|| BottleError::InstallError {
            tool: "opencode mcp".to_string(),
            reason: "mcp is not an object".to_string(),
        })?;

    // Add each server using OpenCode's format
    for (name, server) in servers {
        // Build command array: [command, ...args]
        let mut command_arr: Vec<String> = vec![server.command.clone()];
        for arg in &server.args {
            command_arr.push(expand_env_vars(arg));
        }

        let mut expanded_env: HashMap<String, String> = HashMap::new();
        for (k, v) in &server.env {
            expanded_env.insert(k.clone(), expand_env_vars(v));
        }

        mcp_obj.insert(
            name.clone(),
            json!({
                "type": "local",
                "command": command_arr,
                "environment": expanded_env,
                "enabled": true
            }),
        );
    }

    // Write back
    let updated = serde_json::to_string_pretty(&config).map_err(|e| BottleError::InstallError {
        tool: "opencode mcp".to_string(),
        reason: format!("Failed to serialize config: {}", e),
    })?;

    // Create parent directory if needed
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| BottleError::InstallError {
            tool: "opencode mcp".to_string(),
            reason: format!(
                "Failed to create config directory {}: {}",
                parent.display(),
                e
            ),
        })?;
    }

    fs::write(&config_path, updated).map_err(|e| BottleError::InstallError {
        tool: "opencode mcp".to_string(),
        reason: format!("Failed to write opencode.json: {}", e),
    })?;

    Ok(())
}

/// Register an MCP server with Claude
/// `name` is the MCP server name (e.g., "oh-mcp")
/// `package` is the npm package (e.g., "@cloud-atlas-ai/oh-mcp-server")
pub fn register(name: &str, package: &str, version: &str) -> Result<()> {
    let status = Command::new("claude")
        .args([
            "mcp",
            "add",
            name,
            "-s",
            "user",
            "--",
            "npx",
            "-y",
            &format!("{}@{}", package, version),
        ])
        .status()
        .map_err(|e| BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("Failed to run claude mcp add: {}", e),
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("claude mcp add exited with code {}", status),
        })
    }
}

/// Unregister an MCP server
pub fn unregister(name: &str) -> Result<()> {
    let status = Command::new("claude")
        .args(["mcp", "remove", name])
        .status()
        .map_err(|e| BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("Failed to run claude mcp remove: {}", e),
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("claude mcp remove exited with code {}", status),
        })
    }
}
