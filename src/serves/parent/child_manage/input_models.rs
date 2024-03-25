use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewChild {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ChildId {
    pub cid: i32,
}

#[derive(Debug, Deserialize)]
pub struct StaticalInput {
    pub cid: i32,
    pub resent_days: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ChildCheckRecord {
    pub cid: i32,
    pub month: i32,
    pub year: i32,
}
