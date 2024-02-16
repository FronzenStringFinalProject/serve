use axum::extract::rejection::JsonRejection;
use axum_resp_result::{MapReject, RespError};
use http::StatusCode;
use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Read Json Error: {0}")]
    Json(#[from] JsonRejection),
    #[error("Database Error: {0}")]
    Db(#[from] DbErr),
    #[error("Expect In Child mode")]
    ExpectInChildMode,
    #[error("Children not found")]
    ChildNotFound,
    #[error("Evaluate ability Error: {0}")]
    Evaluate(#[from] level_evaluate::Error),
}

impl RespError for Error {
    fn log_message(&self) -> std::borrow::Cow<'_, str> {
        self.to_string().into()
    }

    fn http_code(&self) -> http::StatusCode {
        match self {
            Error::Json(_) => StatusCode::BAD_REQUEST,
            Error::ExpectInChildMode => StatusCode::UNAUTHORIZED,
            Error::ChildNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub(super) type Result<T> = core::result::Result<T, Error>;
pub(super) type MapRejector<T> = MapReject<T, Error>;
