use crate::router::ServeState;
use crate::serves::ControllerRouter;
use axum::routing::get;
use axum::Router;

mod controller;
mod error;
mod output_models;

pub struct ChildScoreController;

impl ControllerRouter for ChildScoreController {
    fn router(&self) -> Router<ServeState> {
        Router::new().route("", get(Self::get_score))
    }

    fn base(&self) -> &str {
        "/score"
    }
}

use error::{Error, Result};
