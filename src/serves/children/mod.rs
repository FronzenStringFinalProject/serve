mod manage;
mod quiz;
mod social;

use axum::Router;
use tower_http::auth::AsyncRequireAuthorizationLayer;

use crate::authorize::middleware::authorize;
use crate::serves::children::manage::ChildManagerController;

use self::{quiz::ChildrenQuizController, social::ChildrenSocialController};

use super::{ControllerRouter, RouterExt};

pub struct ChildrenController;

impl ControllerRouter for ChildrenController {
    fn router(&self) -> axum::Router<crate::router::ServeState> {
        Router::new()
            .merge_controller(ChildrenSocialController)
            .merge_controller(ChildrenQuizController)
            .merge_controller(ChildManagerController)
            .layer(AsyncRequireAuthorizationLayer::new(
                authorize::<true, false>,
            ))
    }

    fn base(&self) -> &str {
        "/child"
    }
}
