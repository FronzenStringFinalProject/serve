mod score;
mod check_in;

use axum::Router;

use crate::serves::{ControllerRouter, RouterExt};
use crate::serves::children::social::check_in::ChildCheckInController;
use crate::serves::children::social::score::ChildScoreController;

pub struct ChildrenSocialController;

impl ControllerRouter for ChildrenSocialController {
    fn router(&self) -> axum::Router<crate::router::ServeState> {
        Router::new()
            .add_controller(ChildScoreController)
            .add_controller(ChildCheckInController)
    }

    fn base(&self) -> &str {
        "/child"
    }
}
