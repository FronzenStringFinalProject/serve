use axum::routing::get;
use axum::Router;

use error::Result;

use crate::router::ServeState;
use crate::serves::ControllerRouter;

mod controller;
mod error;
mod output_models;

pub struct ChildScoreController;

impl ControllerRouter for ChildScoreController {
    fn router(&self) -> Router<ServeState> {
        Router::new().route("/score", get(Self::get_score))
    }

    fn base(&self) -> &str {
        "/score"
    }
}
