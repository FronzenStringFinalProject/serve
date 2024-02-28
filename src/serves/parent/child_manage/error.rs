use axum::extract::rejection::JsonRejection;
use axum_resp_result::{MapReject, RespError};
use http::StatusCode;
use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Deserialize Json Error: {0}")]
    Json(#[from] JsonRejection),
    #[error("Database Error:{0}")]
    Db(#[from] DbErr),
    #[error("Child Id [{0}] Not Exist")]
    ChildNotFound(i32),
}

impl RespError for Error {
    fn log_message(&self) -> std::borrow::Cow<'_, str> {
        self.to_string().into()
    }
    fn http_code(&self) -> http::StatusCode {
        match self {
            Self::Json(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub(super) type Result<T> = core::result::Result<T, Error>;
pub(super) type MapRejector<T> = MapReject<T, Error>;
