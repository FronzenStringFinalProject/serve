use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, TypedBuilder)]
pub struct CheckTotalInfo {
    total: u64,
    continual: i64,
}
