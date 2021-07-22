use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Errors {
    #[error("Component for entity not found")]
    ComponentNotFound,
}
