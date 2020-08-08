#[derive(Debug, thiserror::Error)]
pub enum AdlError {
    #[error(transparent)]
    LibraryError(#[from] libloading::Error),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error)
}

pub type AdlResult<T> = Result<T, AdlError>;
