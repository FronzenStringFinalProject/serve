use axum::{routing::post, Router};
use tower_http::auth::AsyncRequireAuthorizationLayer;

use crate::{authorize::middleware::authorize, router::ServeState};

use super::ControllerRouter;

mod controller;
mod error;
mod input_models;

pub struct ParentAuthController;

impl ControllerRouter for ParentAuthController {
    fn base(&self) -> &str {
        "/parent"
    }

    fn router(&self) -> Router<ServeState> {
        Router::new()
            .route("/signin", post(Self::register))
            .route("/login", post(Self::login))
            .merge(
                Router::new()
                    .route("/access", post(Self::access))
                    .route("/to_child/:cid", post(Self::child))
                    .layer(AsyncRequireAuthorizationLayer::new(
                        authorize::<false, false>,
                    )),
            )
    }
}
