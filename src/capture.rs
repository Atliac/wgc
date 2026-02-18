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
            Direct2D::{
                D2D1_DEVICE_CONTEXT_OPTIONS_NONE, D2D1_FACTORY_TYPE_SINGLE_THREADED,
                D2D1CreateFactory, ID2D1Device, ID2D1DeviceContext, ID2D1Factory1,
            },
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

/// The main entry point for Windows Graphics Capture (WGC) functionality.
///
/// `Wgc` represents a capture session that streams frames from a capture source
/// (window, monitor, or user-selected item). It implements [`Iterator`] to yield
/// captured frames one by one.
///
/// # Usage Example
/// ```ignore
/// use wgc::*;
///
/// # fn main() -> anyhow::Result<()> {
/// // 1. Create a capture item (e.g., from a window handle or monitor)
/// let hwnd = /* obtain window handle */;
/// let item = wgc::new_item_from_hwnd(hwnd)?;
///
/// // 2. Configure capture settings
/// let settings = WgcSettings {
///     frame_queue_length: 3,
///     ..Default::default()
/// };
///
/// // 3. Initialize the capture session
/// let wgc = Wgc::new(item, settings)?;
///
/// // 4. Capture frames (Wgc implements Iterator)
/// for frame_result in wgc.take(10) {
///     let frame = frame_result?;
///     // Process the frame...
///     println!("Frame captured at {:?}", frame.render_time()?);
/// }
/// # Ok(())
/// # }
/// ```
///
/// # Important Notes
/// - The `Wgc` struct owns all capture resources and must be kept alive while capturing.
/// - It implements [`Iterator`] with [`Frame`] as the item type (wrapped in `Result`).
/// - The iterator will block until a frame is available or an error occurs.
/// - Use [`WgcSettings`] to configure frame buffering, pixel format, and other options.
///
pub struct Wgc {
    _session: GraphicsCaptureSession,
    _control: DispatcherQueueController,
    _item: GraphicsCaptureItem,
    frame_pool: Direct3D11CaptureFramePool,
    settings: WgcSettings,
    buffer_size: SizeInt32,
    direct3d_device: IDirect3DDevice,
    d2d1_context: ID2D1DeviceContext,
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
        let d2d1_factory = create_d2d_factory()?;
        let d2d1_device: ID2D1Device = unsafe { d2d1_factory.CreateDevice(&dxgi_device) }?;
        let d2d1_context: ID2D1DeviceContext =
            unsafe { d2d1_device.CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE) }?;
        assert!(
            settings.frame_queue_length > 0,
            "Frame queue length must be greater than 0"
        );
        let control = create_dispatcher_queue_controller()?;
        let buffer_size = item.Size()?;
        let frame_pool = Direct3D11CaptureFramePool::Create(
            &direct3d_device,
            settings.pixel_format.into(),
            settings.frame_queue_length,
            buffer_size,
        )?;
        // This `no-op` handler is necessary; otherwise, the frame pool will fail to receive updates.
        frame_pool.FrameArrived(
            &TypedEventHandler::<Direct3D11CaptureFramePool, IInspectable>::new(move |_pool, _| {
                trace!("Frame arrived");
                Ok(())
            }),
        )?;
        let session = frame_pool.CreateCaptureSession(&item)?;

        if let Some(capture_cursor) = settings.capture_cursor {
            session.SetIsCursorCaptureEnabled(capture_cursor)?;
        }

        if let Some(include_secondary_windows) = settings.include_secondary_windows {
            session.SetIncludeSecondaryWindows(include_secondary_windows)?;
        }

        if let Some(display_border) = settings.display_border {
            session.SetIsBorderRequired(display_border)?;
        }

        if let Some(_dirty_region_mode) = settings.dirty_region_mode {
            unimplemented!("dirty_region_mode is not yet implemented");
        }

        if let Some(min_update_interval) = settings.min_update_interval {
            session.SetMinUpdateInterval(min_update_interval.into())?;
        }

        item.Closed(
            &TypedEventHandler::<GraphicsCaptureItem, IInspectable>::new(move |_item, _| {
                debug!("Item closed, stopping capture");
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
            d2d1_context,
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
                        trace!(
                            "Frame dropped as buffer size changed from {:?} to {:?}",
                            self.buffer_size, frame_size
                        );
                        match self.frame_pool.Recreate(
                            &self.direct3d_device,
                            self.settings.pixel_format.into(),
                            self.settings.frame_queue_length,
                            frame_size,
                        ) {
                            Ok(_) => {
                                self.buffer_size = frame_size;
                            }
                            Err(err) => return Some(Err(err.into())),
                        }
                    } else {
                        trace!("Got frame");
                        let frame = Frame::new(
                            frame,
                            self.d2d1_context.clone(),
                            self.settings.pixel_format,
                        );
                        return Some(Ok(frame));
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
    Ok(d3d_device.unwrap())
}

fn create_d2d_factory() -> std::result::Result<ID2D1Factory1, WgcError> {
    unsafe {
        D2D1CreateFactory::<ID2D1Factory1>(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)
            .map_err(|e| e.into())
    }
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
