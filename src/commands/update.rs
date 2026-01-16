use crate::error::{BottleError, Result};
use crate::fetch::{fetch_bottle_manifest, fetch_tool_definition};
use crate::install;
use crate::manifest::bottle::BottleManifest;
use crate::manifest::state::{BottleState, ToolState};
use crate::ui;
use chrono::Utc;
use console::style;
use std::collections::HashMap;

/// Update to the latest bottle snapshot
pub fn run(yes: bool) -> Result<()> {
    // 1. Check if a bottle is installed and managed
    let state = BottleState::load().ok_or(BottleError::NoBottleInstalled)?;

    if !state.is_managed() {
        return Err(BottleError::AlreadyEjected);
    }

    // 2. Fetch the latest manifest
    let spinner = ui::spinner("Checking for updates...");
    let latest = fetch_bottle_manifest(&state.bottle)?;
    spinner.finish_and_clear();

    // 3. Calculate what needs updating (check tool versions, not just manifest version)
    let changes = calculate_changes(&state, &latest);

    // 4. If no changes and same version, we're up to date
    if changes.is_empty() && latest.version == state.bottle_version {
        println!(
            "{} {} is already at the latest version ({})",
            style("Bottle").bold(),
            style(&state.bottle).cyan(),
            state.bottle_version
        );
        return Ok(());
    }

    if changes.is_empty() {
        // Version changed but no tool changes - just update state
        println!(
            "{}: {} {} {}",
            style("Bottle update available").bold(),
            state.bottle,
            style(format!("{} -> {}", state.bottle_version, latest.version)).dim(),
            style("(metadata only)").dim()
        );
    } else {
        // Show update plan
        show_update_plan(&state, &latest, &changes);
    }

    // 5. Confirm (unless -y)
    if !yes && !ui::confirm("Proceed with update?", true) {
        return Err(BottleError::Cancelled);
    }

    println!();

    // 6. Apply updates
    let updated_tools = if changes.is_empty() {
        state.tools.clone()
    } else {
        apply_updates(&state, &changes)?
    };

    // 7. Save updated state (preserve integrations across updates)
    let new_state = BottleState {
        bottle: state.bottle.clone(),
        bottle_version: latest.version.clone(),
        installed_at: state.installed_at,
        tools: updated_tools,
        mode: state.mode.clone(),
        integrations: state.integrations.clone(),
    };
    new_state
        .save()
        .map_err(|e| BottleError::Other(format!("Failed to save state: {}", e)))?;

    // 8. Show success
    println!();
    ui::print_success(&format!(
        "Updated to {} {}",
        state.bottle, latest.version
    ));

    Ok(())
}

/// Types of changes that can occur during an update
#[derive(Debug)]
enum ToolChange {
    /// Tool version changed
    Upgrade { from: String, to: String },
    /// New tool added
    Add { version: String },
    /// Tool removed from manifest
    Remove,
}

/// Calculate what tools need to change
fn calculate_changes(state: &BottleState, latest: &BottleManifest) -> HashMap<String, ToolChange> {
    let mut changes = HashMap::new();

    // Check for upgrades and additions
    for (tool, new_version) in &latest.tools {
        if let Some(tool_state) = state.tools.get(tool) {
            if &tool_state.version != new_version {
                changes.insert(
                    tool.clone(),
                    ToolChange::Upgrade {
                        from: tool_state.version.clone(),
                        to: new_version.clone(),
                    },
                );
            }
        } else {
            changes.insert(
                tool.clone(),
                ToolChange::Add {
                    version: new_version.clone(),
                },
            );
        }
    }

    // Check for removals
    for tool in state.tools.keys() {
        if !latest.tools.contains_key(tool) {
            changes.insert(tool.clone(), ToolChange::Remove);
        }
    }

    changes
}

/// Display the update plan
fn show_update_plan(
    state: &BottleState,
    latest: &BottleManifest,
    changes: &HashMap<String, ToolChange>,
) {
    println!();
    println!(
        "{}: {} {}",
        style("Update available").bold(),
        style(&state.bottle).cyan(),
        style(format!("{} -> {}", state.bottle_version, latest.version)).dim(),
    );
    println!();

    println!("{}:", style("Changes").bold());

    let mut sorted_changes: Vec<_> = changes.iter().collect();
    sorted_changes.sort_by_key(|(name, _)| *name);

    for (tool, change) in sorted_changes {
        match change {
            ToolChange::Upgrade { from, to } => {
                println!(
                    "  {:<12} {} {} {}",
                    tool,
                    from,
                    style("->").dim(),
                    style(to).green()
                );
            }
            ToolChange::Add { version } => {
                println!(
                    "  {:<12} {} {}",
                    tool,
                    style("(new)").dim(),
                    style(version).green()
                );
            }
            ToolChange::Remove => {
                println!("  {:<12} {}", tool, style("(will be removed)").red());
            }
        }
    }

    println!();
}

/// Apply the calculated updates
/// AIDEV-NOTE: Intentionally continues on failure and returns Ok with partial results.
/// State tracks what succeeded. User sees warnings for failures and can retry.
/// This matches the install.rs design - partial success is still success.
fn apply_updates(
    state: &BottleState,
    changes: &HashMap<String, ToolChange>,
) -> Result<HashMap<String, ToolState>> {
    let mut tools = state.tools.clone();
    let mut failures: Vec<(String, BottleError)> = Vec::new();

    println!("{}:", style("Updating tools").bold());

    let mut sorted_changes: Vec<_> = changes.iter().collect();
    sorted_changes.sort_by_key(|(name, _)| *name);

    for (tool_name, change) in sorted_changes {
        match change {
            ToolChange::Upgrade { to, .. } => {
                print!("  {:<12} {} ", tool_name, style(to).dim());

                // Fetch tool definition
                let tool_def = match fetch_tool_definition(tool_name) {
                    Ok(def) => def,
                    Err(e) => {
                        println!("{}", style("failed").red());
                        failures.push((tool_name.clone(), e));
                        continue;
                    }
                };

                // Upgrade the tool
                match install::install_tool(&tool_def, to) {
                    Ok(method) => {
                        println!("{}", style("updated").green());
                        tools.insert(
                            tool_name.clone(),
                            ToolState {
                                version: to.clone(),
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
            ToolChange::Add { version } => {
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

                // Install the new tool
                match install::install_tool(&tool_def, version) {
                    Ok(method) => {
                        println!("{}", style("installed").green());
                        tools.insert(
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
            ToolChange::Remove => {
                // Just remove from state - don't uninstall the binary
                // User may want to keep using it manually
                print!("  {:<12} ", tool_name);
                println!("{}", style("removed from tracking").yellow());
                tools.remove(tool_name);
            }
        }
    }

    println!();

    if !failures.is_empty() {
        ui::print_warning(&format!("{} tool(s) failed to update:", failures.len()));
        for (name, err) in &failures {
            println!("  {} - {}", style(name).red(), err);
        }
        println!();
    }

    // Verify unchanged tools are preserved. Since `tools` starts as
    // state.tools.clone() and we only modify tools in `changes`, unchanged
    // tools should always remain in the map.
    debug_assert!(
        state.tools.keys().all(|tool| changes.contains_key(tool) || tools.contains_key(tool)),
        "Unchanged tools were unexpectedly removed from state"
    );

    Ok(tools)
}
