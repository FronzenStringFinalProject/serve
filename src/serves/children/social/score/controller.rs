use super::{Error, Result};
use crate::authorize::{ChildMode, ParentAuthorizeState};
use crate::serves::children::social::score::output_models::ChildScoreResp;
use crate::serves::children::social::score::ChildScoreController;
use axum::Extension;

use axum_resp_result::resp_result;
use persistence::service::{ChildSocialService, DbService};

impl ChildScoreController {
    #[resp_result]
    pub async fn get_score(
        DbService(service): DbService<ChildSocialService>,
        Extension(ParentAuthorizeState {
            child: ChildMode(child_id),
            ..
        }): Extension<ParentAuthorizeState<ChildMode>>,
    ) -> Result<ChildScoreResp> {
        let score = service
            .get_child_score(child_id)
            .await?
            .ok_or_else(|| Error::ChildNotFound(child_id))?;

        Ok(score.into())
    }
}
