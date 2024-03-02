use axum::{extract::State, Extension, Json};
use axum_resp_result::{resp_result, MapReject};
use level_evaluate::AnsweredQuiz;
use persistence::service::child_quiz_service::quiz_record::ChildQuizAns;
use persistence::service::DbService;
use persistence::{
    operations::{ChildrenOperate, OperateTrait},
    service::ChildQuizService,
    PersistenceConnection,
};

use crate::authorize::{ChildMode, ParentAuthorizeState};

use super::{
    error::Error, input_models::QuizAns, output_models::Quiz, ChildrenQuizController, Result,
};

impl ChildrenQuizController {
    #[resp_result]
    pub async fn next(
        DbService(service): DbService<ChildQuizService>,
        Extension(ParentAuthorizeState {
            child: ChildMode(cid),
            ..
        }): Extension<ParentAuthorizeState<ChildMode>>,
    ) -> Result<Quiz> {
        let ret = service
            .next_quiz(cid, 0.2, 1.0)
            .await?
            .ok_or(Error::ChildNotFound)?
            .into();
        Ok(ret)
    }

    #[resp_result]
    pub async fn submit(
        DbService(service): DbService<ChildQuizService>,
        State(db): State<PersistenceConnection>,
        Extension(ParentAuthorizeState {
            child: ChildMode(child_id),
            ..
        }): Extension<ParentAuthorizeState<ChildMode>>,
        MapReject(QuizAns { id, ans }): super::MapRejector<Json<QuizAns>>,
    ) -> Result<bool> {
        // save record
        let correct = service.new_ans_record(child_id, id, ans).await?;
        // get recent records;
        let records = service
            .get_ans_quiz_by_child_id(child_id, 25)
            .await?
            .into_iter()
            .map(
                |ChildQuizAns {
                     diff,
                     disc,
                     lambda,
                     correct,
                     ..
                 }| AnsweredQuiz {
                    diff,
                    disc,
                    lambdas: lambda,
                    correct,
                },
            )
            .collect::<Vec<_>>();
        let ability = level_evaluate::evaluate(&records)?;

        ChildrenOperate
            .update()
            .ability(&db, child_id, ability)
            .await?;
        Ok(correct)
    }
}
