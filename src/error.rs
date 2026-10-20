#[derive(Debug, thiserror::Error, Clone)]
pub enum WgcError {
    #[error("Wgc: {0}")]
    WindowsError(#[from] windows::core::Error),
    #[error("No item selected")]
    NoItemSelected,
    #[error(
        "No foreground window found. The picker requires an interactive desktop session and cannot run in a headless session."
    )]
    NoForegroundWindow,
}
