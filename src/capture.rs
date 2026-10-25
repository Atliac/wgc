use tracing::{debug, trace};
use windows::{
    Foundation::TypedEventHandler,
    Graphics::{
        Capture::{
            Direct3D11CaptureFramePool, GraphicsCaptureDirtyRegionMode, GraphicsCaptureItem,
            GraphicsCaptureSession,
        },
        DirectX::Direct3D11::IDirect3DDevice,
        SizeInt32,
    },
    System::DispatcherQueueController,
    Win32::{
        Foundation::{GetLastError, HMODULE, HWND},
        Graphics::{
            Direct2D::{
                D2D1_DEVICE_CONTEXT_OPTIONS_NONE, D2D1_FACTORY_TYPE_SINGLE_THREADED,
                D2D1CreateFactory, ID2D1Device, ID2D1DeviceContext, ID2D1Factory1,
            },
            Direct3D::{
                D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP, D3D_FEATURE_LEVEL,
                D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0,
                D3D_FEATURE_LEVEL_11_1,
            },
            Direct3D11::*,
            Dxgi::IDXGIDevice,
            Gdi::HMONITOR,
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
/// Capture sessions are created using the builder API via the `Wgc::new_from_*`
/// constructors:
/// - [`Wgc::new_from_item`]
/// - [`Wgc::new_from_hwnd`]
/// - [`Wgc::new_from_monitor`]
/// - [`Wgc::new_from_picker`]
///
/// # Usage Example
/// ```no_run
/// use wgc::*;
///
/// # fn main() -> anyhow::Result<()> {
/// // 1. Create and configure capture session using the builder API
/// let wgc = Wgc::new_from_picker(None)?
///     .frame_queue_length(3)
///     .capture_cursor(false)
///     .build()?;
///
/// // 2. Capture frames (Wgc implements Iterator)
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
/// - Use [`WgcBuilder`] to configure frame buffering, pixel format, and other options.
///
pub struct Wgc {
    _session: GraphicsCaptureSession,
    _control: DispatcherQueueController,
    item: GraphicsCaptureItem,
    frame_pool: Direct3D11CaptureFramePool,
    settings: WgcSettings,
    buffer_size: SizeInt32,
    direct3d_device: IDirect3DDevice,
    d2d1_context: ID2D1DeviceContext,
}

impl Wgc {
    /// Creates a [`WgcBuilder`] from an existing [`GraphicsCaptureItem`].
    ///
    /// # Example
    /// ```no_run
    /// # use wgc::*;
    /// # fn example(item: windows::Graphics::Capture::GraphicsCaptureItem) -> Result<(), WgcError> {
    /// let wgc = Wgc::new_from_item(item)
    ///     .capture_cursor(false)
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_from_item(item: GraphicsCaptureItem) -> WgcBuilder {
        WgcBuilder::new(item)
    }

    /// Creates a [`WgcBuilder`] targeting a window handle (`HWND`).
    ///
    /// # Example
    /// ```no_run
    /// # use wgc::*;
    /// # use windows::Win32::Foundation::HWND;
    /// # fn example(hwnd: HWND) -> Result<(), WgcError> {
    /// let wgc = Wgc::new_from_hwnd(hwnd)?
    ///     .capture_cursor(true)
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_from_hwnd(hwnd: HWND) -> std::result::Result<WgcBuilder, WgcError> {
        WgcBuilder::from_hwnd(hwnd)
    }

    /// Creates a [`WgcBuilder`] targeting a monitor handle (`HMONITOR`).
    ///
    /// # Example
    /// ```no_run
    /// # use wgc::*;
    /// # use windows::Win32::Graphics::Gdi::HMONITOR;
    /// # fn example(monitor: HMONITOR) -> Result<(), WgcError> {
    /// let wgc = Wgc::new_from_monitor(monitor)?
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_from_monitor(monitor: HMONITOR) -> std::result::Result<WgcBuilder, WgcError> {
        WgcBuilder::from_monitor(monitor)
    }

    /// Presents a picker UI and creates a [`WgcBuilder`] for the selected item.
    ///
    /// # Example
    /// ```no_run
    /// # use wgc::*;
    /// # fn example() -> Result<(), WgcError> {
    /// let wgc = Wgc::new_from_picker(None)?
    ///     .frame_queue_length(2)
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_from_picker(
        owner_window: Option<HWND>,
    ) -> std::result::Result<WgcBuilder, WgcError> {
        WgcBuilder::from_picker(owner_window)
    }

    /// Returns a reference to the captured [`GraphicsCaptureItem`].
    pub fn item(&self) -> &GraphicsCaptureItem {
        &self.item
    }

    pub(crate) fn from_builder(builder: WgcBuilder) -> std::result::Result<Self, WgcError> {
        let WgcBuilder { item, settings } = builder;
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
        debug!(
            "Creating frame pool (size: {:?}, queue length: {}, pixel format: {:?})",
            buffer_size, settings.frame_queue_length, settings.pixel_format
        );
        let frame_pool = Direct3D11CaptureFramePool::Create(
            &direct3d_device,
            settings.pixel_format.into(),
            settings.frame_queue_length,
            buffer_size,
        )?;
        // This `no-op` handler is necessary; otherwise, the frame pool will fail to receive updates.
        frame_pool.FrameArrived(
            &TypedEventHandler::<Direct3D11CaptureFramePool, IInspectable>::new(move |_pool, _| {
                trace!(
                    "Frame pool reported an available frame:{:?}",
                    std::thread::current().id()
                );
                Ok(())
            }),
        )?;
        let session = frame_pool.CreateCaptureSession(&item)?;
        debug!("Created capture session");

        if let Some(capture_cursor) = settings.capture_cursor {
            debug!("Setting cursor capture to {}", capture_cursor);
            session.SetIsCursorCaptureEnabled(capture_cursor)?;
        }

        if let Some(include_secondary_windows) = settings.include_secondary_windows {
            debug!(
                "Setting secondary window capture to {}",
                include_secondary_windows
            );
            session.SetIncludeSecondaryWindows(include_secondary_windows)?;
        }

        if let Some(display_border) = settings.display_border {
            debug!("Setting capture border to {}", display_border);
            session.SetIsBorderRequired(display_border)?;
        }

        if let Some(dirty_region_mode) = settings.dirty_region_mode {
            let mode = if dirty_region_mode {
                GraphicsCaptureDirtyRegionMode::ReportAndRender
            } else {
                GraphicsCaptureDirtyRegionMode::ReportOnly
            };
            debug!("Setting dirty region mode to {:?}", mode);
            session.SetDirtyRegionMode(mode)?;
        }

        if let Some(min_update_interval) = settings.min_update_interval {
            debug!(
                "Setting minimum update interval to {:?}",
                min_update_interval
            );
            session.SetMinUpdateInterval(min_update_interval.into())?;
        }

        item.Closed(
            &TypedEventHandler::<GraphicsCaptureItem, IInspectable>::new(move |_item, _| {
                debug!("Capture item closed; stopping capture");
                unsafe { PostQuitMessage(0) };
                Ok(())
            }),
        )?;
        session.StartCapture()?;
        debug!("Capture session started");
        Ok(Self {
            item,
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
            trace!("Waiting for next frame");
            loop {
                let frame = self.frame_pool.TryGetNextFrame();
                if let Ok(frame) = frame {
                    let frame_size = match frame.ContentSize() {
                        Ok(size) => size,
                        Err(err) => return Some(Err(err.into())),
                    };

                    if frame_size != self.buffer_size {
                        debug!(
                            "Capture size changed from {:?} to {:?}; recreating frame pool",
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
                                trace!("Frame pool recreated at new capture size");
                            }
                            Err(err) => return Some(Err(err.into())),
                        }
                    } else {
                        trace!(
                            "Retrieved frame from capture pool:{:?}",
                            std::thread::current().id()
                        );
                        let frame = Frame::new(
                            frame,
                            self.d2d1_context.clone(),
                            self.settings.pixel_format,
                            self.settings,
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
    const FEATURE_LEVELS: [D3D_FEATURE_LEVEL; 4] = [
        D3D_FEATURE_LEVEL_11_1,
        D3D_FEATURE_LEVEL_11_0,
        D3D_FEATURE_LEVEL_10_1,
        D3D_FEATURE_LEVEL_10_0,
    ];

    let mut d3d_device = None;
    let res = unsafe {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            Some(&FEATURE_LEVELS),
            D3D11_SDK_VERSION,
            Some(&mut d3d_device),
            None,
            None,
        )
    };

    if res.is_err() {
        unsafe {
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_WARP,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                Some(&FEATURE_LEVELS),
                D3D11_SDK_VERSION,
                Some(&mut d3d_device),
                None,
                None,
            )?;
        }
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
