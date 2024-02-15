use axum_resp_result::RespError;
use http::{header::ToStrError, StatusCode};
use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("header not found")]
    HeaderNotExist,

    #[error("Header value error : {0}")]
    ToStr(#[from] ToStrError),

    #[error("Jwt Error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("Database Error: {0}")]
    Db(#[from] DbErr),
    #[error("Db connection instance not found")]
    ConnectNotFound,
    #[error("Parent[id={0}] not found")]
    ParentNotFound(i32),
}

impl RespError for Error {
    fn http_code(&self) -> http::StatusCode {
        match self {
            Self::HeaderNotExist | Self::ParentNotFound(_) => StatusCode::NOT_FOUND,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn log_message(&self) -> std::borrow::Cow<'_, str> {
        self.to_string().into()
    }
}
