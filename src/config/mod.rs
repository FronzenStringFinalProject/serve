use std::net::SocketAddr;

use axum_starter::{Configure, Provider};
use figment::{
    providers::{Format, Toml},
    Figment,
};
use persistence::SqlConfig;
use serde::Deserialize;
use simple_logger::SimpleLogger;
use typed_builder::TypedBuilder;
#[derive(Debug, Deserialize, Configure, Provider, TypedBuilder)]
#[provider(r#ref, transparent)]
#[conf(
    address(provide),
    logger(func = "Self::init_log", error = "log::SetLoggerError", associate),
    server
)]
pub struct ServeConfigure {
    #[provider(ignore_global, transparent)]
    pub address: SocketAddr,

    pub sql: SqlConfig,
}

impl ServeConfigure {
    pub fn init_log() -> Result<(), log::SetLoggerError> {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()?;
        Ok(())
    }
}

impl ServeConfigure {
    pub fn load() -> Self {
        Figment::new()
            .merge(Toml::file("./Config.toml"))
            .extract()
            .expect("Failure to load configure")
    }
}
