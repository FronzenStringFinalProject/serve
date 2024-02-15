use jsonwebtoken::get_current_timestamp;
use serde::{Deserialize, Serialize};

use super::JwtConvert;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildClaim {
    exp: usize,
    child_id: i32,
}

impl JwtConvert for ChildClaim {}

impl ChildClaim {
    pub fn new(child_id: i32) -> Self {
        Self {
            exp: (get_current_timestamp() + 180 * 24 * 60 * 60) as usize,
            child_id,
        }
    }
}
