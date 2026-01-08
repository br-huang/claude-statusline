use crate::segment::{Segment, RenderContext, SegmentOutput};
use std::error::Error;

pub struct CostSegment;

impl CostSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for CostSegment {
    fn id(&self) -> &str {
        "cost"
    }

    fn priority(&self) -> i32 {
        30
    }

    fn render(&self, ctx: &RenderContext) -> Result<SegmentOutput, Box<dyn Error>> {
        let colors = &ctx.theme.colors.cost;
        let cost = &ctx.input.cost.total_cost_usd;

        let text = format!(
            "{}💰{} ${}{:.2}{}",
            colors.icon,
            colors.reset,
            colors.value,
            cost,
            colors.reset,
        );

        Ok(SegmentOutput::visible(text))
    }
}

impl Default for CostSegment {
    fn default() -> Self {
        Self::new()
    }
}