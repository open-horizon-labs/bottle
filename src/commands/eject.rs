use crate::error::{BottleError, Result};
use crate::manifest::state::{BottleState, Mode};
use crate::ui;
use console::style;

/// Eject from bottle management
///
/// Ejecting leaves all tools in place but switches to manual management mode.
/// After ejecting, the user is responsible for updating tools individually.
pub fn run(yes: bool) -> Result<()> {
    // Load current state
    let mut state = BottleState::load().ok_or(BottleError::NoBottleInstalled)?;

    // Check if already ejected
    if matches!(state.mode, Mode::Ejected) {
        return Err(BottleError::AlreadyEjected);
    }

    // Explain what ejecting means
    ui::print_header("Eject from Bottle Management");

    println!(
        "Ejecting switches from {} to {} management.",
        style("managed").cyan(),
        style("manual").yellow()
    );
    println!();
    println!("{}:", style("What this means").bold());
    println!(
        "  {} All installed tools remain in place",
        style("•").blue()
    );
    println!("  {} MCP servers stay registered", style("•").blue());
    println!("  {} Plugins remain installed", style("•").blue());
    println!(
        "  {} You manage tool updates yourself (cargo install, etc.)",
        style("•").blue()
    );
    println!(
        "  {} {} will still track what you have, but won't update",
        style("•").blue(),
        style("bottle status").cyan()
    );
    println!();

    // Show what user is keeping
    println!("{}:", style("Tools you're keeping").bold());
    let mut tools: Vec<_> = state.tools.iter().collect();
    tools.sort_by_key(|(name, _)| *name);

    for (name, tool_state) in &tools {
        println!("  {:<12} {}", name, tool_state.version);
    }
    println!();

    // Show how to return to managed mode
    println!(
        "{}: To return to managed mode, run {} to reinstall a bottle.",
        style("Note").dim(),
        style("bottle install <bottle>").cyan()
    );
    println!();

    // Confirm unless --yes was passed
    if !yes && !ui::confirm("Eject from bottle management?", false) {
        return Err(BottleError::Cancelled);
    }

    // Set mode to ejected and save
    state.mode = Mode::Ejected;
    state.save().map_err(BottleError::IoError)?;

    ui::print_success("Ejected from bottle management");
    println!();
    println!(
        "You're now managing tools manually. Run {} to see your tools.",
        style("bottle status").cyan()
    );

    Ok(())
}
