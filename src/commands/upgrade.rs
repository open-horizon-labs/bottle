use crate::error::Result;

/// Bump a tool version in a bottle manifest (curator command)
pub fn run(bottle: &str, tool: &str, version: &str) -> Result<()> {
    println!("Upgrading {} in {}: -> {}", tool, bottle, version);
    println!("(not yet implemented)");
    Ok(())
}
