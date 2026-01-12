use crate::error::{BottleError, Result};
use crate::fetch::fetch_bottle_manifest;
use crate::manifest::state::{BottleState, Mode};
use crate::ui;
use console::style;
use std::process::Command;

/// Show current bottle status and installed tools
pub fn run(check_updates: bool) -> Result<()> {
    let state = match BottleState::load() {
        Some(s) => s,
        None => {
            println!("{}", style("No bottle installed.").dim());
            println!();
            println!("Install a bottle with:");
            println!("  {} {}", style("bottle install").cyan(), style("stable").dim());
            println!();
            println!("Available bottles:");
            println!("  {}  Production-ready Cloud Atlas AI stack", style("stable").cyan());
            println!("  {}    Latest features, may be unstable", style("edge").cyan());
            return Ok(());
        }
    };

    // Show bottle header
    ui::print_bottle_header(&state.bottle, &state.bottle_version);

    // Show mode if ejected
    if matches!(state.mode, Mode::Ejected) {
        ui::print_warning("Mode: ejected (managing tools manually)");
        println!();
    }

    // Show tools
    println!("{}:", style("Tools").bold());

    // Sort tools for consistent output
    let mut tools: Vec<_> = state.tools.iter().collect();
    tools.sort_by_key(|(name, _)| *name);

    for (name, tool_state) in &tools {
        let installed = check_tool_installed(name);
        let status_icon = if installed {
            style("installed").green()
        } else {
            style("missing").red()
        };
        println!(
            "  {:<12} {:<8} {}",
            name,
            tool_state.version,
            status_icon
        );
    }
    println!();

    // Show plugins count
    // AIDEV-NOTE: Plugin count comes from the manifest, not the state.
    // For now, we show the count of tools as a proxy (each tool has a plugin).
    // Future: Track plugins separately in BottleState if they diverge from tools.
    println!("Plugins: {} configured", tools.len());
    println!();

    // Check for updates if requested
    if check_updates {
        check_for_updates(&state)?;
    }

    Ok(())
}

/// Check if a tool binary is actually installed and accessible
/// AIDEV-NOTE: Uses `which` command which is Unix-only. Windows is not currently
/// a supported platform for bottle. If Windows support is added, consider using
/// the `which` crate for cross-platform binary detection.
fn check_tool_installed(tool: &str) -> bool {
    // Map tool names to their binary names
    // Unknown tools fall through and use their name as the binary name
    let binary = match tool {
        "superego" => "sg",
        "wm" => "wm",
        "ba" => "ba",
        "datasphere" => "ds",
        // MCP servers don't have binaries, check via claude mcp list
        "oh-mcp" => return check_mcp_registered("oh-mcp"),
        _ => tool,
    };

    Command::new("which")
        .arg(binary)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Check if an MCP server is registered
fn check_mcp_registered(name: &str) -> bool {
    Command::new("claude")
        .args(["mcp", "list"])
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout).contains(name)
        })
        .unwrap_or(false)
}

/// Check for available updates by comparing with latest manifest
fn check_for_updates(state: &BottleState) -> Result<()> {
    let spinner = ui::spinner("Checking for updates...");

    let latest = match fetch_bottle_manifest(&state.bottle) {
        Ok(m) => m,
        Err(BottleError::BottleNotFound(_)) => {
            spinner.finish_and_clear();
            ui::print_warning(&format!(
                "Could not fetch latest manifest for '{}'",
                state.bottle
            ));
            return Ok(());
        }
        Err(e) => {
            spinner.finish_and_clear();
            return Err(e);
        }
    };

    spinner.finish_and_clear();

    // Compare versions
    if latest.version == state.bottle_version {
        println!("{}", style("Up to date.").green());
        return Ok(());
    }

    // Show update info
    println!(
        "{}: {} ({})",
        style("Update available").yellow().bold(),
        state.bottle,
        latest.version
    );
    println!("  Changes:");

    // Compare tool versions
    for (tool, new_version) in &latest.tools {
        if let Some(tool_state) = state.tools.get(tool) {
            if &tool_state.version != new_version {
                println!(
                    "    {:<12} {} {} {}",
                    tool,
                    tool_state.version,
                    style("->").dim(),
                    style(new_version).green()
                );
            }
        } else {
            // New tool added
            println!(
                "    {:<12} {} {}",
                tool,
                style("(new)").dim(),
                style(new_version).green()
            );
        }
    }

    // Check for removed tools
    for tool in state.tools.keys() {
        if !latest.tools.contains_key(tool) {
            println!(
                "    {:<12} {}",
                tool,
                style("(removed)").red()
            );
        }
    }

    println!();
    println!("Run {} to upgrade", style("bottle update").cyan());

    Ok(())
}
