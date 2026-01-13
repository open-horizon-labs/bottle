use super::common::{check_prerequisites, fetch_or_load_manifest, MARKETPLACE};
use crate::error::{BottleError, Result};
use crate::fetch::fetch_tool_definition;
use crate::install::{self, plugin};
use crate::manifest::bottle::BottleManifest;
use crate::manifest::state::{BottleState, Mode, ToolState};
use crate::ui;
use chrono::Utc;
use console::style;
use std::collections::HashMap;
use std::process::Command;

/// Install a bottle (stable, edge, or bespoke)
pub fn run(bottle: &str, yes: bool, dry_run: bool, force: bool) -> Result<()> {
    // 1. Check if already installed (skip if --force)
    if !force {
        if let Some(state) = BottleState::load() {
            if state.bottle == bottle && state.is_managed() {
                ui::print_warning(&format!(
                    "Bottle '{}' is already installed. Use 'bottle update' to refresh, or --force to reinstall.",
                    bottle
                ));
                return Ok(());
            }
            // Different bottle - this is a switch, not install
            return Err(BottleError::Other(format!(
                "Bottle '{}' is currently installed. Use 'bottle switch {}' to change bottles.",
                state.bottle, bottle
            )));
        }
    }

    // 2. Fetch manifest (local bespoke or remote curated)
    let spinner = ui::spinner("Fetching bottle manifest...");
    let manifest = fetch_or_load_manifest(bottle)?;
    spinner.finish_and_clear();

    // 3. Check prerequisites
    check_prerequisites(&manifest)?;

    // 4. Show what will be installed (or would be installed for dry-run)
    if dry_run {
        show_dry_run_plan(&manifest);
        return Ok(());
    }

    show_install_plan(&manifest);

    // 5. Confirm (unless -y)
    if !yes && !ui::confirm("Proceed with installation?", true) {
        return Err(BottleError::Cancelled);
    }

    println!();

    // 6. Install tools (binaries + MCP)
    let tool_states = install_tools(&manifest)?;

    // 7. Write state
    let state = BottleState {
        bottle: manifest.name.clone(),
        bottle_version: manifest.version.clone(),
        installed_at: Utc::now(),
        tools: tool_states,
        mode: Mode::Managed,
        integrations: HashMap::new(),
    };
    state.save().map_err(|e| BottleError::Other(format!("Failed to save state: {}", e)))?;

    // 9. Show success
    show_success(&manifest);

    Ok(())
}

/// Display the installation plan
fn show_install_plan(manifest: &BottleManifest) {
    println!();
    println!(
        "{} {} ({})",
        style("Installing bottle:").bold(),
        style(&manifest.name).cyan(),
        &manifest.version
    );
    println!("{}", style(&manifest.description).dim());
    println!();

    // Show tools
    println!("{}:", style("Tools").bold());
    let mut tools: Vec<_> = manifest.tools.iter().collect();
    tools.sort_by_key(|(name, _)| *name);
    for (name, version) in &tools {
        println!("  {:<12} {}", name, style(version).dim());
    }
    println!();
}

/// Display the dry-run plan showing what would be installed
fn show_dry_run_plan(manifest: &BottleManifest) {
    println!();
    println!("{}", style("[DRY RUN]").yellow().bold());
    println!(
        "Would install bottle {} ({}):",
        style(&manifest.name).cyan(),
        &manifest.version
    );
    println!("{}", style(&manifest.description).dim());
    println!();

    // Show tools with installation status
    println!("{}:", style("Tools").bold());
    let mut tools: Vec<_> = manifest.tools.iter().collect();
    tools.sort_by_key(|(name, _)| *name);
    for (name, target_version) in &tools {
        let current = get_tool_version(name);
        match current {
            Some(installed_ver) if installed_ver == **target_version => {
                println!("  {:<12} {} {}", name, style("current").green(), style(target_version).dim());
            }
            Some(installed_ver) => {
                let arrow = match compare_versions(&installed_ver, target_version) {
                    "upgrade" => style("↑").green(),
                    "downgrade" => style("↓").red(),
                    _ => style("→").yellow(),
                };
                println!("  {:<12} {} {} {}", name, arrow, style(&installed_ver).dim(), target_version);
            }
            None => {
                println!("  {:<12} {} {}", name, style("install").yellow(), target_version);
            }
        }
    }
    println!();

    // Show plugins available via integrate
    if !manifest.plugins.is_empty() {
        println!("{} {}:", style("Plugins").bold(), style("(via bottle integrate)").dim());
        for plugin in &manifest.plugins {
            println!("  {}", plugin);
        }
        println!();
    }

    // Show detected platforms for integration
    println!("{} {}:", style("Platform Integrations").bold(), style("(optional, run after install)").dim());
    println!();
    show_claude_code_integration();
    show_opencode_integration();
    show_codex_integration();
    println!();

    // Show state changes
    println!("{}:", style("State changes").bold());
    println!("  Create ~/.bottle/state.json with:");
    println!("    bottle: {}", manifest.name);
    println!("    version: {}", manifest.version);
    println!("    mode: managed");
    println!();

    println!("{}", style("No changes made.").dim());
    println!();
}

fn show_claude_code_integration() {
    let detected = crate::integrate::claude_code::is_detected();
    if detected {
        println!("  {} {}", style("Claude Code").cyan().bold(), style("(~/.claude/ detected)").dim());
        println!("    {} bottle integrate claude_code", style("→").dim());
        println!("    Adds /bottle commands: status, update, switch, integrate, list");
        println!("    Plugin: bottle@open-horizon-labs");
    } else {
        println!("  {} {}", style("Claude Code").dim(), style("not detected").dim());
    }
    println!();
}

fn show_opencode_integration() {
    let detected = crate::integrate::opencode::is_detected();
    if detected {
        println!("  {} {}", style("OpenCode").cyan().bold(), style("(~/.opencode/ detected)").dim());
        println!("    {} bottle integrate opencode", style("→").dim());
        println!("    Adds bottle-* tools to OpenCode");
        println!("    Config: adds @cloud-atlas-ai/bottle to plugins");
    } else {
        println!("  {} {}", style("OpenCode").dim(), style("not detected").dim());
    }
    println!();
}

fn show_codex_integration() {
    let detected = crate::integrate::codex::is_detected();
    if detected {
        println!("  {} {}", style("Codex").cyan().bold(), style("(~/.codex/ detected)").dim());
        println!("    {} bottle integrate codex", style("→").dim());
        println!("    Adds $bottle commands as Codex skill");
        println!("    Skill: ~/.codex/skills/bottle/SKILL.md");
    } else {
        println!("  {} {}", style("Codex").dim(), style("not detected").dim());
    }
}

/// Compare two version strings, returns "upgrade", "downgrade", or "update"
fn compare_versions(current: &str, target: &str) -> &'static str {
    // Simple semver comparison - split by dots and compare numerically
    let parse = |v: &str| -> Vec<u32> {
        v.split('.')
            .filter_map(|s| s.parse().ok())
            .collect()
    };

    let current_parts = parse(current);
    let target_parts = parse(target);

    for (c, t) in current_parts.iter().zip(target_parts.iter()) {
        if t > c {
            return "upgrade";
        } else if t < c {
            return "downgrade";
        }
    }

    // If we get here, compare lengths (e.g., 1.0 vs 1.0.1)
    match target_parts.len().cmp(&current_parts.len()) {
        std::cmp::Ordering::Greater => "upgrade",
        std::cmp::Ordering::Less => "downgrade",
        std::cmp::Ordering::Equal => "update", // shouldn't happen if versions are equal
    }
}

/// Get installed version of a tool, or None if not installed
fn get_tool_version(tool: &str) -> Option<String> {
    let binary = match tool {
        "superego" => "sg",
        "datasphere" => "ds",
        "oh-mcp" => return get_mcp_version("oh-mcp"),
        _ => tool,
    };

    Command::new(binary)
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| {
            let stdout = String::from_utf8_lossy(&o.stdout);
            // Parse "tool x.y.z" or "x.y.z" format
            stdout
                .split_whitespace()
                .find(|s| s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
                .map(|s| s.trim().to_string())
        })
}

/// Get version of an MCP server (if registered)
fn get_mcp_version(name: &str) -> Option<String> {
    // MCP servers don't have a standard version query
    // Just check if registered
    if is_mcp_registered(name) {
        Some("registered".to_string())
    } else {
        None
    }
}

/// Check if an MCP server is registered
fn is_mcp_registered(name: &str) -> bool {
    Command::new("claude")
        .args(["mcp", "list"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains(name))
        .unwrap_or(false)
}

/// Get installed version of a Claude Code plugin
/// Path structure: ~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/
fn get_plugin_version(plugin: &str) -> Option<String> {
    let cache = dirs::home_dir()?.join(".claude/plugins/cache");

    // Check if plugin is a marketplace name (e.g., superego/superego/0.9.0)
    let marketplace_path = cache.join(plugin);
    if marketplace_path.exists() {
        if let Some(ver) = find_plugin_version(&marketplace_path, plugin) {
            return Some(ver);
        }
    }

    // Check inside all marketplaces for the plugin
    for entry in std::fs::read_dir(&cache).ok()?.filter_map(|e| e.ok()) {
        let plugin_path = entry.path().join(plugin);
        if plugin_path.exists() {
            if let Some(ver) = find_plugin_version(&entry.path(), plugin) {
                return Some(ver);
            }
        }
    }

    None
}

/// Find version directory for a plugin within a marketplace
fn find_plugin_version(marketplace_path: &std::path::Path, plugin: &str) -> Option<String> {
    let plugin_path = marketplace_path.join(plugin);
    std::fs::read_dir(&plugin_path)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .next()
}

/// Install all tools from the manifest
/// AIDEV-NOTE: Intentionally continues on failure and returns Ok with partial results.
/// State tracks what succeeded. User sees warnings for failures and can retry.
/// This is a design decision to avoid leaving users in a broken state when one
/// tool fails but others succeed. Consider adding --strict flag if needed later.
fn install_tools(manifest: &BottleManifest) -> Result<HashMap<String, ToolState>> {
    let mut states = HashMap::new();
    let mut failures: Vec<(String, BottleError)> = Vec::new();

    println!("{}:", style("Installing tools").bold());

    let mut tools: Vec<_> = manifest.tools.iter().collect();
    tools.sort_by_key(|(name, _)| *name);

    for (tool_name, version) in tools {
        print!("  {:<12} {} ", tool_name, style(version).dim());

        // Fetch tool definition
        let tool_def = match fetch_tool_definition(tool_name) {
            Ok(def) => def,
            Err(e) => {
                println!("{}", style("failed").red());
                failures.push((tool_name.clone(), e));
                continue;
            }
        };

        // Install the tool
        match install::install_tool(&tool_def, version) {
            Ok(method) => {
                println!("{}", style("installed").green());
                states.insert(
                    tool_name.clone(),
                    ToolState {
                        version: version.clone(),
                        installed_at: Utc::now(),
                        method,
                    },
                );
            }
            Err(e) => {
                println!("{}", style("failed").red());
                failures.push((tool_name.clone(), e));
            }
        }
    }

    println!();

    if !failures.is_empty() {
        ui::print_warning(&format!(
            "{} tool(s) failed to install:",
            failures.len()
        ));
        for (name, err) in &failures {
            println!("  {} - {}", style(name).red(), err);
        }
        println!();
    }

    Ok(states)
}

/// Install all plugins from the manifest
fn install_plugins(manifest: &BottleManifest) -> Result<()> {
    if manifest.plugins.is_empty() {
        return Ok(());
    }

    println!("{}:", style("Installing plugins").bold());

    let mut failures: Vec<(String, BottleError)> = Vec::new();

    for plugin_name in &manifest.plugins {
        print!("  {:<12} ", plugin_name);

        match plugin::install(plugin_name, MARKETPLACE) {
            Ok(()) => {
                println!("{}", style("installed").green());
            }
            Err(e) => {
                println!("{}", style("failed").red());
                failures.push((plugin_name.clone(), e));
            }
        }
    }

    println!();

    if !failures.is_empty() {
        ui::print_warning(&format!(
            "{} plugin(s) failed to install:",
            failures.len()
        ));
        for (name, err) in &failures {
            println!("  {} - {}", style(name).red(), err);
        }
        println!();
    }

    Ok(())
}

/// Display success message with next steps
fn show_success(manifest: &BottleManifest) {
    println!();
    ui::print_success(&format!(
        "Bottle '{}' installed successfully!",
        manifest.name
    ));
    println!();
    println!("{}:", style("Next steps").bold());
    println!("  {} - Check installed tools", style("bottle status").cyan());
    println!("  {} - Initialize ba for task tracking", style("ba init").cyan());
    println!("  {} - Initialize working memory", style("wm init").cyan());
    println!();
}
