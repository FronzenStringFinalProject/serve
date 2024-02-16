use axum::{
    extract::OriginalUri,
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
    Router,
};
use axum_macros::FromRef;
use axum_resp_result::{RespError, RespResult};
use axum_starter::{
    prepare,
    router::{Fallback, Nest},
    FromStateCollector, PrepareRouteEffect,
};

use persistence::PersistenceConnection;

use crate::serves::{ChildrenController, ParentController, RouterExt};
#[derive(Debug, Clone, FromStateCollector, FromRef)]
pub struct ServeState {
    db: PersistenceConnection,
}

pub fn root_router() -> Router<ServeState> {
    Router::new()
        .add_controller(ParentController)
        .add_controller(ChildrenController)
}

#[prepare(RootRouter)]
pub fn set_root_router() -> Nest<ServeState> {
    Nest::new("/api/v0", root_router())
}

#[prepare(RouteFallback)]
pub fn set_fallback() -> impl PrepareRouteEffect<ServeState> {
    async fn fallback(OriginalUri(uri): OriginalUri) -> Response {
        #[derive(Debug, thiserror::Error)]
        #[error("No route for {0}")]
        struct RouterNotFound(Uri);

        impl RespError for RouterNotFound {
            fn log_message(&self) -> std::borrow::Cow<'_, str> {
                self.to_string().into()
            }
            fn http_code(&self) -> http::StatusCode {
                StatusCode::NOT_FOUND
            }
        }
        RespResult::<(), _>::Err(RouterNotFound(uri)).into_response()
    }
    Fallback::new(fallback)
}
