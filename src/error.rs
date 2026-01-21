#[derive(Debug, thiserror::Error)]
pub enum WgcError {
    #[error("Failed to create capture item: {0}")]
    Error1(#[from] windows::core::Error),
    #[error("Failed to create capture item: {0}")]
    Error2(#[from] windows::core::Error),
}

pub type Result<T> = std::result::Result<T, WgcError>;
