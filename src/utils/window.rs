use windows::{
    Graphics::Capture::GraphicsCaptureItem,
    Win32::{Foundation::HWND, System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop},
};

use crate::*;

/// Creates a new `GraphicsCaptureItem` from a window handle.
///
/// This function uses the Windows Runtime interop interface to create a graphics capture item
/// that represents the specified window. This is commonly used for screen capture scenarios
/// where you want to capture the contents of a specific application window.
///
/// # Arguments
///
/// * `hwnd` - An `HWND` handle representing the window to create a capture item for.
///
/// # Returns
///
/// * `Ok(GraphicsCaptureItem)` - A successfully created graphics capture item for the window.
/// * `Err(WgcError)` - An error occurred during the creation process, such as:
/// - The interop factory could not be obtained.
/// - The `CreateForWindow` call failed (e.g., invalid window handle).
///
/// # Safety
///
/// This function uses unsafe code to call the `CreateForWindow` method. The caller must ensure
/// that the provided `HWND` handle is valid.
///
/// # Example
///
/// ```ignore
/// use windows::Win32::Foundation::HWND;
/// let hwnd = /* obtain HWND handle */;
/// let capture_item = new_item_from_hwnd(hwnd)?;
/// ```
pub fn new_item_from_hwnd(hwnd: HWND) -> std::result::Result<GraphicsCaptureItem, WgcError> {
    let interop = windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()?;
    let item: GraphicsCaptureItem = unsafe { interop.CreateForWindow(hwnd) }?;
    Ok(item)
}
