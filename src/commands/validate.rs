use crate::error::Result;

/// Validate a bottle manifest (curator command)
pub fn run(bottle: &str) -> Result<()> {
    println!("Validating bottle: {}", bottle);
    println!("(not yet implemented)");
    Ok(())
}
