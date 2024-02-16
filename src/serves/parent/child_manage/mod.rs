mod controller;
mod error;
mod input_models;
mod output_models;
use axum::{routing::post, Router};

use crate::serves::ControllerRouter;

use error::{MapRejector, Result};

pub struct ChildManageController;

impl ControllerRouter for ChildManageController {
    fn router(&self) -> axum::Router<crate::router::ServeState> {
        Router::new().route("/children", post(Self::add))
    }

    fn base(&self) -> &str {
        "/parent"
    }
}
