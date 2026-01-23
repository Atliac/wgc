use crate::*;
use windows::{
    Graphics::Capture::*,
    Win32::{
        Foundation::*,
        System::LibraryLoader::GetModuleHandleW,
        UI::{Shell::*, WindowsAndMessaging::*},
    },
    core::*,
};
use windows_future::AsyncStatus;
#[derive(Debug, thiserror::Error)]
pub enum CaptureItemPickerError {
    #[error("Failed to run picker: {0}")]
    WindowsError(#[from] windows::core::Error),
    #[error("No item selected")]
    NoItemSelected,
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn new_item_with_picker() -> std::result::Result<GraphicsCaptureItem, CaptureItemPickerError> {
    debug!("Starting picker");
    let picker_window = create_a_hidden_window()?;
    let picker = GraphicsCapturePicker::new()?;
    let initialize_with_window: IInitializeWithWindow = picker.cast()?;
    unsafe { initialize_with_window.Initialize(picker_window)? };
    let op = picker.PickSingleItemAsync()?;
    let mut msg = MSG::default();
    unsafe {
        loop {
            match GetMessageW(&mut msg, None, 0, 0).0 {
                -1 => {
                    let e: windows::core::Error = GetLastError().to_hresult().into();
                    return Err(e.into());
                }
                0 => break,
                _ => {
                    let _ = TranslateMessage(&msg);
                    let _ = DispatchMessageW(&msg);
                    if op.Status()? != AsyncStatus::Started {
                        break;
                    }
                }
            }
        }
    }
    op.GetResults().map_err(|e| {
        if e.code() == S_OK {
            CaptureItemPickerError::NoItemSelected
        } else {
            e.into()
        }
    })
}

fn create_a_hidden_window() -> std::result::Result<HWND, CaptureItemPickerError> {
    let instance = unsafe { GetModuleHandleW(None)? };
    let window_class = w!("Rust Crate wgc(Window.Graphics.Capture) Picker");
    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wnd_proc),
        hInstance: instance.into(),
        lpszClassName: window_class,
        ..Default::default()
    };
    unsafe {
        if RegisterClassExW(&wc) == 0 {
            let err = GetLastError();
            if err != ERROR_CLASS_ALREADY_EXISTS {
                err.ok()?;
            }
        }
    }

    let hwnd = unsafe {
        CreateWindowExW(
            WS_EX_TOOLWINDOW,
            window_class,
            w!("Rust Crate wgc(Window.Graphics.Capture) Picker"),
            WS_POPUP | WS_VISIBLE,
            0,
            0,
            0,
            0,
            None,
            None,
            Some(instance.into()),
            None,
        )?
    };
    Ok(hwnd)
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}
