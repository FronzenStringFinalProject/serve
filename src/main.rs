use axum::routing::Route;
use axum_starter::ServerPrepare;
use persistence::{ConnectSQL, PersistenceConnection};
use router::{RootRouter, RouteFallback};
use starter::StateToExtension;
use tokio::signal::ctrl_c;
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer};
mod authorize;
mod config;
mod middlewares;
mod router;
mod serves;
mod starter;
fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Init tokio rt failure");

    rt.block_on(entry())
}

async fn entry() {
    ServerPrepare::with_config(config::ServeConfigure::load())
        .init_logger()
        .expect("Init logger failure")
        // init connections / service
        .prepare_state(ConnectSQL)
        // set service router
        .prepare_route(RootRouter)
        .prepare_route(RouteFallback)
        // middleware
        .layer(TraceLayer::new_for_http())
        .layer(CatchPanicLayer::new())
        // move state
        .prepare_middleware::<Route, _>(StateToExtension::<_, PersistenceConnection>)
        .convert_state()
        .graceful_shutdown(async {
            ctrl_c().await.ok();
        })
        .preparing()
        .await
        .expect("Failure to Prepare start")
        .launch()
        .await
        .expect("Service Error");
}
