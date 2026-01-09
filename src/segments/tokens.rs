use crate::segment::{Segment, RenderContext, SegmentOutput};
use std::error::Error;

pub struct TokensSegment;

impl TokensSegment {
    pub fn new () -> Self {
        Self
    }

    fn format_tokens(tokens: u64) -> String {
        if tokens >= 1_000_000 {
            format!("{:.1}M", tokens as f64 / 1_000_000.0)
        } else if tokens >= 1_000 {
            format!("{:.1}K", tokens as f64 / 1_000.0)
        } else {
            tokens.to_string()
        }
    }
}


impl Segment for TokensSegment {
    fn id(&self) -> &str {
        "tokens"
    }

    fn priority(&self) -> i32 {
        20
    }

    fn render(&self, ctx: &RenderContext) -> Result<SegmentOutput, Box<dyn Error>> {
        let colors = &ctx.theme.colors.tokens;
        let window = &ctx.input.context_window;

        let total = window.total_input_tokens + window.total_output_tokens;

        let usage_pct = if window.context_window_size > 0 {
            (total as f64 / window.context_window_size as f64 * 100.0) as u32
        } else {
            0
        };

        let text= format!(
            "{}📊{} {}{} / {} ({}%){}",
            colors.icon,
            colors.reset,
            colors.value,
            Self::format_tokens(total),
            Self::format_tokens(window.context_window_size),
            usage_pct,
            colors.reset,
        );

        Ok(SegmentOutput::visible(text))
    }
}

impl Default for TokensSegment {
    fn default() -> Self {
        Self::new()
    }
}