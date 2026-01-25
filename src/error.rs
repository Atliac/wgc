#[derive(Debug, thiserror::Error)]
pub enum WgcError {
    #[error("Wgc: {0}")]
    WindowsError(#[from] windows::core::Error),
    #[error("No item selected")]
    NoItemSelected,
}
