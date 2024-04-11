use crate::router::ServeState;
use crate::serves::ControllerRouter;
use axum::routing::{delete, get, post};
use axum::Router;

mod controller;
mod error;
mod input_models;

pub struct ChildManagerController;

impl ControllerRouter for ChildManagerController {
    fn router(&self) -> Router<ServeState> {
        Router::new()
            .route("/name", get(Self::get_name))
            .route("/quiz_group", get(Self::get_all_quiz_group))
            .route("/quiz_group", post(Self::add_quiz_group))
            .route("/quiz_group", delete(Self::remove_quiz_group))
            .route("/wrong_record", get(Self::get_wrong_record))
    }

    fn base(&self) -> &str {
        "/child"
    }
}

use error::MapRejector;
use error::Result;
