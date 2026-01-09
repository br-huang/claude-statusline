use thiserror::Error;

pub enum StatuslineError {
    #[error("Failed to parse JSON input: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Git command failed: {0}")]
    GitCommand(String),

    #[error("Invalid configuration: {0}")]
    Config(String),
}