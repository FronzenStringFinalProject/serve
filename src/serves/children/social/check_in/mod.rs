use crate::router::ServeState;
use crate::serves::ControllerRouter;
use axum::routing::{get, post};
use axum::Router;
use axum_resp_result::MapReject;

mod controller;
mod error;
mod input_model;
mod output_model;

pub struct ChildCheckInController;

impl ControllerRouter for ChildCheckInController {
    fn router(&self) -> Router<ServeState> {
        Router::new()
            .route("/check", post(Self::check))
            .route("/check/available", get(Self::can_check))
            .route("/check", get(Self::get_check_info))
            .route("/check/month", get(Self::get_month_check_record))
    }

    fn base(&self) -> &str {
        "/check"
    }
}

use error::{Error, Result};

type MapRejecter<T> = MapReject<T, Error>;
