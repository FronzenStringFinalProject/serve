use std::{any::type_name, marker::PhantomData};

use axum::Extension;
use axum_starter::{prepare, PrepareMiddlewareEffect};

#[derive(Debug)]
pub struct StateToExtensionEffect<S>(PhantomData<S>);

impl<S, Sev> PrepareMiddlewareEffect<Sev> for StateToExtensionEffect<S>
where
    S: 'static + Clone + Send + Sync,
{
    type Middleware = Extension<S>;

    fn take(self, states: &mut axum_starter::StateCollector) -> Self::Middleware {
        let s = states.take::<S>().expect(&format!(
            "type[{}] not found in collectors",
            type_name::<S>()
        ));
        states.insert(s.clone());
        Extension(s)
    }
}

#[prepare(StateToExtension)]
pub fn state_to_extension<S>() -> StateToExtensionEffect<S>
where
    S: 'static + Clone + Send + Sync,
{
    StateToExtensionEffect(Default::default())
}
