use axum::{
    http::{StatusCode, Uri},
    routing::get,
    Router,
};
use axum_macros::FromRef;
use axum_starter::{
    prepare,
    router::{Fallback, Nest},
    FromStateCollector, PrepareRouteEffect,
};

use persistence::PersistenceConnection;
#[derive(Debug, Clone, FromStateCollector, FromRef)]
pub struct ServeState {
    db: PersistenceConnection,
}

pub fn root_router() -> Router<ServeState> {
    Router::new().route("/echo", get(|| async { "Hallo world" }))
}

#[prepare(RootRouter)]
pub fn set_root_router() -> Nest<ServeState> {
    Nest::new("/api/v0", root_router())
}

#[prepare(RouteFallback)]
pub fn set_fallback() -> impl PrepareRouteEffect<ServeState> {
    async fn fallback(uri: Uri) -> (StatusCode, String) {
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
    Fallback::new(fallback)
}
