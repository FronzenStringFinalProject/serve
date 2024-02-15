use jsonwebtoken::get_current_timestamp;
use persistence::entities::parent;
use serde::{Deserialize, Serialize};

use crate::authorize::ParentAuthorizeState;

use super::{FromModel, JwtConvert};

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentClaims {
    exp: usize,
    pub parent_id: i32,
    pub pwd_version: i32,
    pub child: Option<i32>,
}

impl From<ParentAuthorizeState> for ParentClaims {
    fn from(ParentAuthorizeState { model, child }: ParentAuthorizeState) -> Self {
        let mut this = Self::from_model(&model);
        this.child = child;
        this
    }
}

impl FromModel for ParentClaims {
    type Model = parent::Model;

    fn from_model(model: &Self::Model) -> Self {
        Self::new(model.pid, model.pwd_ver, None)
    }
}

impl JwtConvert for ParentClaims {}

impl ParentClaims {
    pub fn new(parent_id: i32, pwd_version: i32, child: Option<i32>) -> Self {
        Self {
            exp: (get_current_timestamp() + 180 * 24 * 60 * 60) as usize,
            parent_id,
            pwd_version,
            child,
        }
    }

    pub fn child_mode(&mut self, child_id: i32) {
        self.child = child_id.into()
    }
    pub fn parent_mode(&mut self) {
        self.child = None
    }
}
