pub mod middleware;
use crate::authorize::private::Seal;
use persistence::entities::parent;
use typed_builder::TypedBuilder;

pub mod user_tokens;

#[derive(Debug, Clone, TypedBuilder)]
pub struct ParentAuthorizeState<M: ChildMark = ParentMode> {
    pub model: parent::Model,
    pub child: M,
}

mod private {
    pub trait Seal {}
}

pub trait ChildMark: Seal + Copy {
    fn child_id(&self) -> Option<i32> {
        None
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ChildMode(pub i32);

impl ChildMark for ChildMode {
    fn child_id(&self) -> Option<i32> {
        self.0.into()
    }
}

impl Seal for ChildMode {}

#[derive(Debug, Copy, Clone)]
pub struct ParentMode;

impl ChildMark for ParentMode {}

impl Seal for ParentMode {}
