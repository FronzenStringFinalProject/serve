mod controller;
mod error;
mod input_models;
mod output_models;
use axum::routing::{delete, get};
use axum::{routing::post, Router};
use tower_http::auth::AsyncRequireAuthorizationLayer;

use crate::{authorize::middleware::authorize, serves::ControllerRouter};

use error::{MapRejector, Result};

pub struct ChildManageController;

impl ControllerRouter for ChildManageController {
    fn router(&self) -> axum::Router<crate::router::ServeState> {
        Router::new()
            .route("/children", post(Self::add))
            .route("/children", delete(Self::remove))
            .route("/children/all", get(Self::all))
            .route("/children", get(Self::basic))
            .route(
                "/children/statical/quiz_group",
                get(Self::quiz_group_statical),
            )
            .route(
                "/children/statical/correct_trend",
                get(Self::resent_correct_statical),
            )
            .route("/children/check", get(Self::child_month_check))
            .route("/children/statical/check", get(Self::get_check_info))
            .route("/children/score", get(Self::get_score))
            .route("/children/wrongRecords", get(Self::get_wrong_record))
            .route("/children/activate", get(Self::get_activate_map))
            .layer(AsyncRequireAuthorizationLayer::new(
                authorize::<false, true>,
            ))
    }

    fn base(&self) -> &str {
        "/parent"
    }
}
