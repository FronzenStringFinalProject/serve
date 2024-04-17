use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::extract::State;
use axum::{Extension, Json};

use axum_resp_result::{resp_result, MapReject};
use log::info;
use password_hash::{PasswordHasher, SaltString};
use persistence::operations::{OperateTrait, ParentOperate};
use persistence::PersistenceConnection;
use rand::rngs::OsRng;

use crate::authorize::user_tokens::parent::ParentClaims;
use crate::authorize::user_tokens::{FromModel, JwtConvert};
use crate::authorize::{ChildMode, ParentAuthorizeState};

use super::error::{Error, MapRejecter, Result};
use super::input_models::{ChildId, ParentLogin, ParentRegister, ParentSecret};
use super::ParentAuthController;

impl ParentAuthController {
    #[resp_result]
    pub async fn register(
        State(db): State<PersistenceConnection>,
        MapReject(mut new_parent): MapRejecter<Json<ParentRegister>>,
    ) -> Result<()> {
        let encoded_pwd = tokio::task::spawn_blocking({
            let hasher = Argon2::default();
            let local_pwd = new_parent.password.clone();
            let salt = SaltString::generate(&mut OsRng);
            move || {
                let pwd = hasher.hash_password(local_pwd.as_bytes(), &salt)?;
                Ok::<_, super::error::Error>(pwd.to_string())
            }
        })
        .await??;
        let encoded_secret = tokio::task::spawn_blocking({
            let hasher = Argon2::default();
            let local_secret = new_parent.secret.clone();
            let salt = SaltString::generate(&mut OsRng);
            move || {
                let pwd = hasher.hash_password(local_secret.as_bytes(), &salt)?;
                Ok::<_, super::error::Error>(pwd.to_string())
            }
        })
        .await??;
        new_parent.secret = encoded_secret;
        new_parent.password = encoded_pwd;
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

        let hasher = Argon2::default();
        tokio::task::spawn_blocking({
            let local_pwd = model.password.clone();
            move || {
                let password_hash = PasswordHash::new(local_pwd.as_str())?;
                hasher.verify_password(pwd.as_bytes(), &password_hash)?;
                Ok::<_, super::error::Error>(())
            }
        })
        .await??;
        let claims = ParentClaims::from_model(&model);
        info!("current claims is :{:?}", claims);
        let jwt = claims.encode()?;
        Ok(jwt)
    }
    #[resp_result]
    pub async fn access(
        Extension(ParentAuthorizeState { model, .. }): Extension<ParentAuthorizeState<ChildMode>>,
        MapReject(ParentSecret { secret }): MapRejecter<Json<ParentSecret>>,
    ) -> Result<String> {
        let mut claim = ParentClaims::from_model(&model);

        let hasher = Argon2::default();
        tokio::task::spawn_blocking({
            let local_pwd = model.secret.clone();
            move || {
                let password_hash = PasswordHash::new(local_pwd.as_str())?;
                hasher.verify_password(secret.as_bytes(), &password_hash)?;
                Ok::<_, super::error::Error>(())
            }
        })
        .await??;
        claim.parent_mode();
        Ok(claim.encode()?)
    }
    #[resp_result]
    pub async fn child(
        Extension(ParentAuthorizeState { model, .. }): Extension<ParentAuthorizeState>,
        MapReject(ChildId { cid }): MapRejecter<Json<ChildId>>,
    ) -> Result<String> {
        let mut claim = ParentClaims::from_model(&model);
        claim.child_mode(cid);
        Ok(claim.encode()?)
    }

    #[resp_result]
    pub async fn parent_name(
        Extension(ParentAuthorizeState { model, .. }): Extension<ParentAuthorizeState>,
    ) -> Result<String> {
        Ok(model.name)
    }
}
