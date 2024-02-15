mod parent;
use axum::Router;

use crate::router::ServeState;

pub trait ControllerRouter {
    fn router(&self) -> Router<ServeState>;
    fn base(&self) -> &str;
}

pub trait RouterExt: Sized {
    fn add_controller<C: ControllerRouter>(self, controller: C) -> Self;
}

impl RouterExt for Router<ServeState> {
    fn add_controller<C: ControllerRouter>(self, controller: C) -> Self {
        self.nest(controller.base(), controller.router())
    }
}

pub use parent::ParentController;
