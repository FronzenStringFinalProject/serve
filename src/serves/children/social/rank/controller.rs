use crate::authorize::{ChildMode, ParentAuthorizeState};
use axum::Extension;
use axum_resp_result::resp_result;
use persistence::service::child_social::ChildRank;
use persistence::service::{ChildSocialService, DbService};

impl super::ChildrenRankController {
    #[resp_result]
    pub async fn score_rank(
        Extension(ParentAuthorizeState {
            child: ChildMode(child_id),
            ..
        }): Extension<ParentAuthorizeState<ChildMode>>,
        DbService(db): DbService<ChildSocialService>,
    ) -> super::Result<Vec<ChildRank>> {
        let ret = db.score_rank(child_id).await?;
        Ok(ret)
    }

    #[resp_result]
    pub async fn check_rank(
        Extension(ParentAuthorizeState {
            child: ChildMode(child_id),
            ..
        }): Extension<ParentAuthorizeState<ChildMode>>,
        DbService(db): DbService<ChildSocialService>,
    ) -> super::Result<Vec<ChildRank>> {
        let ret = db.check_rank(child_id).await?;
        Ok(ret)
    }
}
