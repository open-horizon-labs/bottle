use crate::error::{BottleError, Result};
use crate::fetch::{fetch_bottle_manifest, fetch_tool_definition};
use crate::install::{self, plugin};
use crate::manifest::bottle::BottleManifest;
use crate::manifest::state::{BottleState, Mode, ToolState};
use crate::ui;
use chrono::Utc;
use console::style;
use std::collections::HashMap;
use std::fs;

const MARKETPLACE: &str = "cloud-atlas-ai/bottle";

/// Install a bottle (stable, edge, or bespoke)
pub fn run(bottle: &str, yes: bool) -> Result<()> {
    // 1. Check if already installed
    if let Some(state) = BottleState::load() {
        if state.bottle == bottle && state.is_managed() {
            ui::print_warning(&format!(
                "Bottle '{}' is already installed. Use 'bottle update' to refresh.",
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

    // 2. Fetch manifest (local bespoke or remote curated)
    let spinner = ui::spinner("Fetching bottle manifest...");
    let manifest = fetch_or_load_manifest(bottle)?;
    spinner.finish_and_clear();

    // 3. Check prerequisites
    check_prerequisites(&manifest)?;

    // 4. Show what will be installed
    show_install_plan(&manifest);

    // 5. Confirm (unless -y)
    if !yes && !ui::confirm("Proceed with installation?", true) {
        return Err(BottleError::Cancelled);
    }

    println!();

    // 6. Install tools (binaries + MCP)
    let tool_states = install_tools(&manifest)?;

    // 7. Install plugins
    install_plugins(&manifest)?;

    // 8. Write state
    let state = BottleState {
        bottle: manifest.name.clone(),
        bottle_version: manifest.version.clone(),
        installed_at: Utc::now(),
        tools: tool_states,
        mode: Mode::Managed,
    };
    state.save().map_err(|e| BottleError::Other(format!("Failed to save state: {}", e)))?;

    // 9. Show success
    show_success(&manifest);

    Ok(())
}

/// Fetch manifest from bespoke location or GitHub
fn fetch_or_load_manifest(bottle: &str) -> Result<BottleManifest> {
    // Check bespoke first (~/.bottle/bottles/<name>/)
    if let Some(home) = dirs::home_dir() {
        let bespoke_path = home
            .join(".bottle")
            .join("bottles")
            .join(bottle)
            .join("manifest.json");

        if bespoke_path.exists() {
            let contents = fs::read_to_string(&bespoke_path).map_err(|e| {
                BottleError::Other(format!("Failed to read bespoke manifest: {}", e))
            })?;
            return serde_json::from_str(&contents).map_err(|e| {
                BottleError::ParseError(e)
            });
        }
    }

    // Fall back to curated (fetch from GitHub)
    fetch_bottle_manifest(bottle)
}

/// Check that required prerequisites are available
fn check_prerequisites(manifest: &BottleManifest) -> Result<()> {
    let mut missing = Vec::new();

    if manifest.prerequisites.contains_key("cargo") && !crate::install::cargo::is_available() {
        missing.push("cargo (install Rust: https://rustup.rs)");
    }

    if manifest.prerequisites.contains_key("node") && which::which("node").is_err() {
        missing.push("node (install Node.js: https://nodejs.org)");
    }

    if !missing.is_empty() {
        return Err(BottleError::PrerequisitesNotMet(missing.join(", ")));
    }

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

    // Show plugins
    println!("{}:", style("Plugins").bold());
    for plugin in &manifest.plugins {
        println!("  {}", plugin);
    }
    println!();
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
