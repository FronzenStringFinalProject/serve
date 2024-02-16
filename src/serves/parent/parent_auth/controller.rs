use axum::extract::{Path, State};
use axum::{Extension, Json};

use axum_resp_result::{resp_result, MapReject};
use persistence::operations::{OperateTrait, ParentOperate};
use persistence::PersistenceConnection;

use crate::authorize::user_tokens::parent::ParentClaims;
use crate::authorize::user_tokens::{FromModel, JwtConvert};
use crate::authorize::ParentAuthorizeState;

use super::error::{Error, MapRejecter, Result};
use super::input_models::{ParentLogin, ParentRegister, ParentSecret};
use super::ParentAuthController;

impl ParentAuthController {
    #[resp_result]
    pub async fn register(
        State(db): State<PersistenceConnection>,
        MapReject(new_parent): MapRejecter<Json<ParentRegister>>,
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
        MapReject(ParentLogin { unique_id, pwd }): MapRejecter<Json<ParentLogin>>,
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
        MapReject(ParentSecret { secret }): MapRejecter<Json<ParentSecret>>,
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
        MapReject(cid): MapRejecter<Path<i32>>,
    ) -> Result<String> {
        let mut claim = ParentClaims::from_model(&model);
        claim.child_mode(cid);
        Ok(claim.encode()?)
    }
}
