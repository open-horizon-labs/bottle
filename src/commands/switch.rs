use crate::error::Result;

/// Switch to a different bottle
pub fn run(bottle: &str, _yes: bool) -> Result<()> {
    println!("Switching to bottle: {}", bottle);
    println!("(not yet implemented)");
    Ok(())
}
