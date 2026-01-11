use crate::error::Result;

/// Tag and publish a bottle update (curator command)
pub fn run(bottle: &str, message: Option<&str>) -> Result<()> {
    println!("Releasing bottle: {}", bottle);
    if let Some(msg) = message {
        println!("Message: {}", msg);
    }
    println!("(not yet implemented)");
    Ok(())
}
