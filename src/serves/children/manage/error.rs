use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum_resp_result::{MapReject, RespError};
use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error, RespError)]
pub enum Error {
    #[error("Parse Params Failure: {0}")]
    #[resp_result(err_code = "BadRequest")]
    Query(#[from] QueryRejection),
    #[error("Parse Json Failure: {0}")]
    #[resp_result(err_code = "BadRequest")]
    Json(#[from] JsonRejection),
    #[error("Database Error: {0}")]
    #[resp_result(err_msg = "Database Error")]
    Db(#[from] DbErr),
    #[error("Child[{0}] Not Found")]
    #[resp_result(err_code = "NotFound")]
    ChildNotFound(i32),
}

pub type Result<T> = core::result::Result<T, Error>;
pub type MapRejector<T> = MapReject<T, Error>;
