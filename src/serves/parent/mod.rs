mod child_manage;
mod parent_auth;

use axum::Router;
use parent_auth::ParentAuthController;

use super::ControllerRouter;

pub struct ParentController;

impl ControllerRouter for ParentController {
    fn router(&self) -> axum::Router<crate::router::ServeState> {
        Router::new().merge(ParentAuthController.router())
    }

    fn base(&self) -> &str {
        "/parent"
    }
}
