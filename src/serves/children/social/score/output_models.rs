use persistence::service::child_social;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ChildScoreResp {
    total: i64,
    level: i64,
    level_score: i64,
}

impl From<child_social::ChildScore> for ChildScoreResp {
    fn from(
        child_social::ChildScore {
            total_score,
            current_level,
            current_level_score,
        }: child_social::ChildScore,
    ) -> Self {
        Self {
            total: total_score,
            level: current_level,
            level_score: current_level_score,
        }
    }
}
