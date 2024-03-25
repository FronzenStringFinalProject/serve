use super::MapRejector;
use super::Result;
use crate::authorize::{ChildMode, ParentAuthorizeState};
use crate::serves::children::manage::input_models::QuizGroup;
use axum::{extract::State, Extension, Json};
use axum_resp_result::{resp_result, MapReject};
use persistence::{
    operations::ChildQuizGroupOperate,
    operations::OperateTrait,
    service::child_quiz_service::fetch_quiz_group::QuizGroupItem,
    service::{ChildQuizService, DbService},
    PersistenceConnection,
};

impl super::ChildManagerController {
    #[resp_result]
    pub async fn get_all_quiz_group(
        DbService(service): DbService<ChildQuizService>,
        Extension(ParentAuthorizeState {
            child: ChildMode(child_id),
            ..
        }): Extension<ParentAuthorizeState<ChildMode>>,
    ) -> Result<Vec<QuizGroupItem>> {
        let data = service.fetch_all_quiz_group(child_id).await?;
        Ok(data)
    }
    #[resp_result]
    pub async fn add_quiz_group(
        State(db): State<PersistenceConnection>,
        Extension(ParentAuthorizeState {
            child: ChildMode(child_id),
            ..
        }): Extension<ParentAuthorizeState<ChildMode>>,
        MapReject(QuizGroup { gid }): MapRejector<Json<QuizGroup>>,
    ) -> Result<()> {
        ChildQuizGroupOperate
            .insert()
            .add(&db, child_id, gid)
            .await?;
        Ok(())
    }
    #[resp_result]
    pub async fn remove_quiz_group(
        State(db): State<PersistenceConnection>,
        Extension(ParentAuthorizeState {
            child: ChildMode(child_id),
            ..
        }): Extension<ParentAuthorizeState<ChildMode>>,
        MapReject(QuizGroup { gid }): MapRejector<Json<QuizGroup>>,
    ) -> Result<()> {
        ChildQuizGroupOperate
            .delete()
            .one(&db, child_id, gid)
            .await?;
        Ok(())
    }
}
