mod child_manage;
mod parent_auth;

use axum::Router;
use parent_auth::ParentAuthController;

use self::child_manage::ChildManageController;

use super::{ControllerRouter, RouterExt};

pub struct ParentController;

impl ControllerRouter for ParentController {
    fn router(&self) -> axum::Router<crate::router::ServeState> {
        Router::new()
            .merge_controller(ParentAuthController)
            .merge_controller(ChildManageController)
    }

    fn base(&self) -> &str {
        "/parent"
    }
}
