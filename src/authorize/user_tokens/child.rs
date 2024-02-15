use jsonwebtoken::get_current_timestamp;
use persistence::entities::children;
use serde::{Deserialize, Serialize};

use super::{FromModel, JwtConvert};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildClaim {
    exp: usize,
    child_id: i32,
}

impl FromModel for ChildClaim {
    type Model = children::Model;

    fn from_model(model: &Self::Model) -> Self {
        Self::new(model.cid)
    }
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
