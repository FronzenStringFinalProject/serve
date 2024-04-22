use crate::router::ServeState;
use crate::serves::ControllerRouter;
use axum::routing::get;
use axum::Router;

mod controller;
mod error;

pub struct ChildrenRankController;

impl ControllerRouter for ChildrenRankController {
    fn router(&self) -> Router<ServeState> {
        Router::new()
            .route("/total-score", get(Self::score_rank))
            .route("/check-days", get(Self::check_rank))
    }

    fn base(&self) -> &str {
        "/rank"
    }
}

use error::Result;
