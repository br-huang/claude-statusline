use crate::segment::{Segment, RenderContext};
use crate::theme::Theme;
use std::error::Error;
use std::sync::Arc;

pub struct OutputRenderer {
    separator: String,
}

impl OutputRenderer {
    pub fn new(theme: &Theme) -> Self {
        Self {
            separator: theme.separator.clone(),
        }
    }

    pub fn render(
        &self,
        segments: &[Arc<dyn Segment>],
        ctx: &RenderContext,
    ) -> Result<String, Box<dyn Error>> {
        let mut parts: Vec<String> = Vec::new();

        for segment in segments {
            if segment.is_enabled(ctx) {
                match segment.render(ctx) {
                    Ok(output) if output.visible => {
                        // Sucess and render
                        parts.push(output.text);
                    }
                    Ok(_) => {
                        // Do nothing, skip
                        continue;
                    } Err(e) => {
                        // Error and log
                        eprintln!("Segment {} error: {}", segment.id(), e);
                    }
                }
            }
        }

        Ok(parts.join(&self.separator))
    }
}