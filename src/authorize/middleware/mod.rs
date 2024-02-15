mod error;
use axum::{
    body::Body,
    http::{Request, Response},
    response::IntoResponse,
};
use axum_resp_result::{resp_result, RespResult};
use persistence::{
    operations::{OperateTrait, ParentOperate},
    PersistenceConnection,
};

use crate::authorize::{
    middleware::error::Error,
    user_tokens::{parent::ParentClaims, JwtConvert},
};

use super::ParentAuthorizeState;

pub async fn authorize(mut request: Request<Body>) -> Result<Request<Body>, Response<Body>> {
    let result = preform_authorize(&mut request).await;
    if let this @ RespResult::Err(_) = result {
        Err(this.into_response())
    } else {
        Ok(request)
    }
}

#[resp_result]
async fn preform_authorize(req: &mut Request<Body>) -> Result<(), Error> {
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .ok_or(Error::HeaderNotExist)?;

    let token = token.to_str()?;

    let ParentClaims {
        parent_id,
        pwd_version,
        child,
        ..
    } = ParentClaims::decode(token)?;

    let db = req
        .extensions()
        .get::<PersistenceConnection>()
        .ok_or(Error::ConnectNotFound)?;

    let model = ParentOperate
        .retrieve()
        .by_id_and_pwd_version(db, parent_id, pwd_version)
        .await?
        .ok_or(Error::ParentNotFound(parent_id))?;

    let state = ParentAuthorizeState::builder()
        .child(child)
        .model(model)
        .build();

    req.extensions_mut().insert(state);

    Ok(())
}
