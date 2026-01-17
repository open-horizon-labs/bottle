use crate::error::{BottleError, Result};
use crate::manifest::state::BottleState;

/// Output AGENTS.md snippet for the active bottle.
/// Used by AI agents to get the snippet content they should inject.
pub fn run() -> Result<()> {
    let state = BottleState::load().ok_or(BottleError::NoBottleInstalled)?;

    match BottleState::load_snippet() {
        Some(snippet) => {
            print!("{}", snippet);
            Ok(())
        }
        None => {
            // No snippet for this bottle - not an error, just empty output
            // This happens for curated bottles without agents_md config
            eprintln!(
                "No AGENTS.md snippet configured for bottle '{}'",
                state.bottle
            );
            Ok(())
        }
    }
}
