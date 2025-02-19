use thiserror::Error;

#[derive(Debug, Error)]
pub enum CheckerError {
    #[error("polars error")]
    PolarsError(#[from] polars::error::PolarsError),
    #[error("other error")]
    OtherError(#[from] anyhow::Error),
}
