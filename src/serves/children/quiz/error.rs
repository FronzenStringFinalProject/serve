use axum::extract::rejection::JsonRejection;
use axum_resp_result::{MapReject, RespError};

use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error, RespError)]
#[non_exhaustive]
pub enum Error {
    #[error("Read Json Error: {0}")]
    #[resp_result(err_code = "bad request")]
    Json(#[from] JsonRejection),
    #[error("Database Error: {0}")]
    Db(#[from] DbErr),
    #[error("Children not found")]
    #[resp_result(err_code = "Not Found")]
    ChildNotFound,
    #[error("Evaluate ability Error: {0}")]
    Evaluate(#[from] level_evaluate::Error),
}

pub(super) type Result<T> = core::result::Result<T, Error>;
pub(super) type MapRejector<T> = MapReject<T, Error>;
