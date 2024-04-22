use axum_resp_result::RespError;
use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error, RespError)]
pub enum Error {
    #[error("Database Error: {0}")]
    Db(#[from] DbErr),
}

pub type Result<T> = core::result::Result<T, Error>;
