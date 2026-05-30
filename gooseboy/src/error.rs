//! This is used to hold the Gooseboy host's errors.
use thiserror::Error;

/// An error returned from the Gooseboy host.
#[derive(Error, Debug)]
pub enum GooseboyError {
    /// The crate is unauthorized.
    #[error("Unauthorized")]
    Unauthorized,
}
