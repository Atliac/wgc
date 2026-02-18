use windows::{
    Graphics::Capture::GraphicsCaptureItem,
    Win32::{Foundation::HWND, System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop},
};

use crate::*;

pub fn new_item_from_hwnd(hwnd: HWND) -> std::result::Result<GraphicsCaptureItem, WgcError> {
    let interop = windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()?;
    let item: GraphicsCaptureItem = unsafe { interop.CreateForWindow(hwnd) }?;
    Ok(item)
}
