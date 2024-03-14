mod check_in;
mod score;

use axum::Router;

use crate::serves::children::social::check_in::ChildCheckInController;
use crate::serves::children::social::score::ChildScoreController;
use crate::serves::{ControllerRouter, RouterExt};

pub struct ChildrenSocialController;

impl ControllerRouter for ChildrenSocialController {
    fn router(&self) -> axum::Router<crate::router::ServeState> {
        Router::new()
            .merge_controller(ChildScoreController)
            .merge_controller(ChildCheckInController)
    }

    fn base(&self) -> &str {
        "/child"
    }
}
