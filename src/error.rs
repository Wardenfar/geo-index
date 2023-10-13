use std::fmt::Debug;
use thiserror::Error;

/// Enum with all errors in this crate.
#[derive(Error, Debug)]
pub enum FlatbushError {
    #[error("General error: {0}")]
    General(String),
}
