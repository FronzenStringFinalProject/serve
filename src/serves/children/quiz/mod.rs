mod controller;
mod error;
mod input_models;
mod output_models;
use axum::routing::get;
use axum::routing::post;
use axum::Router;

use crate::serves::ControllerRouter;

pub struct ChildrenQuizController;

impl ControllerRouter for ChildrenQuizController {
    fn router(&self) -> axum::Router<crate::router::ServeState> {
        Router::new()
            .route("/quiz", get(Self::next))
            .route("/submit", post(Self::submit))
    }

    fn base(&self) -> &str {
        "/child"
    }
}

use error::MapRejector;
use error::Result;
