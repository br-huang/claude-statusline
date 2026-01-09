use crate::segment::{Segment, RenderContext, SegmentOutput};
use std::error::Error;

pub struct DurationSegment;

impl DurationSegment {
    pub fn new() -> Self {
        Self
    }

    fn format_duration(duration: u64) -> String {
        let total_seconds = duration / 1000;

        if total_seconds >= 3600 {
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            format!("{}h {}m", hours, minutes)
        } else if total_seconds >= 60 {
            let minutes = total_seconds / 60;
            let seconds = total_seconds % 60;
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", total_seconds)
        }
    }
}

impl Segment for DurationSegment {
    fn id(&self) -> &str {
        "duration"
    }

    fn priority(&self) -> i32 {
        40
    }

    fn render(&self, ctx: &RenderContext) -> Result<SegmentOutput, Box<dyn Error>> {
        let colors = &ctx.theme.colors.duration;
        let duration_ms = ctx.input.cost.total_duration_ms;

        let text = format!(
            "{}🕒{} {}{}{}",
            colors.icon,
            colors.reset,
            colors.value,
            Self::format_duration(duration_ms),
            colors.reset,
        );

        Ok(SegmentOutput::visible(text))
    }
}

impl Default for DurationSegment {
    fn default() -> Self {
        Self::new()
    }
}