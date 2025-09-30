#[cfg(feature = "ssr")]
pub type Result<T> = core::result::Result<T, EmailError>;

#[cfg(feature = "ssr")]
#[derive(Debug, Clone, thiserror::Error)]
pub enum EmailError {
    #[error("{0}")]
    FailedToSend(String),
    #[error("No Email found")]
    NoEmailFound,
}
