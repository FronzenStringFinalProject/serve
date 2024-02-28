use axum::extract::{Path, Query};
use axum::{extract::State, Extension, Json};
use axum_resp_result::{resp_result, MapReject};
use log::info;
use persistence::service::child_quiz_service::child_statical::{ChildStaticalItem, ResentType};
use persistence::service::parent_child_service::all_children::ChildItem;
use persistence::service::{ChildQuizService, DbService, ParentChildService};
use persistence::{
    operations::{ChildrenOperate, OperateTrait},
    PersistenceConnection,
};

use crate::authorize::ParentAuthorizeState;
use crate::serves::parent::child_manage::input_models::{ChildId, StaticalInput};
use crate::serves::parent::child_manage::output_models::BaseChildInfo;

use super::{input_models::NewChild, MapRejector, Result};

impl super::ChildManageController {
    #[resp_result]
    pub async fn add(
        State(db): State<PersistenceConnection>,
        Extension(auth): Extension<ParentAuthorizeState>,
        MapReject(NewChild { name }): MapRejector<Json<NewChild>>,
    ) -> Result<i32> {
        info!("parent id: {}", auth.model.pid);
        let ret = ChildrenOperate
            .insert()
            .new_child(&db, name, auth.model.pid)
            .await?;

        Ok(ret)
    }
    #[resp_result]
    pub async fn all(
        DbService(service): DbService<ParentChildService<PersistenceConnection>>,
        Extension(auth): Extension<ParentAuthorizeState>,
    ) -> Result<Vec<ChildItem>> {
        let ret = service.all_children(auth.model.pid).await?;
        Ok(ret)
    }
    #[resp_result]
    pub async fn basic(
        State(db): State<PersistenceConnection>,
        Query(ChildId { cid }): Query<ChildId>,
    ) -> Result<BaseChildInfo> {
        let child = ChildrenOperate
            .retrieve()
            .by_id(&db, cid)
            .await?
            .ok_or(super::error::Error::ChildNotFound(cid))?
            .into();
        Ok(child)
    }
    #[resp_result]
    pub async fn statical(
        DbService(service): DbService<ChildQuizService>,
        Query(StaticalInput {
            child: ChildId { cid },
            resent_days,
        }): Query<StaticalInput>,
    ) -> Result<Vec<ChildStaticalItem>> {
        let ret = service
            .child_statical(cid, ResentType::Days(resent_days.unwrap_or(30)))
            .await?;
        Ok(ret)
    }
}
