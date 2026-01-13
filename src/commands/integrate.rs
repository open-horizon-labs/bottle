use crate::error::{BottleError, Result};
use crate::integrate::{self, Platform};
use crate::manifest::state::{BottleState, IntegrationState};
use crate::ui;
use chrono::Utc;
use console::style;

/// Add, remove, or list platform integrations
pub fn run(platform: Option<Platform>, list: bool, remove: bool) -> Result<()> {
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
        remove_integration(&state, platform)
    } else {
        add_integration(&state, platform)
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
fn add_integration(state: &BottleState, platform: Platform) -> Result<()> {
    // Check if already installed
    if state.integrations.contains_key(platform.key()) {
        ui::print_warning(&format!("{} integration is already installed.", platform));
        return Ok(());
    }

    // Warn if not detected (but allow anyway)
    let detections = integrate::detect_platforms();
    let detection = detections.iter().find(|d| d.platform == platform);
    if let Some(d) = detection {
        if !d.detected {
            ui::print_warning(&format!(
                "{} not detected ({} not found). Installing anyway.",
                platform, d.detection_hint
            ));
        }
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
fn remove_integration(state: &BottleState, platform: Platform) -> Result<()> {
    // Check if installed
    if !state.integrations.contains_key(platform.key()) {
        ui::print_warning(&format!("{} integration is not installed.", platform));
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
