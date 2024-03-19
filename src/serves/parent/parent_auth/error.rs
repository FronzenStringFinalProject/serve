use axum::extract::rejection::ExtensionRejection;
use axum::extract::rejection::{JsonRejection, PathRejection};
use axum_resp_result::RespError;
use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error, RespError)]
#[non_exhaustive]
pub enum Error {
    #[error("database error: {0}")]
    Sql(#[from] DbErr),
    #[error("parent not found error :{0}")]
    #[resp_result(err_code = "NotFound")]
    ParentNotFound(String),
    #[error("password not match error")]
    #[resp_result(err_code = 401)]
    Password,
    #[error("Jwt error: {0}")]
    #[resp_result(err_code = 401)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("access secret not match")]
    #[resp_result(err_code = 401)]
    BadSecret,
    #[error("Deserialize Json Error: {0}")]
    #[resp_result(err_code = "BadRequest")]
    Json(#[from] JsonRejection),
    #[error("Read Path arguments Error: {0}")]
    #[resp_result(err_code = "BadRequest")]
    Path(#[from] PathRejection),
    #[error("Extension Item Not Found: {0}")]
    Extension(#[from] ExtensionRejection),
}

pub(super) type Result<T> = core::result::Result<T, Error>;
pub(super) type MapRejecter<T> = axum_resp_result::MapReject<T, Error>;
