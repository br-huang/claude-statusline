use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme_path: Option<PathBuf>,
    pub theme_name: Option<String>,
    pub enabled_segments: Option<Vec<String>>,
    pub cache_ttl: u64,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_paths = vec![
            dirs::config_dir().map(|p| p.join("claude-statusline/config.toml")),
            Some(PathBuf::from(".claude-statusline.toml")),
        ];

        for path in config_paths.into_iter().flatten() {
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                let mut config: Self = toml::from_str(&content)?;
                config.apply_env_overrides();
                return Ok(config);
            }
        }

        let mut config = Self::default();
        config.apply_env_overrides();
        Ok(config)
    }

    fn apply_env_overrides(&mut self) {
        if let Ok(theme) = std::env::var("CLAUDE_STATUSLINE_THEME") {
            self.theme_name = Some(theme);
        }
         
         if let Ok(segments) = std::env::var("CLAUDE_STATUSLINE_SEGMENTS") {
            self.enabled_segments = Some(
                segments
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            );
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme_path: None,
            theme_name: Some("default".to_string()),
            enabled_segments: None,
            cache_ttl: 5,
        }
    }
}