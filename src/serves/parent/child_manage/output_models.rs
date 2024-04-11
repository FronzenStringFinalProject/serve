use persistence::service::child_social;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize)]
pub struct BaseChildInfo {
    pub cid: i32,
    pub name: String,
    pub ability: f64,
}

impl From<persistence::entities::children::Model> for BaseChildInfo {
    fn from(value: persistence::entities::children::Model) -> Self {
        Self {
            cid: value.cid,
            name: value.name,
            ability: value.ability,
        }
    }
}
#[derive(Debug, Serialize, TypedBuilder)]
pub struct CheckTotalInfo {
    total: u64,
    continual: i64,
}
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
