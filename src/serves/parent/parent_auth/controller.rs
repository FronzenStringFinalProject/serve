use axum::extract::{Path, State};
use axum::{Extension, Json};

use axum_resp_result::resp_result;
use persistence::operations::{OperateTrait, ParentOperate};
use persistence::PersistenceConnection;

use crate::authorize::user_tokens::parent::ParentClaims;
use crate::authorize::user_tokens::{FromModel, JwtConvert};
use crate::authorize::ParentAuthorizeState;

use super::error::{Error, Result};
use super::input_models::{ParentLogin, ParentRegister, ParentSecret};
use super::ParentAuthController;

impl ParentAuthController {
    #[resp_result]
    pub async fn register(
        State(db): State<PersistenceConnection>,
        Json(new_parent): Json<ParentRegister>,
    ) -> Result<()> {
        ParentOperate
            .insert()
            .new_parent(&db, new_parent.into())
            .await?;
        Ok(())
    }
    #[resp_result]
    pub async fn login(
        State(db): State<PersistenceConnection>,
        Json(ParentLogin { unique_id, pwd }): Json<ParentLogin>,
    ) -> Result<String> {
        let model = ParentOperate
            .retrieve()
            .by_unique_key(&db, &unique_id)
            .await?
            .ok_or(Error::ParentNotFound(unique_id))?;

        if model.password == pwd {
            let claims = ParentClaims::from_model(&model);
            let jwt = claims.encode()?;
            Ok(jwt)
        } else {
            Err(Error::Password)
        }
    }
    #[resp_result]
    pub async fn access(
        Extension(ParentAuthorizeState { model, child }): Extension<ParentAuthorizeState>,
        Json(ParentSecret { secret }): Json<ParentSecret>,
    ) -> Result<String> {
        let mut claim = ParentClaims::from_model(&model);
        if child.is_some() {
            if model.secret == secret {
                claim.parent_mode();
                Ok(claim.encode()?)
            } else {
                Err(Error::BadSecret)
            }
        } else {
            Ok(claim.encode()?)
        }
    }
    #[resp_result]
    pub async fn child(
        Extension(ParentAuthorizeState { model, .. }): Extension<ParentAuthorizeState>,
        Path(cid): Path<i32>,
    ) -> Result<String> {
        let mut claim = ParentClaims::from_model(&model);
        claim.child_mode(cid);
        Ok(claim.encode()?)
    }
}
