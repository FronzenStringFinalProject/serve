use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QuizAns {
    pub id: i32,
    pub ans: i32,
}
