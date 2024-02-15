use axum::http::StatusCode;
use axum_resp_result::RespError;
use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error)]
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
}

impl RespError for Error {
    fn resp_message(&self) -> std::borrow::Cow<'_, str> {
        self.log_message()
    }

    fn http_code(&self) -> axum::http::StatusCode {
        match self {
            Error::ParentNotFound(_) => StatusCode::NOT_FOUND,
            Error::BadSecret | Error::Password => StatusCode::UNAUTHORIZED,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn resp_message_default() -> Option<std::borrow::Cow<'static, str>> {
        None
    }

    fn log_message(&self) -> std::borrow::Cow<'_, str> {
        self.to_string().into()
    }
}

pub(super) type Result<T> = core::result::Result<T, Error>;
