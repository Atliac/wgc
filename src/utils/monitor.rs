use windows::{
    Graphics::Capture::GraphicsCaptureItem,
    Win32::{
        Graphics::Gdi::HMONITOR, System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop,
    },
};

use crate::*;

pub fn new_item_from_monitor(
    monitor: HMONITOR,
) -> std::result::Result<GraphicsCaptureItem, WgcError> {
    let interop = windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()?;
    let item: GraphicsCaptureItem = unsafe { interop.CreateForMonitor(monitor) }?;
    Ok(item)
}
