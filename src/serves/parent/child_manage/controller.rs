use axum::extract::Query;
use axum::{extract::State, Extension, Json};
use axum_resp_result::{resp_result, MapReject};
use chrono::NaiveDate;
use log::info;
use persistence::operations::ChildCheckOperate;
use persistence::output_models::child_check::MonthlyCheckItem;
use persistence::service::child_quiz_service::child_statical::{
    ChildQuizGroupStaticalItem, ChildResentCorrectStaticalItem, ResentType,
};
use persistence::service::child_quiz_service::wrong_records::WrongQuizItem;
use persistence::service::parent_child_service::all_children::ChildItem;
use persistence::service::{ChildQuizService, ChildSocialService, DbService, ParentChildService};
use persistence::{
    operations::{ChildrenOperate, OperateTrait},
    PersistenceConnection,
};

use crate::authorize::{ChildMode, ParentAuthorizeState};
use crate::serves::parent::child_manage::input_models::{ChildCheckRecord, ChildId, StaticalInput};
use crate::serves::parent::child_manage::output_models::{
    BaseChildInfo, CheckTotalInfo, ChildScoreResp,
};

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
    pub async fn remove(
        State(db): State<PersistenceConnection>,
        MapReject(ChildId { cid }): MapRejector<Json<ChildId>>,
    ) -> Result<()> {
        ChildrenOperate.delete().by_id(&db, cid).await?;
        Ok(())
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
    pub async fn quiz_group_statical(
        DbService(service): DbService<ChildQuizService>,
        Query(StaticalInput { cid, resent_days }): Query<StaticalInput>,
    ) -> Result<Vec<ChildQuizGroupStaticalItem>> {
        let ret = service
            .child_quiz_group_statical(cid, ResentType::Days(resent_days.unwrap_or(30)))
            .await?;
        Ok(ret)
    }
    #[resp_result]
    pub async fn resent_correct_statical(
        DbService(service): DbService<ChildQuizService>,
        Query(StaticalInput { cid, resent_days }): Query<StaticalInput>,
    ) -> Result<Vec<ChildResentCorrectStaticalItem>> {
        Ok(service
            .child_resent_correct_statical(cid, ResentType::Days(resent_days.unwrap_or(30)))
            .await?)
    }
    #[resp_result]
    pub async fn child_month_check(
        State(db): State<PersistenceConnection>,
        MapReject(ChildCheckRecord { cid, month, year }): MapRejector<Query<ChildCheckRecord>>,
    ) -> Result<Vec<NaiveDate>> {
        let list = ChildCheckOperate
            .retrieve()
            .spec_month_check(&db, cid, Some(month), Some(year))
            .await?
            .into_iter()
            .map(|MonthlyCheckItem { check_date }| check_date)
            .collect();
        Ok(list)
    }

    #[resp_result]
    pub async fn get_check_info(
        State(db): State<PersistenceConnection>,
        MapReject(ChildId { cid: child_id }): MapRejector<Query<ChildId>>,
    ) -> Result<CheckTotalInfo> {
        let continual_days = ChildCheckOperate
            .retrieve()
            .continual_check_days(&db, child_id)
            .await?;
        let total = ChildCheckOperate
            .retrieve()
            .total_check(&db, child_id)
            .await?;
        Ok(CheckTotalInfo::builder()
            .total(total)
            .continual(continual_days)
            .build())
    }
    #[resp_result]
    pub async fn get_score(
        DbService(service): DbService<ChildSocialService>,
        MapReject(ChildId { cid: child_id }): MapRejector<Query<ChildId>>,
    ) -> Result<ChildScoreResp> {
        let score = service.get_child_score(child_id).await?;

        Ok(score.into())
    }
    #[resp_result]
    pub async fn get_wrong_record(
        DbService(service): DbService<ChildQuizService>,
        MapReject(ChildId { cid: child_id }): MapRejector<Query<ChildId>>,
    ) -> Result<Vec<WrongQuizItem>> {
        Ok(service.get_wrong_quiz_list(child_id).await?)
    }
}
