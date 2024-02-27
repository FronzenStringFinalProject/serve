use axum::{extract::State, Extension, Json};
use axum_resp_result::{resp_result, MapReject};
use log::info;
use persistence::{
    operations::{ChildrenOperate, OperateTrait},
    PersistenceConnection,
};

use crate::authorize::ParentAuthorizeState;

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
}
