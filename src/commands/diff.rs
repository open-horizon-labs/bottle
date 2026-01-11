use crate::error::Result;

/// Compare bottles or check for updates (curator command)
pub fn run(from: &str, to: &str) -> Result<()> {
    println!("Comparing {} -> {}", from, to);
    println!("(not yet implemented)");
    Ok(())
}
