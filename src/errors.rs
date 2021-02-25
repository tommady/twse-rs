use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum TwseError {
    #[error("Enum data casting failed")]
    EnumCastingError,
    #[error("Error from TWSE state {0}")]
    TWSEError(String),
    #[error("Cannot do ureq get")]
    UreqError(#[from] ureq::Error),
    #[error("Json decode failed")]
    JsonError(#[from] std::io::Error),
}
