use crate::error::Result;

/// Install a bottle (stable, edge, minimal)
pub fn run(bottle: &str, _yes: bool) -> Result<()> {
    println!("Installing bottle: {}", bottle);
    println!("(not yet implemented)");
    Ok(())
}
