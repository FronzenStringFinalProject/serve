use axum::extract::{Query, State};
use axum::Extension;
use axum_resp_result::resp_result;
use persistence::operations::{ChildCheckOperate, OperateTrait};
use persistence::sea_orm::prelude::Date;
use persistence::PersistenceConnection;

use crate::authorize::{ChildMode, ParentAuthorizeState};
use crate::serves::children::social::check_in::input_model::SpecMonth;
use crate::serves::children::social::check_in::output_model::CheckTotalInfo;

use super::{ChildCheckInController, MapReject, MapRejecter, Result};

impl ChildCheckInController {
    #[resp_result]
    pub async fn check(
        State(db): State<PersistenceConnection>,
        Extension(ParentAuthorizeState {
            child: ChildMode(child_id),
            ..
        }): Extension<ParentAuthorizeState<ChildMode>>,
    ) -> Result<()> {
        ChildCheckOperate.insert().check(&db, child_id).await?;
        Ok(())
    }
    #[resp_result]
    pub async fn can_check(
        State(db): State<PersistenceConnection>,
        Extension(ParentAuthorizeState {
            child: ChildMode(child_id),
            ..
        }): Extension<ParentAuthorizeState<ChildMode>>,
    ) -> Result<bool> {
        let ab = ChildCheckOperate
            .retrieve()
            .can_check(&db, child_id)
            .await?;
        Ok(ab)
    }

    #[resp_result]
    pub async fn get_check_info(
        State(db): State<PersistenceConnection>,
        Extension(ParentAuthorizeState {
            child: ChildMode(child_id),
            ..
        }): Extension<ParentAuthorizeState<ChildMode>>,
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
    pub async fn get_month_check_record(
        State(db): State<PersistenceConnection>,
        Extension(ParentAuthorizeState {
            child: ChildMode(child_id),
            ..
        }): Extension<ParentAuthorizeState<ChildMode>>,
        MapReject(SpecMonth { month, year }): MapRejecter<Query<SpecMonth>>,
    ) -> Result<Vec<Date>> {
        let resp = ChildCheckOperate
            .retrieve()
            .spec_month_check(&db, child_id, month.map(|month| month.0), year)
            .await?;

        Ok(resp.into_iter().map(|item| item.check_date).collect())
    }
}
