use crate::error::{BottleError, Result};
use crate::integrate::{self, Platform};
use crate::manifest::state::{BottleState, IntegrationState};
use crate::ui;
use chrono::Utc;
use console::style;

/// Add, remove, or list platform integrations
pub fn run(platform: Option<Platform>, list: bool, remove: bool, dry_run: bool) -> Result<()> {
    // Must have a bottle installed
    let state = BottleState::load().ok_or(BottleError::NoBottleInstalled)?;

    // Handle --list
    if list {
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
        remove_integration(&state, platform, dry_run)
    } else {
        add_integration(&state, platform, dry_run)
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
        let installed = state.integrations.contains_key(platform.key());
        let detected = detection.detected;

        let status = if installed {
            style("installed").green()
        } else if detected {
            style("detected").yellow()
        } else {
            style("not detected").dim()
        };

        let hint = if detected {
            format!("({})", detection.detection_hint)
        } else {
            format!("({} not found)", detection.detection_hint)
        };

        println!(
            "  {:<12} {} {}",
            platform.display_name(),
            status,
            style(hint).dim()
        );
    }

    println!();
    println!("{}:", style("Commands").dim());
    println!(
        "  {} {} - Add an integration",
        style("bottle integrate").cyan(),
        style("<platform>").dim()
    );
    println!(
        "  {} {} - Remove an integration",
        style("bottle integrate --remove").cyan(),
        style("<platform>").dim()
    );
    println!();

    Ok(())
}

/// Add a platform integration
fn add_integration(state: &BottleState, platform: Platform, dry_run: bool) -> Result<()> {
    // Check if already installed
    if state.integrations.contains_key(platform.key()) {
        ui::print_warning(&format!("{} integration is already installed.", platform));
        return Ok(());
    }

    // Get detection info
    let detections = integrate::detect_platforms();
    let detection = detections.iter().find(|d| d.platform == platform);
    let detected = detection.map(|d| d.detected).unwrap_or(false);
    let hint = detection.map(|d| d.detection_hint.as_str()).unwrap_or("unknown");

    // Dry run: show what would happen
    if dry_run {
        println!();
        println!("{}", style("[DRY RUN]").yellow().bold());
        println!("Would install {} integration:", style(platform.display_name()).cyan());
        println!();
        if detected {
            println!("  Platform:  {} ({})", style("detected").green(), hint);
        } else {
            println!("  Platform:  {} ({} not found)", style("not detected").yellow(), hint);
        }
        println!("  Action:    {}", describe_install_action(platform));
        println!("  State:     Add {} to ~/.bottle/state.json", platform.key());
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

    integrate::install(platform)?;

    // Update state
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
            println!("  Restart OpenCode to load the bottle plugin.");
        }
        Platform::Codex => {
            println!(
                "  Use {} commands in Codex.",
                style("$bottle").cyan()
            );
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
        println!("Would remove {} integration:", style(platform.display_name()).cyan());
        println!();
        println!("  Installed: {}", style("yes").green());
        println!("  Action:    {}", describe_remove_action(platform));
        println!("  State:     Remove {} from ~/.bottle/state.json", platform.key());
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
        Platform::ClaudeCode => "Run `claude plugin install bottle@cloud-atlas-ai/bottle`",
        Platform::OpenCode => "Add @cloud-atlas-ai/bottle to opencode.json plugins",
        Platform::Codex => "Create ~/.codex/skills/bottle/SKILL.md",
    }
}

/// Describe the remove action for a platform (for dry-run output)
fn describe_remove_action(platform: Platform) -> &'static str {
    match platform {
        Platform::ClaudeCode => "Run `claude plugin uninstall bottle@cloud-atlas-ai/bottle`",
        Platform::OpenCode => "Remove @cloud-atlas-ai/bottle from opencode.json plugins",
        Platform::Codex => "Remove ~/.codex/skills/bottle/",
    }
}
