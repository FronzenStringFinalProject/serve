use persistence::service::child_quiz_service::next_quiz::QuizFetched;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, TypedBuilder)]
pub struct Quiz {
    pub id: i32,
    pub quiz: String,
}

impl From<QuizFetched> for Quiz {
    fn from(QuizFetched { id, quiz }: QuizFetched) -> Self {
        Self { id, quiz }
    }
}
