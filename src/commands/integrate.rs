use crate::commands::common::fetch_or_load_manifest;
use crate::error::{BottleError, Result};
use crate::integrate::{self, Platform};
use crate::manifest::state::{BottleState, IntegrationState};
use crate::ui;
use chrono::Utc;
use console::style;

/// Add, remove, or list platform integrations
pub fn run(
    platform: Option<Platform>,
    manifest_path: Option<&std::path::Path>,
    list: bool,
    remove: bool,
    dry_run: bool,
) -> Result<()> {
    // For --manifest mode, we don't require a bottle to be installed
    let state = if manifest_path.is_some() {
        BottleState::load()
    } else {
        Some(BottleState::load().ok_or(BottleError::NoBottleInstalled)?)
    };

    // Handle --list
    if list {
        let state = state.ok_or(BottleError::NoBottleInstalled)?;
        return show_integrations(&state);
    }

    // Platform is required for add/remove
    let platform = platform.ok_or_else(|| {
        BottleError::Other(
            "Platform required. Use 'bottle integrate claude_code', 'opencode', or 'codex'.\n\
             Use 'bottle integrate --list' to see available integrations."
                .to_string(),
        )
    })?;

    if remove {
        let state = state.ok_or(BottleError::NoBottleInstalled)?;
        remove_integration(&state, platform, dry_run)
    } else {
        add_integration(state.as_ref(), manifest_path, platform, dry_run)
    }
}

/// Show available and installed integrations
fn show_integrations(state: &BottleState) -> Result<()> {
    println!();
    println!("{}:", style("Platform Integrations").bold());
    println!();

    let detections = integrate::detect_platforms();

    for detection in &detections {
        let platform = detection.platform;
        let detected = detection.detected;

        // For Claude Code with multiple directories, show each one
        if platform == Platform::ClaudeCode && !detection.directories.is_empty() {
            for dir in &detection.directories {
                let status = if dir.installed {
                    style("installed").green()
                } else {
                    style("available").yellow()
                };
                println!(
                    "  {:<12} {} {}",
                    platform.display_name(),
                    status,
                    style(format!("({})", dir.display_path)).dim()
                );
            }
        } else {
            // Original single-line output for other platforms or when no directories detected
            let installed = state.integrations.contains_key(platform.key());

            let status = if installed {
                style("installed").green()
            } else if detected {
                style("available").yellow()
            } else {
                style("not found").dim()
            };

            let hint = if detected {
                format!("({})", detection.detection_hint)
            } else {
                format!("({})", detection.detection_hint)
            };

            println!(
                "  {:<12} {} {}",
                platform.display_name(),
                status,
                style(hint).dim()
            );
        }
    }

    println!();
    println!("{}:", style("Commands").dim());
    println!(
        "  {} {}",
        style("bottle integrate <platform>").cyan(),
        style("Add integration").dim()
    );
    println!(
        "  {} {}",
        style("bottle integrate --remove <platform>").cyan(),
        style("Remove integration").dim()
    );
    println!(
        "  {} {}",
        style("CLAUDE_CONFIG_DIR=~/.claude-work bottle integrate claude_code").cyan(),
        style("Target other Claude directory").dim()
    );
    println!();

    Ok(())
}

/// Add a platform integration
fn add_integration(
    state: Option<&BottleState>,
    manifest_path: Option<&std::path::Path>,
    platform: Platform,
    dry_run: bool,
) -> Result<()> {
    // For Claude Code: clean up old @bottle marketplace entries first
    // This fixes the "Plugin not found in marketplace 'bottle'" error
    if platform == Platform::ClaudeCode {
        crate::integrate::claude_code::cleanup_old_marketplace_entries();
    }

    // Check if actually installed (not just in state) - handles partial installs
    let actually_installed = integrate::is_installed(platform);
    let in_state = state
        .map(|s| s.integrations.contains_key(platform.key()))
        .unwrap_or(false);

    if actually_installed && in_state {
        ui::print_warning(&format!("{} integration is already installed.", platform));
        return Ok(());
    }

    // If in state but not actually installed, we'll reinstall missing components
    if in_state && !actually_installed {
        ui::print_info(&format!(
            "{} integration incomplete, installing missing components...",
            platform
        ));
    }

    // Get detection info
    let detections = integrate::detect_platforms();
    let detection = detections.iter().find(|d| d.platform == platform);
    let detected = detection.map(|d| d.detected).unwrap_or(false);
    let hint = detection
        .map(|d| d.detection_hint.as_str())
        .unwrap_or("unknown");

    // Dry run: show what would happen
    if dry_run {
        println!();
        println!("{}", style("[DRY RUN]").yellow().bold());
        println!(
            "Would install {} integration:",
            style(platform.display_name()).cyan()
        );
        println!();
        if detected {
            println!("  Platform:  {} ({})", style("detected").green(), hint);
        } else {
            println!(
                "  Platform:  {} ({} not found)",
                style("not detected").yellow(),
                hint
            );
        }
        println!("  Action:    {}", describe_install_action(platform));
        if state.is_some() {
            println!(
                "  State:     Add {} to ~/.bottle/state.json",
                platform.key()
            );
        }
        println!();
        println!("{}", style("No changes made.").dim());
        println!();
        return Ok(());
    }

    // Warn if not detected (but allow anyway)
    if !detected {
        ui::print_warning(&format!(
            "{} not detected ({} not found). Installing anyway.",
            platform, hint
        ));
    }

    // Install
    println!(
        "Installing {} integration...",
        style(platform.display_name()).cyan()
    );

    // Fetch manifest to get opencode_plugins versions (if available)
    let opencode_plugins = if platform == Platform::OpenCode {
        let bottle_name = state.map(|s| s.bottle.as_str()).unwrap_or("local");
        fetch_or_load_manifest(bottle_name, manifest_path)
            .ok()
            .map(|m| m.opencode_plugins)
            .filter(|p| !p.is_empty())
    } else {
        None
    };

    integrate::install(platform, opencode_plugins.as_ref())?;

    // Update state (skip if using --manifest without existing state)
    if let Some(state) = state {
        let mut new_state = state.clone();
        new_state.integrations.insert(
            platform.key().to_string(),
            IntegrationState {
                installed_at: Utc::now(),
            },
        );
        new_state
            .save()
            .map_err(|e| BottleError::Other(format!("Failed to save state: {}", e)))?;
    }

    // Success message with platform-specific hints
    println!();
    ui::print_success(&format!("{} integration installed.", platform));

    match platform {
        Platform::ClaudeCode => {
            println!(
                "  The {} plugin is now available.",
                style("/bottle:").cyan()
            );
        }
        Platform::OpenCode => {
            println!("  Restart OpenCode to load the bottle ecosystem plugins.");
        }
        Platform::Codex => {
            println!("  Use {} commands in Codex.", style("$bottle").cyan());
        }
    }
    println!();

    Ok(())
}

/// Remove a platform integration
fn remove_integration(state: &BottleState, platform: Platform, dry_run: bool) -> Result<()> {
    // Check if installed
    if !state.integrations.contains_key(platform.key()) {
        ui::print_warning(&format!("{} integration is not installed.", platform));
        return Ok(());
    }

    // Dry run: show what would happen
    if dry_run {
        println!();
        println!("{}", style("[DRY RUN]").yellow().bold());
        println!(
            "Would remove {} integration:",
            style(platform.display_name()).cyan()
        );
        println!();
        println!("  Installed: {}", style("yes").green());
        println!("  Action:    {}", describe_remove_action(platform));
        println!(
            "  State:     Remove {} from ~/.bottle/state.json",
            platform.key()
        );
        println!();
        println!("{}", style("No changes made.").dim());
        println!();
        return Ok(());
    }

    // Remove
    println!(
        "Removing {} integration...",
        style(platform.display_name()).cyan()
    );

    integrate::remove(platform)?;

    // Update state
    let mut new_state = state.clone();
    new_state.integrations.remove(platform.key());
    new_state
        .save()
        .map_err(|e| BottleError::Other(format!("Failed to save state: {}", e)))?;

    // Success
    println!();
    ui::print_success(&format!("{} integration removed.", platform));
    println!();

    Ok(())
}

/// Describe the install action for a platform (for dry-run output)
fn describe_install_action(platform: Platform) -> &'static str {
    match platform {
        Platform::ClaudeCode => "Install all plugins: bottle, ba, superego, wm, oh-mcp, miranda",
        Platform::OpenCode => {
            "Add bottle ecosystem plugins to opencode.json (bottle, ba, wm, superego)"
        }
        Platform::Codex => "Create ~/.codex/skills/bottle/SKILL.md",
    }
}

/// Describe the remove action for a platform (for dry-run output)
fn describe_remove_action(platform: Platform) -> &'static str {
    match platform {
        Platform::ClaudeCode => "Remove all plugins: bottle, ba, superego, wm, oh-mcp, miranda",
        Platform::OpenCode => "Remove bottle ecosystem plugins from opencode.json",
        Platform::Codex => "Remove ~/.codex/skills/bottle/",
    }
}
