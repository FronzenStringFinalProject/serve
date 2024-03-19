use axum_resp_result::RespError;
use http::header::ToStrError;
use persistence::sea_orm::DbErr;

#[derive(Debug, thiserror::Error, RespError)]
pub enum Error {
    #[error("header not found")]
    #[resp_result(err_code = "Unauthorized")]
    HeaderNotExist,

    #[error("Header value error : {0}")]
    #[resp_result(err_code = "bad request")]
    ToStr(#[from] ToStrError),

    #[error("Jwt Error: {0}")]
    #[resp_result(err_code = "Bad request")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("Database Error: {0}")]
    Db(#[from] DbErr),
    #[error("Db connection instance not found")]
    ConnectNotFound,
    #[error("Parent[id={0}] not found")]
    #[resp_result(err_code = "NotFound")]
    ParentNotFound(i32),
    #[error("Expect In Parent Mode, But In Child Mode")]
    #[resp_result(err_code = "Unauthorized")]
    ExpectInParentMode,
    #[error("Expect In Child Mode, but In Parent Mode")]
    #[resp_result(err_code = "Unauthorized")]
    ExpectInChildMode,
}
