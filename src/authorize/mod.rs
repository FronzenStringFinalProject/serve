pub mod middleware;
use persistence::entities::parent;
use typed_builder::TypedBuilder;

pub mod user_tokens;

#[derive(Debug, Clone, TypedBuilder)]
pub struct ParentAuthorizeState {
    pub model: parent::Model,
    pub child: Option<i32>,
}
