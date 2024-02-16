use axum::{extract::State, Extension, Json};
use axum_resp_result::{resp_result, MapReject};
use level_evaluate::AnsweredQuiz;
use persistence::{
    operations::{ChildrenOperate, OperateTrait},
    service::{quiz_record::ChildQuizAns, ChildQuizService},
    PersistenceConnection,
};

use crate::authorize::ParentAuthorizeState;

use super::{
    error::Error, input_models::QuizAns, output_models::Quiz, ChildrenQuizController, Result,
};

impl ChildrenQuizController {
    #[resp_result]
    pub async fn next(
        State(db): State<PersistenceConnection>,
        Extension(ParentAuthorizeState { child, .. }): Extension<ParentAuthorizeState>,
    ) -> Result<Quiz> {
        let ret =
            ChildQuizService::next_quiz(&db, child.ok_or(Error::ExpectInChildMode)?, 0.5, 1.0)
                .await?
                .ok_or(Error::ChildNotFound)?
                .into();
        Ok(ret)
    }

    #[resp_result]
    pub async fn submit(
        State(db): State<PersistenceConnection>,
        Extension(ParentAuthorizeState { child, .. }): Extension<ParentAuthorizeState>,
        MapReject(QuizAns { id, ans }): super::MapRejector<Json<QuizAns>>,
    ) -> Result<()> {
        let child_id = child.ok_or(Error::ExpectInChildMode)?;
        // save record
        ChildQuizService::new_ans_record(&db, child_id, id, ans).await?;
        // get recent records;
        let records = ChildQuizService::get_ans_quiz_by_child_id(&db, child_id, 25)
            .await?
            .into_iter()
            .map(
                |ChildQuizAns {
                     diff,
                     disc,
                     lambdas,
                     correct,
                     ..
                 }| AnsweredQuiz {
                    diff,
                    disc,
                    lambdas,
                    correct,
                },
            )
            .collect::<Vec<_>>();
        let ability = level_evaluate::evaluate(&records)?;

        ChildrenOperate
            .update()
            .ability(&db, child_id, ability)
            .await?;
        Ok(())
    }
}
