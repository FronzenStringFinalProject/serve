use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum_resp_result::{MapReject, RespError};

use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error, RespError)]
#[non_exhaustive]
pub enum Error {
    #[error("Deserialize Json Error: {0}")]
    #[resp_result(err_code = "BadRequest")]
    Json(#[from] JsonRejection),
    #[error("Parse params Error: {0}")]
    #[resp_result(err_code = "BadRequest")]
    Query(#[from] QueryRejection),
    #[error("Database Error: {0}")]
    Db(#[from] DbErr),
    #[error("Child Id [{0}] Not Exist")]
    #[resp_result(err_code = "Not found")]
    ChildNotFound(i32),
}

pub(super) type Result<T> = core::result::Result<T, Error>;
pub(super) type MapRejector<T> = MapReject<T, Error>;
