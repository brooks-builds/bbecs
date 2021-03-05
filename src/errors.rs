use thiserror::Error;

#[derive(Debug, Error)]
pub enum BbEcsError {
    #[error("attempted to cast component from {from:?} to {to:?}")]
    CastingComponents { from: String, to: String },
}
