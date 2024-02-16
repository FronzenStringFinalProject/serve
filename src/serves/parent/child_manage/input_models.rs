use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewChild {
    pub name: String,
}
