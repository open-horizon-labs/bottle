use thiserror::Error;

/// Result type for bottle operations
pub type Result<T> = std::result::Result<T, BottleError>;

/// Errors that can occur during bottle operations
#[derive(Error, Debug)]
pub enum BottleError {
    #[error("Failed to fetch manifest: {0}")]
    FetchError(#[from] reqwest::Error),

    #[error("Failed to parse manifest: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Tool installation failed: {tool} - {reason}")]
    InstallError { tool: String, reason: String },

    #[error("No bottle installed. Run `bottle install` first.")]
    NoBottleInstalled,

    #[error("Bottle not found: {0}")]
    BottleNotFound(String),

    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("State file corrupted: {0}")]
    StateCorrupted(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("User cancelled operation")]
    Cancelled,

    #[error("Prerequisites not met: {0}")]
    PrerequisitesNotMet(String),

    #[error("Already ejected from bottle management")]
    AlreadyEjected,

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("{0}")]
    Other(String),
}
