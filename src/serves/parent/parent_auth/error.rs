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
    #[error("Jwt error: {0}")]
    #[resp_result(err_code = 401)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("Deserialize Json Error: {0}")]
    #[resp_result(err_code = "BadRequest")]
    Json(#[from] JsonRejection),
    #[error("Read Path arguments Error: {0}")]
    #[resp_result(err_code = "BadRequest")]
    Path(#[from] PathRejection),
    #[error("Extension Item Not Found: {0}")]
    Extension(#[from] ExtensionRejection),
    #[error("Argon2 Verify Failure: {0}")]
    #[resp_result(err_code = 401, err_msg = "Bad Password")]
    Argon2(#[from] argon2::Error),
    #[error("PasswordHash Encode Error : {0}")]
    #[resp_result(err_msg = "Bad Password Encode")]
    PasswordHash(#[from] password_hash::Error),
    #[error("Tokio Join Task Error : {0}")]
    TokioTask(#[from] tokio::task::JoinError),
}

pub(super) type Result<T> = core::result::Result<T, Error>;
pub(super) type MapRejecter<T> = axum_resp_result::MapReject<T, Error>;
