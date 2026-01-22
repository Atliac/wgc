#[derive(Debug, thiserror::Error)]
pub enum WgcError {
    #[error("Failed to run picker: {0}")]
    CapturePickerFailed(#[from] crate::utils::picker::CapturePickerFailed),
}

pub type Result<T> = std::result::Result<T, WgcError>;
