//! Custom errors.
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChabloError {
    #[error("Failed to get response: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Failed to render template")]
    AskamaError(#[from] askama::Error),
    #[error("Failed to read glob pattern: {0}")]
    GlobError(#[from] glob::PatternError),
    #[error("Failed to match the pattern")]
    MatchError,
    #[error("Failed to parse Int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}
