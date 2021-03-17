use thiserror::Error;

use crate::components::Components;
use crate::resources::resource::Resource;

#[derive(Debug, Error)]
pub enum BbEcsError {
    #[error("attempted to cast component from {from:?} to {to:?}")]
    CastingComponents { from: Components, to: Components },
    #[error("attempted to cast resource from {from:?} to {to:?}")]
    CastingResource { from: Resource, to: &'static str },
    #[error("you need to register before inserting components")]
    NeedToRegister,
    #[error("component with name `{0}` not found")]
    ComponentNotFound(String),
    #[error("resource with names `{0} not found")]
    ResourceNotFound(String),
    #[error("already registered component with name `{0}`")]
    ComponentAlreadyRegistered(String),
}
