use thiserror::Error;

#[derive(Debug, Error)]
pub enum BbEcsError {
    #[error("attempted to cast component from `{0}`")]
    CastingComponents(&'static str),
    #[error("attempted to cast resource from to `{0}`")]
    CastingResource(&'static str),
    #[error("you need to register before inserting components")]
    NeedToRegister,
    #[error("component with name `{0}` not found")]
    ComponentNotFound(String),
    #[error("resource with names `{0} not found")]
    ResourceNotFound(String),
    #[error("already registered component with name `{0}`")]
    ComponentAlreadyRegistered(String),
    #[error("tried to insert into the bitmap before registering")]
    BitMapInsertBeforeRegister,
    #[error("BitMap component `{0}` not found")]
    BitMapComponentNotFound(String),
    #[error(
        "You tried to access a vector with an index that is greater than the length of the vector"
    )]
    OutOfRangeInVector,
}
