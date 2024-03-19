use axum::extract::rejection::QueryRejection;
use axum_resp_result::RespError;

use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error, RespError)]
pub enum Error {
    #[error("Database Error: {0}")]
    Db(#[from] DbErr),
    #[error("Query Error : {0}")]
    #[resp_result(err_code = "Bad-request")]
    Query(#[from] QueryRejection),
    #[resp_result(err_code = "NotFound")]
    #[error("Target Record Not Found")]
    NoRecord,
}

pub type Result<T> = core::result::Result<T, Error>;
