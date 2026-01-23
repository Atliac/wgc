use windows::{
    Foundation::TypedEventHandler,
    Graphics::{
        Capture::{Direct3D11CaptureFramePool, GraphicsCaptureItem, GraphicsCaptureSession},
        DirectX::Direct3D11::IDirect3DDevice,
        SizeInt32,
    },
    System::DispatcherQueueController,
    Win32::{
        Foundation::{GetLastError, HMODULE},
        Graphics::{
            Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_11_1},
            Direct3D11::*,
            Dxgi::IDXGIDevice,
        },
        System::WinRT::{
            CreateDispatcherQueueController, DQTAT_COM_NONE, DQTYPE_THREAD_CURRENT,
            Direct3D11::CreateDirect3D11DeviceFromDXGIDevice, DispatcherQueueOptions,
        },
        UI::WindowsAndMessaging::{
            DispatchMessageW, GetMessageW, MSG, PostQuitMessage, TranslateMessage,
        },
    },
    core::*,
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum WgcError {
    #[error("Wgc: {0}")]
    WindowsError(#[from] windows::core::Error),
    #[error("Wgc: {0}")]
    D3DCreateDeviceError(String),
    #[error("Wgc: Frame queue length must be greater than 0, got {0}")]
    InvalidFrameQueueLength(i32),
}

pub struct Wgc {
    _session: GraphicsCaptureSession,
    _control: DispatcherQueueController,
    _item: GraphicsCaptureItem,
    frame_pool: Direct3D11CaptureFramePool,
    settings: WgcSettings,
    buffer_size: SizeInt32,
    direct3d_device: IDirect3DDevice,
}

impl Wgc {
    pub fn new(
        item: GraphicsCaptureItem,
        settings: WgcSettings,
    ) -> std::result::Result<Self, WgcError> {
        let d3d_device = create_d3d_device()?;
        let dxgi_device: IDXGIDevice = d3d_device.cast()?;
        let direct3d_device: IDirect3DDevice =
            unsafe { CreateDirect3D11DeviceFromDXGIDevice(&dxgi_device)?.cast()? };
        if settings.frame_queue_length <= 0 {
            return Err(WgcError::InvalidFrameQueueLength(
                settings.frame_queue_length,
            ));
        }
        let control = create_dispatcher_queue_controller()?;
        let buffer_size = item.Size()?;
        let frame_pool = Direct3D11CaptureFramePool::Create(
            &direct3d_device,
            settings.pixel_format,
            settings.frame_queue_length,
            buffer_size,
        )?;
        // This `no-op` handler is necessary; otherwise, the frame pool will fail to receive updates.
        frame_pool.FrameArrived(
            &TypedEventHandler::<Direct3D11CaptureFramePool, IInspectable>::new(move |_pool, _| {
                Ok(())
            }),
        )?;
        let session = frame_pool.CreateCaptureSession(&item)?;

        item.Closed(
            &TypedEventHandler::<GraphicsCaptureItem, IInspectable>::new(move |_item, _| {
                unsafe { PostQuitMessage(0) };
                Ok(())
            }),
        )?;
        session.StartCapture()?;
        Ok(Self {
            _item: item,
            _session: session,
            _control: control,
            frame_pool,
            settings,
            direct3d_device,
            buffer_size,
        })
    }
}

impl Iterator for Wgc {
    type Item = std::result::Result<Frame, WgcError>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut msg = MSG::default();
        unsafe {
            loop {
                let frame = self.frame_pool.TryGetNextFrame();
                if let Ok(frame) = frame {
                    let frame_size = match frame.ContentSize() {
                        Ok(size) => size,
                        Err(err) => return Some(Err(err.into())),
                    };

                    if frame_size != self.buffer_size {
                        match self.frame_pool.Recreate(
                            &self.direct3d_device,
                            self.settings.pixel_format,
                            self.settings.frame_queue_length,
                            frame_size,
                        ) {
                            Ok(_) => {
                                self.buffer_size = frame_size;
                            }
                            Err(err) => return Some(Err(err.into())),
                        }
                    } else {
                        return Some(Ok(Frame::new(frame)));
                    }
                }
                match GetMessageW(&mut msg, None, 0, 0).0 {
                    -1 => {
                        let e: windows::core::Error = GetLastError().to_hresult().into();
                        return Some(Err(e.into()));
                    }
                    0 => return None,
                    _ => {
                        let _ = TranslateMessage(&msg);
                        let _ = DispatchMessageW(&msg);
                    }
                }
            }
        }
    }
}

fn create_d3d_device() -> std::result::Result<ID3D11Device, WgcError> {
    let mut d3d_device = None;
    unsafe {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            Some(&[D3D_FEATURE_LEVEL_11_1]),
            D3D11_SDK_VERSION,
            Some(&mut d3d_device),
            None,
            None,
        )?;
    }
    d3d_device
        .ok_or_else(|| WgcError::D3DCreateDeviceError("Failed to create D3D device".to_string()))
}

fn create_dispatcher_queue_controller() -> std::result::Result<DispatcherQueueController, WgcError>
{
    let options = DispatcherQueueOptions {
        dwSize: std::mem::size_of::<DispatcherQueueOptions>() as u32,
        threadType: DQTYPE_THREAD_CURRENT,
        apartmentType: DQTAT_COM_NONE,
    };
    let control = unsafe { CreateDispatcherQueueController(options)? };
    Ok(control)
}
