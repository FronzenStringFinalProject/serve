use axum::Router;

use crate::serves::ControllerRouter;

pub struct ChildManageController;

impl ControllerRouter for ChildManageController {
    fn router(&self) -> axum::Router<crate::router::ServeState> {
        Router::new()
    }

    fn base(&self) -> &str {
        "/parent"
    }
}
