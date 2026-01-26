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
pub fn new_item_with_picker(
    owner_window: Option<HWND>,
) -> std::result::Result<GraphicsCaptureItem, WgcError> {
    let picker = GraphicsCapturePicker::new()?;
    let initialize_with_window: IInitializeWithWindow = picker.cast()?;
    if let Some(owner_window) = owner_window {
        unsafe { initialize_with_window.Initialize(owner_window)? };
    } else {
        let _hidden_window = HiddenWindow::new()?;
        unsafe { initialize_with_window.Initialize(GetForegroundWindow())? };
    }
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
            unsafe { PostQuitMessage(0) };
            debug!("the hidden window for picker is destroyed: {:?}", hwnd);
        }
        _ => {}
    }
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

struct HiddenWindow {
    hwnd: HWND,
}
impl HiddenWindow {
    fn new() -> std::result::Result<Self, WgcError> {
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
                WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
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
        Ok(Self { hwnd })
    }
}
impl Drop for HiddenWindow {
    fn drop(&mut self) {
        unsafe {
            let r = PostMessageW(Some(self.hwnd), WM_DESTROY, WPARAM(0), LPARAM(0));
            if let Err(_e) = r {
                debug!("failed to destroy the hidden window for picker: {:?}", _e);
            }
            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).0 != 0 {
                let _ = TranslateMessage(&msg);
                let _ = DispatchMessageW(&msg);
            }
        };
    }
}
