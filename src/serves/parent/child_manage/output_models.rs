use serde::Serialize;

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
