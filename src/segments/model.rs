use crate::segment::{Segment, RenderContext, SegmentOutput};
use std::error::Error;

pub struct ModelSegment;

impl ModelSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for ModelSegment {
    fn id(&self) -> &str {
        "model"
    }

    fn priority(&self) -> i32 {
        10 // Highest priority
    }

    fn render(&self, ctx: &RenderContext) -> Result<SegmentOutput, Box<dyn Error>> {
        let colors = &ctx.theme.colors.model;
        let model_name = &ctx.input.model.display_name;

        let text = format!(
            "{}🤖{} {}{}{}",
              colors.icon,           // icon 顏色
              colors.reset,          // 重置
              colors.value,          // 值的顏色
              model_name,            // 模型名稱
              colors.reset,          // 重置
        );

        Ok(SegmentOutput::visible(text))
    }
}

impl Default for ModelSegment {
    fn default() -> Self {
        Self::new()
    }
}