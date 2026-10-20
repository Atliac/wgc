use std::time::Duration;
use windows::{
    Graphics::Capture::GraphicsCaptureItem,
    Win32::{Foundation::HWND, Graphics::Gdi::HMONITOR},
};

use crate::{
    new_item_from_hwnd, new_item_from_monitor, new_item_with_picker, FrameInterpolationMode,
    PixelFormat, Wgc, WgcError, WgcSettings,
};

/// Fluent builder for configuring and creating a [`Wgc`] capture session.
///
/// Instances of `WgcBuilder` are typically created using one of the `Wgc::new_from_*`
/// constructors:
/// - [`Wgc::new_from_item`]
/// - [`Wgc::new_from_hwnd`]
/// - [`Wgc::new_from_monitor`]
/// - [`Wgc::new_from_picker`]
///
/// # Example
///
/// ```no_run
/// use wgc::Wgc;
///
/// # fn main() -> anyhow::Result<()> {
/// let wgc = Wgc::new_from_picker(None)?
///     .capture_cursor(false)
///     .display_border(true)
///     .frame_queue_length(2)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct WgcBuilder {
    pub(crate) item: GraphicsCaptureItem,
    pub(crate) settings: WgcSettings,
}

impl WgcBuilder {
    /// Creates a new builder for the given [`GraphicsCaptureItem`] with default settings.
    pub fn new(item: GraphicsCaptureItem) -> Self {
        Self {
            item,
            settings: WgcSettings::default(),
        }
    }

    /// Creates a builder targeting a window handle (`HWND`).
    pub fn from_hwnd(hwnd: HWND) -> Result<Self, WgcError> {
        let item = new_item_from_hwnd(hwnd)?;
        Ok(Self::new(item))
    }

    /// Creates a builder targeting a monitor handle (`HMONITOR`).
    pub fn from_monitor(monitor: HMONITOR) -> Result<Self, WgcError> {
        let item = new_item_from_monitor(monitor)?;
        Ok(Self::new(item))
    }

    /// Presents a picker UI and creates a builder for the selected item.
    pub fn from_picker(owner_window: Option<HWND>) -> Result<Self, WgcError> {
        let item = new_item_with_picker(owner_window)?;
        Ok(Self::new(item))
    }

    /// Sets the pixel format for the captured frames.
    ///
    /// Defaults to [`PixelFormat::RGBA8`].
    pub fn pixel_format(mut self, pixel_format: PixelFormat) -> Self {
        self.settings.pixel_format = pixel_format;
        self
    }

    /// Sets the number of frames to queue in the frame pool.
    ///
    /// Defaults to `1`. Must be greater than 0.
    pub fn frame_queue_length(mut self, frame_queue_length: i32) -> Self {
        self.settings.frame_queue_length = frame_queue_length;
        self
    }

    /// Sets whether to capture the mouse cursor.
    pub fn capture_cursor(mut self, capture_cursor: bool) -> Self {
        self.settings.capture_cursor = Some(capture_cursor);
        self
    }

    /// Sets whether to show a yellow border around the captured area.
    pub fn display_border(mut self, display_border: bool) -> Self {
        self.settings.display_border = Some(display_border);
        self
    }

    /// Sets whether to capture secondary / child windows and popups.
    pub fn include_secondary_windows(mut self, include_secondary_windows: bool) -> Self {
        self.settings.include_secondary_windows = Some(include_secondary_windows);
        self
    }

    /// Sets the dirty region mode for capture optimization.
    ///
    /// - `true`: Enables `ReportAndRender` mode (render changed areas only).
    /// - `false`: Explicitly sets `ReportOnly` mode (full frame rendered + dirty regions reported).
    pub fn dirty_region_mode(mut self, dirty_region_mode: bool) -> Self {
        self.settings.dirty_region_mode = Some(dirty_region_mode);
        self
    }

    /// Sets the minimum update interval to throttle capture rate.
    pub fn min_update_interval(mut self, min_update_interval: Duration) -> Self {
        self.settings.min_update_interval = Some(min_update_interval);
        self
    }

    /// Sets the interpolation mode used for scaling frames.
    ///
    /// Defaults to [`FrameInterpolationMode::Linear`].
    pub fn frame_interpolation_mode(
        mut self,
        frame_interpolation_mode: FrameInterpolationMode,
    ) -> Self {
        self.settings.frame_interpolation_mode = frame_interpolation_mode;
        self
    }

    /// Returns a reference to the target [`GraphicsCaptureItem`].
    pub fn item(&self) -> &GraphicsCaptureItem {
        &self.item
    }

    /// Builds and starts the capture session.
    pub fn build(self) -> Result<Wgc, WgcError> {
        Wgc::from_builder(self)
    }
}

