use thiserror::Error;

/// List of errors that we can return as part of the results. We are using the external crate
/// thiserror which allows us to set up this error enum very easily without a lot of boilerplate.
/// Specifically, when the error is printed out the the screen, if the error was `ComponentNotFound`
/// Then the string `Component for entity not found` is what would print out before the stack trace.
///
/// thiserror can do a lot more, including taking in variables.
#[derive(Debug, Error, PartialEq)]
pub enum Errors {
    #[error("Component for entity not found")]
    ComponentNotFound,
}
