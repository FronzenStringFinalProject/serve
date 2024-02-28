mod controller;
mod error;
mod input_models;
mod output_models;
use axum::routing::get;
use axum::{routing::post, Router};
use tower_http::auth::AsyncRequireAuthorizationLayer;

use crate::{authorize::middleware::authorize, serves::ControllerRouter};

use error::{MapRejector, Result};

pub struct ChildManageController;

impl ControllerRouter for ChildManageController {
    fn router(&self) -> axum::Router<crate::router::ServeState> {
        Router::new()
            .route("/children", post(Self::add))
            .route("/children/all", get(Self::all))
            .route("/children", get(Self::basic))
            .route("/children/statical", get(Self::statical))
            .layer(AsyncRequireAuthorizationLayer::new(
                authorize::<false, true>,
            ))
    }

    fn base(&self) -> &str {
        "/parent"
    }
}
