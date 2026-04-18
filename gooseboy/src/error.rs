use thiserror::Error;

#[derive(Error, Debug)]
pub enum GooseboyError {
    #[error("Unauthorized")]
    Unauthorized,
}
