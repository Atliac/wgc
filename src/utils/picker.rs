use windows::{
    Graphics::Capture::*,
    Win32::{
        Foundation::*,
        System::LibraryLoader::GetModuleHandleW,
        UI::{Shell::*, WindowsAndMessaging::*},
    },
    core::*,
};
#[derive(Debug, thiserror::Error)]
pub enum CapturePickerFailed {
    #[error("Failed to run picker: {0}")]
    CapturePickerFailed(#[from] windows::core::Error),
}
pub async fn capture_picker() -> std::result::Result<GraphicsCaptureItem, CapturePickerFailed> {
    let picker_window = PickerWindow::new()?;
    let picker = GraphicsCapturePicker::new()?;
    // Initialize the picker with our hidden window.
    let initialize_with_window: IInitializeWithWindow = picker.cast()?;
    unsafe { initialize_with_window.Initialize(picker_window.hwnd)? };

    Ok(picker.PickSingleItemAsync()?.await?)
}

/// A hidden window that is used to initialize the picker.
struct PickerWindow {
    hwnd: HWND,
}
impl PickerWindow {
    fn new() -> std::result::Result<Self, CapturePickerFailed> {
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
        Ok(Self { hwnd })
    }
}
impl Drop for PickerWindow {
    fn drop(&mut self) {
        unsafe {
            let _ = DestroyWindow(self.hwnd);
        }
    }
}

/// Minimal Window Procedure for our hidden owner window.
unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}
