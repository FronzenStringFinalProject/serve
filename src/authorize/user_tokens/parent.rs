use jsonwebtoken::get_current_timestamp;
use persistence::entities::parent;
use serde::{Deserialize, Serialize};

use super::{FromModel, JwtConvert};

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentClaims {
    exp: usize,
    parent_id: i32,
    pwd_version: i32,
    child_mode: bool,
}

impl FromModel for ParentClaims {
    type Model = parent::Model;

    fn from_model(model: &Self::Model) -> Self {
        Self::new(model.pid, model.pwd_ver, false)
    }
}

impl JwtConvert for ParentClaims {}

impl ParentClaims {
    pub fn new(parent_id: i32, pwd_version: i32, child_mode: bool) -> Self {
        Self {
            exp: (get_current_timestamp() + 180 * 24 * 60 * 60) as usize,
            parent_id,
            pwd_version,
            child_mode,
        }
    }
}
