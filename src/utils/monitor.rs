use windows::{
    Graphics::Capture::GraphicsCaptureItem,
    Win32::{
        Graphics::Gdi::HMONITOR, System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop,
    },
};

use crate::*;

/// Creates a new `GraphicsCaptureItem` from a monitor handle.
///
/// This function uses the Windows Runtime interop interface to create a graphics capture item
/// that represents the specified monitor. This is commonly used for screen capture scenarios
/// where you want to capture the entire contents of a specific monitor.
///
/// # Arguments
///
/// * `monitor` - An `HMONITOR` handle representing the monitor to create a capture item for.
///
/// # Returns
///
/// * `Ok(GraphicsCaptureItem)` - A successfully created graphics capture item for the monitor.
/// * `Err(WgcError)` - An error occurred during the creation process, such as:
///   - The interop factory could not be obtained.
///   - The `CreateForMonitor` call failed (e.g., invalid monitor handle).
///
/// # Safety
///
/// This function uses unsafe code to call the `CreateForMonitor` method. The caller must ensure
/// that the provided `HMONITOR` handle is valid.
///
/// # Example
///
/// ```ignore
/// use windows::Win32::Graphics::Gdi::HMONITOR;
/// let monitor = /* obtain HMONITOR handle */;
/// let capture_item = new_item_from_monitor(monitor)?;
/// ```
pub fn new_item_from_monitor(
    monitor: HMONITOR,
) -> std::result::Result<GraphicsCaptureItem, WgcError> {
    let interop = windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()?;
    let item: GraphicsCaptureItem = unsafe { interop.CreateForMonitor(monitor) }?;
    Ok(item)
}
