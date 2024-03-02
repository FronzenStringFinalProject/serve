use axum::extract::rejection::ExtensionRejection;
use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
};
use axum_resp_result::RespError;
use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("database error: {0}")]
    Sql(#[from] DbErr),
    #[error("parent not found error :{0}")]
    ParentNotFound(String),
    #[error("password not match error")]
    Password,
    #[error("Jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("access secret not match")]
    BadSecret,
    #[error("Deserialize Json Error: {0}")]
    Json(#[from] JsonRejection),
    #[error("Read Path arguments Error: {0}")]
    Path(#[from] PathRejection),
    #[error("Extension Item Not Found: {0}")]
    Extension(#[from] ExtensionRejection),
}

impl RespError for Error {
    fn log_message(&self) -> std::borrow::Cow<'_, str> {
        self.to_string().into()
    }

    fn http_code(&self) -> axum::http::StatusCode {
        match self {
            Error::ParentNotFound(_) => StatusCode::NOT_FOUND,
            Error::BadSecret | Error::Password => StatusCode::UNAUTHORIZED,
            Error::Json(_) | Error::Path(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub(super) type Result<T> = core::result::Result<T, Error>;
pub(super) type MapRejecter<T> = axum_resp_result::MapReject<T, Error>;
