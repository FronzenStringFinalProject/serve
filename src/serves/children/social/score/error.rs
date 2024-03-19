use axum_resp_result::RespError;
use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error, RespError)]
pub enum Error {
    #[error("Database Error: {0}")]
    Db(#[from] DbErr),
    #[error("Child[{0}] Not Found")]
    #[resp_result(err_code = "not found")]
    ChildNotFound(i32),
}

pub type Result<T> = core::result::Result<T, Error>;
