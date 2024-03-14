use axum_resp_result::RespError;
use http::StatusCode;
use persistence::sea_orm::DbErr;
use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database Error: {0}")]
    Db(#[from] DbErr),
    #[error("Child[{0}] Not Found")]
    ChildNotFound(i32),
}

impl RespError for Error {
    fn log_message(&self) -> Cow<'_, str> {
        self.to_string().into()
    }

    fn http_code(&self) -> StatusCode {
        match self {
            Error::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ChildNotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
