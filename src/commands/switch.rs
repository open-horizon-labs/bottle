use super::common::{build_agents_md_snippet, check_prerequisites, fetch_or_load_manifest, MARKETPLACE};
use crate::error::{BottleError, Result};
use crate::fetch::fetch_tool_definition;
use crate::install::{self, mcp, plugin};
use crate::manifest::bottle::BottleManifest;
use crate::manifest::state::{BottleState, InstallMethod, Mode, ToolState};
use crate::ui;
use chrono::Utc;
use console::style;
use std::collections::{HashMap, HashSet};

/// Switch to a different bottle
pub fn run(bottle: &str, yes: bool) -> Result<()> {
    // 1. Load current state
    let state = BottleState::load().ok_or(BottleError::NoBottleInstalled)?;

    // 2. Check if already on this bottle
    if state.bottle == bottle && state.is_managed() {
        ui::print_warning(&format!(
            "Already on bottle '{}'. Use 'bottle update' to refresh.",
            bottle
        ));
        return Ok(());
    }

    // 3. Check if ejected
    if matches!(state.mode, Mode::Ejected) {
        return Err(BottleError::Other(
            "Cannot switch while ejected. Use 'bottle install' to reinstall a managed bottle."
                .to_string(),
        ));
    }

    // 4. Fetch new bottle manifest
    let spinner = ui::spinner("Fetching bottle manifest...");
    let new_manifest = fetch_or_load_manifest(bottle, None)?;
    spinner.finish_and_clear();

    // 5. Check prerequisites
    check_prerequisites(&new_manifest)?;

    // 6. Calculate and show switch plan
    let plan = calculate_switch_plan(&state, &new_manifest);
    show_switch_plan(&state.bottle, &new_manifest, &plan);

    // 7. Confirm (unless -y)
    if !yes && !ui::confirm("Proceed with switch?", true) {
        return Err(BottleError::Cancelled);
    }

    println!();

    // 8. Execute the switch
    let tool_states = execute_switch(&state, &plan)?;

    // 9. Handle plugins
    update_plugins(&new_manifest)?;

    // 10. Build snippet for new bottle (if any)
    let snippet = match build_agents_md_snippet(&new_manifest) {
        Ok(s) => s,
        Err(e) => {
            ui::print_warning(&format!("Failed to build AGENTS.md snippet: {}", e));
            None
        }
    };

    // 11. Save new state (preserve integrations and custom tools across bottle switches)
    let new_state = BottleState {
        bottle: new_manifest.name.clone(),
        bottle_version: new_manifest.version.clone(),
        installed_at: Utc::now(),
        tools: tool_states,
        mode: Mode::Managed,
        integrations: state.integrations.clone(),
        custom_tools: state.custom_tools.clone(),
    };
    new_state
        .save()
        .map_err(|e| BottleError::Other(format!("Failed to save state: {}", e)))?;

    // Save snippet alongside state if present
    if let Some(snippet_content) = &snippet {
        new_state.save_snippet(snippet_content)
            .map_err(|e| BottleError::Other(format!("Failed to save AGENTS.md snippet: {}", e)))?;
    }

    // 12. Show success
    show_success(&state.bottle, &new_manifest);

    Ok(())
}

/// What changes need to be made
#[derive(Debug)]
struct SwitchPlan {
    add: Vec<(String, String)>,             // (tool, version)
    remove: Vec<String>,                    // tool name
    upgrade: Vec<(String, String, String)>, // (tool, old_version, new_version)
    downgrade: Vec<(String, String, String)>, // (tool, old_version, new_version)
    unchanged: Vec<(String, String)>,       // (tool, version)
}

/// Calculate what changes are needed to switch bottles
fn calculate_switch_plan(state: &BottleState, new_manifest: &BottleManifest) -> SwitchPlan {
    let current_tools: HashSet<&str> = state.tools.keys().map(|s| s.as_str()).collect();
    let new_tools: HashSet<&str> = new_manifest.tools.keys().map(|s| s.as_str()).collect();

    let mut plan = SwitchPlan {
        add: Vec::new(),
        remove: Vec::new(),
        upgrade: Vec::new(),
        downgrade: Vec::new(),
        unchanged: Vec::new(),
    };

    // Tools to add (in new but not in current)
    for tool in new_tools.difference(&current_tools) {
        let version = new_manifest.tools.get(*tool).unwrap();
        plan.add.push((tool.to_string(), version.clone()));
    }

    // Tools to remove (in current but not in new)
    for tool in current_tools.difference(&new_tools) {
        plan.remove.push(tool.to_string());
    }

    // Tools in both - check versions
    for tool in current_tools.intersection(&new_tools) {
        let current_version = &state.tools.get(*tool).unwrap().version;
        let new_version = new_manifest.tools.get(*tool).unwrap();

        if current_version == new_version {
            plan.unchanged.push((tool.to_string(), current_version.clone()));
        } else {
            // Compare versions (simple string comparison for semver)
            match compare_versions(current_version, new_version) {
                std::cmp::Ordering::Less => {
                    plan.upgrade.push((
                        tool.to_string(),
                        current_version.clone(),
                        new_version.clone(),
                    ));
                }
                std::cmp::Ordering::Greater => {
                    plan.downgrade.push((
                        tool.to_string(),
                        current_version.clone(),
                        new_version.clone(),
                    ));
                }
                std::cmp::Ordering::Equal => {
                    plan.unchanged.push((tool.to_string(), current_version.clone()));
                }
            }
        }
    }

    // Sort all vectors for consistent output
    plan.add.sort_by(|a, b| a.0.cmp(&b.0));
    plan.remove.sort();
    plan.upgrade.sort_by(|a, b| a.0.cmp(&b.0));
    plan.downgrade.sort_by(|a, b| a.0.cmp(&b.0));
    plan.unchanged.sort_by(|a, b| a.0.cmp(&b.0));

    plan
}

/// Simple semver comparison (numeric parts only)
/// Note: Pre-release suffixes (e.g., "-beta", "-rc1") are ignored.
/// Versions like "1.2.3-beta" and "1.2.3" compare as equal.
fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |v: &str| -> Vec<u32> {
        v.split('.')
            .filter_map(|part| {
                // Strip pre-release suffix (e.g., "3-beta" -> "3")
                let numeric = part.split('-').next().unwrap_or(part);
                numeric.parse::<u32>().ok()
            })
            .collect()
    };

    let va = parse(a);
    let vb = parse(b);

    for (pa, pb) in va.iter().zip(vb.iter()) {
        match pa.cmp(pb) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }

    va.len().cmp(&vb.len())
}

/// Display the switch plan
fn show_switch_plan(from_bottle: &str, to_manifest: &BottleManifest, plan: &SwitchPlan) {
    println!();
    println!(
        "{} {} ({}) {} {} ({})",
        style("Switching from").bold(),
        style(from_bottle).cyan(),
        style("current").dim(),
        style("→").bold(),
        style(&to_manifest.name).cyan(),
        &to_manifest.version
    );
    println!("{}", style(&to_manifest.description).dim());
    println!();

    // Show changes
    let has_changes = !plan.add.is_empty()
        || !plan.remove.is_empty()
        || !plan.upgrade.is_empty()
        || !plan.downgrade.is_empty();

    if !has_changes {
        println!(
            "{}",
            style("No tool changes needed (bottles have identical tools).").dim()
        );
        println!();
        return;
    }

    println!("{}:", style("Changes").bold());

    // New tools
    for (tool, version) in &plan.add {
        println!(
            "  {} {:<12} {}",
            style("+").green().bold(),
            tool,
            style(version).dim()
        );
    }

    // Removed tools
    for tool in &plan.remove {
        println!("  {} {:<12}", style("-").red().bold(), tool);
    }

    // Upgrades
    for (tool, old, new) in &plan.upgrade {
        println!(
            "  {} {:<12} {} → {}",
            style("↑").blue().bold(),
            tool,
            style(old).dim(),
            style(new).green()
        );
    }

    // Downgrades
    for (tool, old, new) in &plan.downgrade {
        println!(
            "  {} {:<12} {} → {}",
            style("↓").yellow().bold(),
            tool,
            style(old).dim(),
            style(new).yellow()
        );
    }

    // Unchanged (show count only)
    if !plan.unchanged.is_empty() {
        println!(
            "  {} {} tool(s) unchanged",
            style("=").dim(),
            plan.unchanged.len()
        );
    }

    println!();
}

/// Execute the switch plan
/// AIDEV-NOTE: Intentionally continues on failure and returns Ok with partial results.
/// State tracks what succeeded. User sees warnings for failures and can retry.
/// This is a design decision to avoid leaving users in a broken state when one
/// tool fails but others succeed.
fn execute_switch(state: &BottleState, plan: &SwitchPlan) -> Result<HashMap<String, ToolState>> {
    let mut tool_states: HashMap<String, ToolState> = HashMap::new();
    let mut failures: Vec<(String, BottleError)> = Vec::new();

    // Copy unchanged tools to new state
    for (tool, version) in &plan.unchanged {
        if let Some(existing) = state.tools.get(tool) {
            tool_states.insert(tool.clone(), existing.clone());
        } else {
            tool_states.insert(
                tool.clone(),
                ToolState {
                    version: version.clone(),
                    installed_at: Utc::now(),
                    method: InstallMethod::Cargo, // Default
                },
            );
        }
    }

    // Install new tools
    if !plan.add.is_empty() {
        println!("{}:", style("Installing new tools").bold());
        for (tool, version) in &plan.add {
            print!("  {:<12} {} ", tool, style(version).dim());

            let tool_def = match fetch_tool_definition(tool) {
                Ok(def) => def,
                Err(e) => {
                    println!("{}", style("failed").red());
                    failures.push((tool.clone(), e));
                    continue;
                }
            };

            match install::install_tool(&tool_def, version) {
                Ok(method) => {
                    println!("{}", style("installed").green());
                    tool_states.insert(
                        tool.clone(),
                        ToolState {
                            version: version.clone(),
                            installed_at: Utc::now(),
                            method,
                        },
                    );
                }
                Err(e) => {
                    println!("{}", style("failed").red());
                    failures.push((tool.clone(), e));
                }
            }
        }
        println!();
    }

    // Upgrade/downgrade tools (reinstall with new version)
    let version_changes: Vec<_> = plan.upgrade.iter().chain(plan.downgrade.iter()).collect();

    if !version_changes.is_empty() {
        println!("{}:", style("Updating tools").bold());
        for (tool, _old, new) in &version_changes {
            print!("  {:<12} {} ", tool, style(new).dim());

            let tool_def = match fetch_tool_definition(tool) {
                Ok(def) => def,
                Err(e) => {
                    println!("{}", style("failed").red());
                    failures.push((tool.clone(), e));
                    // Keep old state on failure
                    if let Some(existing) = state.tools.get(tool) {
                        tool_states.insert(tool.clone(), existing.clone());
                    }
                    continue;
                }
            };

            match install::install_tool(&tool_def, new) {
                Ok(method) => {
                    println!("{}", style("updated").green());
                    tool_states.insert(
                        tool.clone(),
                        ToolState {
                            version: new.clone(),
                            installed_at: Utc::now(),
                            method,
                        },
                    );
                }
                Err(e) => {
                    println!("{}", style("failed").red());
                    failures.push((tool.clone(), e));
                    // Keep old state on failure
                    if let Some(existing) = state.tools.get(tool) {
                        tool_states.insert(tool.clone(), existing.clone());
                    }
                }
            }
        }
        println!();
    }

    // Handle tools no longer needed
    if !plan.remove.is_empty() {
        // Separate MCP servers (can be unregistered) from binaries (kept for safety)
        let (mcp_tools, binary_tools): (Vec<_>, Vec<_>) = plan
            .remove
            .iter()
            .partition(|tool| {
                state
                    .tools
                    .get(*tool)
                    .map(|s| matches!(s.method, InstallMethod::Mcp))
                    .unwrap_or(false)
            });

        // Unregister MCP servers
        if !mcp_tools.is_empty() {
            println!("{}:", style("Unregistering MCP servers").bold());
            for tool in &mcp_tools {
                print!("  {:<12} ", tool);
                match mcp::unregister(tool) {
                    Ok(()) => {
                        println!("{}", style("removed").green());
                    }
                    Err(e) => {
                        println!("{}", style("failed").red());
                        failures.push((tool.to_string(), e));
                    }
                }
            }
            println!();
        }

        // Note binary tools that are no longer needed but kept for safety
        if !binary_tools.is_empty() {
            println!(
                "{}:",
                style("Binary tools no longer in bottle (kept installed)").dim()
            );
            for tool in &binary_tools {
                println!("  {:<12} {}", tool, style("use cargo uninstall to remove").dim());
            }
            println!();
        }
    }

    // Report failures
    if !failures.is_empty() {
        ui::print_warning(&format!("{} operation(s) failed:", failures.len()));
        for (name, err) in &failures {
            println!("  {} - {}", style(name).red(), err);
        }
        println!();
    }

    Ok(tool_states)
}

/// Update plugins between bottles
fn update_plugins(new_manifest: &BottleManifest) -> Result<()> {
    // For now, bottles don't track installed plugins in state
    // So we just install new plugins (idempotent)
    if new_manifest.plugins.is_empty() {
        return Ok(());
    }

    println!("{}:", style("Updating plugins").bold());

    let mut failures: Vec<(String, BottleError)> = Vec::new();

    for plugin_name in &new_manifest.plugins {
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

/// Display success message
fn show_success(from_bottle: &str, to_manifest: &BottleManifest) {
    println!();
    ui::print_success(&format!(
        "Switched from '{}' to '{}'!",
        from_bottle, to_manifest.name
    ));
    println!();
    println!("{}:", style("Next steps").bold());
    println!(
        "  {} - Verify installed tools",
        style("bottle status").cyan()
    );
    println!();
}
