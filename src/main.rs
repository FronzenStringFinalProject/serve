use std::net::{Ipv4Addr, SocketAddr};

use axum_starter::ServerPrepare;
use persistence::{ConnectSQL, SqlConfig};
use router::{RootRouter, RouteFallback};
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer};
mod config;
mod middlewares;
mod router;
fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Init tokio rt failure");

    rt.block_on(entry())
}

async fn entry() {
    ServerPrepare::with_config(config::ServeConfigure {
        address: SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 6000),
        sql: SqlConfig {
            url: url::Url::parse("postgres://FrozenString:wyq020222@localhost/mydb")
                .expect("Bad Url"),
        },
    })
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
    .convert_state()
    .preparing()
    .await
    .expect("Failure to Prepare start")
    .launch()
    .await
    .expect("Service Error");
}
