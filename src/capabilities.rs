use crate::*;
use windows::core::*;
use windows::{Foundation::Metadata::ApiInformation, Graphics::Capture::GraphicsCaptureSession};

/// Checks if the Windows Graphics Capture API is supported.
pub fn is_wgc_supported() -> std::result::Result<bool, WgcError> {
    GraphicsCaptureSession::IsSupported().map_err(|e| e.into())
}

/// Checks if the current Windows version supports configuring the capture border
pub fn is_border_configurable() -> std::result::Result<bool, WgcError> {
    ApiInformation::IsPropertyPresent(
        h!("Windows.Graphics.Capture.GraphicsCaptureSession"),
        h!("IsBorderRequired"),
    )
    .map_err(|e| e.into())
}

/// Checks if the cursor capture setting can be configured on the current Windows version.
pub fn is_cursor_configurable() -> std::result::Result<bool, WgcError> {
    ApiInformation::IsPropertyPresent(
        h!("Windows.Graphics.Capture.GraphicsCaptureSession"),
        h!("IsCursorCaptureEnabled"),
    )
    .map_err(|e| e.into())
}

/// Checks if the "dirty region mode" can be configured on the current Windows version.
pub fn is_dirty_region_mode_configurable() -> std::result::Result<bool, WgcError> {
    ApiInformation::IsPropertyPresent(
        h!("Windows.Graphics.Capture.GraphicsCaptureSession"),
        h!("DirtyRegionMode"),
    )
    .map_err(|e| e.into())
}

/// Checks if the "include secondary windows" setting can be configured on the current Windows version.
pub fn is_include_secondary_windows_configurable() -> std::result::Result<bool, WgcError> {
    ApiInformation::IsPropertyPresent(
        h!("Windows.Graphics.Capture.GraphicsCaptureSession"),
        h!("IncludeSecondaryWindows"),
    )
    .map_err(|e| e.into())
}

/// Checks if the "min update interval" setting can be configured on the current Windows version.
pub fn is_min_update_interval_configurable() -> std::result::Result<bool, WgcError> {
    ApiInformation::IsPropertyPresent(
        h!("Windows.Graphics.Capture.GraphicsCaptureSession"),
        h!("MinUpdateInterval"),
    )
    .map_err(|e| e.into())
}
