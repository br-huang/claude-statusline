use crate::input::StatusInput;
use crate::theme::Theme;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct RenderContext<'a> {
    pub input: &'a StatusInput, 
    pub theme: &'a Theme,
}

#[derive(Debug, Clone)]
pub struct SegmentOutput {
    pub text: String,
    pub visible: bool,
}

impl SegmentOutput {
    pub fn visible(text: String) -> Self {
        Self { text, visible: true}
    }

    pub fn hidden() -> Self {
        Self {
            text: String::new(),
            visible: false,
        }
    }
}

pub trait Segment: Send + Sync {
    fn id(&self) -> &str;

    fn render(&self, ctx: &RenderContext) -> Result<SegmentOutput, Box<dyn Error>>;

    fn is_enabled(&self, ctx: &RenderContext) -> bool {
        ctx.theme.enabled_segments.contains(&self.id().to_string())
    }

    fn priority(&self) -> i32 {
        100 // Default priority
    }
}