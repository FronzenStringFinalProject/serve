mod score;

use axum::Router;

use crate::serves::ControllerRouter;

pub struct ChildrenSocialController;

impl ControllerRouter for ChildrenSocialController {
    fn router(&self) -> axum::Router<crate::router::ServeState> {
        Router::new()
    }

    fn base(&self) -> &str {
        "/child"
    }
}
