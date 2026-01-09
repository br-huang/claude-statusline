pub mod model;
pub mod tokens;
pub mod cost;
pub mod duration;
pub mod git;

// Re-export
pub use model::ModelSegment;
pub use tokens::TokensSegment;
pub use cost::CostSegment;
pub use duration::DurationSegment;
pub use git::GitSegment;

use crate::segment::Segment;
use std::sync::Arc;

pub struct SegmentRegistry {
    segments: Vec<Arc<dyn Segment>>,
}

impl SegmentRegistry {
    pub fn new() -> Self {
        let mut segments: Vec<Arc<dyn Segment>> = vec![
            Arc::new(ModelSegment::new()),
            Arc::new(TokensSegment::new()),
            Arc::new(CostSegment::new()),
            Arc::new(DurationSegment::new()),
            Arc::new(GitSegment::new()),
        ];
        segments.sort_by_key(|s| s.priority());
        Self { segments }
    }

    pub fn segments(&self) -> &[Arc<dyn Segment>] {
        &self.segments
    }
}

impl Default for SegmentRegistry {
    fn default() -> Self {
        Self::new()
    }
}