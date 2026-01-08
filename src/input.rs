// Analyze input from user

use serde::{Deserialize, Serialize};
use std::{io::{self, Read}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusInput {
    pub model: ModelInfo,
    pub cost: CostInfo,
    pub context_window: ContextWindow,
    pub workspace: WorkspaceInfo,
    pub session_id: String,
    pub transcript_path: String,

    #[serde(default)]
    pub hook_event_name: Option<String>,

    #[serde(default)]
    pub version: Option<String>,
}

// Model Info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub display_name: String,
    pub id: String,
}

// Cost Info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostInfo {
    #[serde(default)]
    pub total_cost_usd: f64,

    #[serde(default)]
    pub total_duration_ms: u64,

    #[serde(default)]
    pub total_api_duration_ms: u64,
}

// Context Window (Token Usage)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWindow {
    #[serde(default)]
    pub total_input_tokens: u64,

    #[serde(default)]
    pub total_output_tokens: u64,

    #[serde(default)]
    pub context_window_size: u64,

    #[serde(default)]
    pub current_usage: Option<CurrentUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUsage {
    #[serde(default)]
    pub input_token: u64,

    #[serde(default)]
    pub output_token: u64,

    #[serde(default)]
    pub cache_creation_input_tokens: u64,

    #[serde(default)]
    pub cache_read_input_tokens: u64,
}

// Workspace Info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub current_dir: String,
    pub project_dir: Option<String>,
}

impl StatusInput {
    pub fn from_stdin() -> Result<Self, Box<dyn std::error::Error>> {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        let input: Self = serde_json::from_str(&buffer)?;
        Ok(input)
    }

    pub fn from_string(string: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input: Self = serde_json::from_str(string)?;
        Ok(input)
    }
    
}