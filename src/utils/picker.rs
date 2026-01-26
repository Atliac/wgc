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

#[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", skip_all))]
pub fn new_item_with_picker() -> std::result::Result<GraphicsCaptureItem, WgcError> {
    let picker_window = PickerWindow::new()?;
    let picker = GraphicsCapturePicker::new()?;
    let initialize_with_window: IInitializeWithWindow = picker.cast()?;
    unsafe { initialize_with_window.Initialize(picker_window.hwnd)? };
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
            debug!("No item selected");
            WgcError::NoItemSelected
        } else {
            e.into()
        }
    })
}

fn create_a_hidden_window() -> std::result::Result<HWND, WgcError> {
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
    match msg {
        WM_CREATE => {
            debug!("a hidden window for picker is created: {:?}", hwnd);
        }
        WM_DESTROY => {
            debug!("the hidden window for picker is destroyed: {:?}", hwnd);
        }
        _ => {}
    }
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

struct PickerWindow {
    hwnd: HWND,
}
impl PickerWindow {
    fn new() -> std::result::Result<Self, WgcError> {
        let hwnd = create_a_hidden_window()?;
        Ok(Self { hwnd })
    }
}
impl Drop for PickerWindow {
    fn drop(&mut self) {
        unsafe {
            let r = PostMessageW(Some(self.hwnd), WM_DESTROY, WPARAM(0), LPARAM(0));
            debug!("destroying the hidden window for picker: {:?}", self.hwnd);
            if let Err(_e) = r {
                debug!("failed to destroy the hidden window for picker: {:?}", _e);
            }
            let mut msg = MSG::default();
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).0 != 0 {
                let _ = TranslateMessage(&msg);
                let _ = DispatchMessageW(&msg);
            }
        };
    }
}
