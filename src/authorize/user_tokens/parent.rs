use jsonwebtoken::get_current_timestamp;
use persistence::entities::parent;
use serde::{Deserialize, Serialize};

use super::JwtConvert;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentClaims {
    exp: usize,
    parent_id: i32,
    pwd_version: u32,
    child_mode: bool,
}

impl JwtConvert for ParentClaims {}

impl ParentClaims {
    pub fn new(parent_id: i32, pwd_version: u32, child_mode: bool) -> Self {
        Self {
            exp: (get_current_timestamp() + 180 * 24 * 60 * 60) as usize,
            parent_id,
            pwd_version,
            child_mode,
        }
    }

    pub fn from_model(model: &parent::Model) -> Self {
        Self::new(model.pid, 0, false)
    }
}
