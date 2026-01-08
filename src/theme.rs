use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub theme_name: String,
    pub enabled_segments: Vec<String>,
    pub separtor: String,
    pub colors: ColorScheme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub model: SegmentColors,
    pub tokens: SegmentColors,
    pub cost: SegmentColors,
    pub duration: SegmentColors,
    pub git: GitColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentColors {
    pub icon: String,
    pub label: String,
    pub value: String,
    pub reset: String, // Reset to default
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitColors {
    pub branch: String,
    pub ahead: String,
    pub behind: String,
    pub modified: String,
    pub reset: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            theme_name: "default".to_string(),
            enabled_segments: vec![
                "model".to_string(),
                "tokens".to_string(),
                "cost".to_string(),
                "duration".to_string(),
                "git".to_string(),
            ],
            separtor: " │ ".to_string(),  // Unicode box-drawing character
            colors: ColorScheme::default(),

        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            model: SegmentColors { 
                icon: "\x1b[95m".to_string(), // 亮紫色
                 label: "\x1b[36m".to_string(), // 青色
                 value: "\x1b[96m".to_string(), // 亮青色
                 reset: "\x1b[0m".to_string() 
            },
            tokens: SegmentColors {
                  icon: "\x1b[93m".to_string(),   // 亮黃色
                  label: "\x1b[33m".to_string(),  // 黃色
                  value: "\x1b[93m".to_string(),  // 亮黃色
                  reset: "\x1b[0m".to_string(),
              },
              cost: SegmentColors {
                  icon: "\x1b[92m".to_string(),   // 亮綠色
                  label: "\x1b[32m".to_string(),  // 綠色
                  value: "\x1b[92m".to_string(),  // 亮綠色
                  reset: "\x1b[0m".to_string(),
              },
              duration: SegmentColors {
                  icon: "\x1b[94m".to_string(),   // 亮藍色
                  label: "\x1b[35m".to_string(),  // 紫色
                  value: "\x1b[95m".to_string(),  // 亮紫色
                  reset: "\x1b[0m".to_string(),
              },
              git: GitColors::default(),
        }
    }
}

impl Default for GitColors {
    fn default() -> Self {
        Self {
              branch: "\x1b[34m".to_string(),   // 藍色
              ahead: "\x1b[32m".to_string(),    // 綠色（領先）
              behind: "\x1b[31m".to_string(),   // 紅色（落後）
              modified: "\x1b[33m".to_string(),  // 黃色（有修改）
              reset: "\x1b[0m".to_string(),
        }
    }
}

impl Theme {
    pub fn from_file<P: AsRef<PAth>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let theme: Self = toml::from_str(&content)?;
        Ok(theme)
    }
}