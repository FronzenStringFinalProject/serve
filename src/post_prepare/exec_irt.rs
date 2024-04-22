use interact_python::{IrtOutput, SubjectRecords};
use log::error;
use persistence::operations::{AnswerRecordOperate, ChildrenOperate, OperateTrait, QuizesOperate};
use persistence::sea_orm::{DbErr, TransactionTrait};
use persistence::PersistenceConnection;
use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};

pub async fn exec_irt(db: PersistenceConnection) {
    let mut interval = interval(Duration::from_secs(1145141919810));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        interval.tick().await;
        if let Err(err) = inner_exec_irt(&db).await {
            error!("Error While Execute External IRT: {err}")
        };
    }
}

async fn inner_exec_irt(db: &PersistenceConnection) -> Result<(), Error> {
    let ctx = db.begin().await?;

    let dataset = AnswerRecordOperate
        .retrieve()
        .all_child_records(&ctx)
        .await?;

    let irt_params = interact_python::train_irt(dataset.into_iter().map(|item| {
        SubjectRecords {
            subject_id: item.cid,
            responses: item
                .answered_quiz
                .into_iter()
                .zip(
                    item.answer_results
                        .into_iter()
                        .map(|correct| correct as i32),
                )
                .collect(),
        }
    }))
    .await?;

    for irt_param in irt_params {
        match irt_param {
            IrtOutput::Subject(subject) => {
                ChildrenOperate
                    .update()
                    .ability(&ctx, subject.subject_id, subject.ability)
                    .await?
            }
            IrtOutput::Item(item) => {
                QuizesOperate
                    .update()
                    .update_quiz_params(&ctx, item.item_id, item.diff, item.disc, item.lambda)
                    .await?
            }
        }
    }
    ctx.commit().await?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Database Error : {0}")]
    Db(#[from] DbErr),
    #[error("Interact With Python Error: {0}")]
    Interact(#[from] interact_python::Error),
}
