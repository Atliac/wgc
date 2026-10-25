use std::time::Duration;
use windows::{Graphics::DirectX::DirectXPixelFormat, Win32::Graphics::Dxgi::Common::DXGI_FORMAT};

/// Internal configuration settings for Windows Graphics Capture (WGC).
#[derive(smart_default::SmartDefault, Debug, Clone, Copy)]
pub(crate) struct WgcSettings {
    /// The pixel format for capture output.
    ///
    /// Defaults to [`PixelFormat::RGBA8`].
    #[default(PixelFormat::RGBA8)]
    pub pixel_format: PixelFormat,
    /// The number of frames to queue for capture.
    ///
    /// Controls the buffer depth for frame capture. Higher values may increase latency
    /// but can help prevent frame drops.
    ///
    /// Defaults to `1`.
    #[default(1)]
    pub frame_queue_length: i32,
    /// Whether to capture the cursor in the output.
    ///
    /// Set this field to `Some(true)` to include the cursor, or `Some(false)` to exclude it.
    /// Leave as `None` to use the system default behavior.
    ///
    /// **Note:** This field can only be set to `Some(...)` if
    /// [`is_cursor_configurable()`](crate::capabilities::is_cursor_configurable) returns `true`.
    #[default(None)]
    pub capture_cursor: Option<bool>,
    /// Whether to display the capture border around the captured window.
    ///
    /// Set this field to `Some(true)` to show the border, or `Some(false)` to hide it.
    /// Leave as `None` to use the system default behavior.
    ///
    /// **Note:** This field can only be set to `Some(...)` if
    /// [`is_border_configurable()`](crate::capabilities::is_border_configurable) returns `true`.
    #[default(None)]
    pub display_border: Option<bool>,
    /// Whether to include secondary windows in the capture.
    ///
    /// Set this field to `Some(true)` to include child/pop-up windows, or `Some(false)`
    /// to capture only the main window. Leave as `None` to use the system default behavior.
    ///
    /// **Note:** This field can only be set to `Some(...)` if
    /// [`is_include_secondary_windows_configurable()`](crate::capabilities::is_include_secondary_windows_configurable)
    /// returns `true`.
    #[default(None)]
    pub include_secondary_windows: Option<bool>,
    /// The dirty region mode for capture optimization.
    ///
    /// On supported Windows versions, dirty regions are always reported automatically by default
    /// via [`Frame::dirty_regions`](crate::frame::Frame::dirty_regions) while rendering the full
    /// frame (`ReportOnly`). You only need to touch this setting if you want `ReportAndRender`
    /// mode (`Some(true)`), where DWM only renders the changed areas to save GPU bandwidth.
    ///
    /// - `None` (default): Uses the system default behavior (`ReportOnly` on supported systems).
    /// - `Some(false)`: Explicitly sets `ReportOnly` mode (full frame rendered + dirty regions reported).
    /// - `Some(true)`: Enables `ReportAndRender` mode (only changed pixels are rendered + dirty regions reported).
    ///
    /// **Note:** This field can only be set to `Some(...)` if
    /// [`is_dirty_region_mode_configurable()`](crate::capabilities::is_dirty_region_mode_configurable)
    /// returns `true`.
    #[default(None)]
    pub dirty_region_mode: Option<bool>,
    /// The minimum update interval for frame captures.
    ///
    /// Set this field to `Some(duration)` to throttle the capture rate, or `None` to
    /// capture as fast as possible.
    ///
    /// **Note:** This field can only be set to `Some(...)` if
    /// [`is_min_update_interval_configurable()`](crate::capabilities::is_min_update_interval_configurable)
    /// returns `true`.
    #[default(None)]
    pub min_update_interval: Option<Duration>,
    /// The interpolation mode used for scaling frames.
    ///
    /// This setting controls how frames are scaled when the capture size differs from
    /// the desired output size. Different methods offer trade-offs between performance
    /// and visual quality.
    ///
    /// Defaults to [`FrameInterpolationMode::Linear`].
    #[default(FrameInterpolationMode::Linear)]
    pub frame_interpolation_mode: FrameInterpolationMode,
}

/// Specifies the interpolation method used for scaling frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameInterpolationMode {
    /// Samples the nearest single point and uses that exact color.
    ///
    /// This mode requires the least processing time but produces the lowest quality image.
    /// It is best suited for scenarios where performance is critical and visual fidelity is secondary.
    NearestNeighbor,

    /// Uses a four-point sample with linear interpolation.
    ///
    /// This mode requires more processing time than [`Self::NearestNeighbor`] but produces
    /// a higher quality image by smoothing transitions between pixels.
    Linear,

    /// Uses a 16-sample cubic kernel for interpolation.
    ///
    /// This mode requires the most processing time among the standard interpolation methods
    /// but produces a higher quality image with smoother curves and better detail preservation
    /// compared to [`Self::Linear`].
    Cubic,

    /// Uses four linear samples within a single pixel to provide effective edge anti-aliasing.
    ///
    /// This mode is particularly effective when scaling down by small factors on images
    /// with low resolution or few pixels, helping to reduce jagged edges.
    MultiSampleLinear,

    /// Uses a variable-size, high-quality cubic kernel to pre-downscale the image if
    /// downscaling is involved in the transform matrix, followed by cubic interpolation
    /// for the final output.
    ///
    /// This mode offers the highest visual quality for complex scaling operations, especially
    /// when significant downscaling is required, at the cost of increased processing time.
    HighQualityCubic,
}

impl From<FrameInterpolationMode> for windows::Win32::Graphics::Direct2D::D2D1_INTERPOLATION_MODE {
    fn from(value: FrameInterpolationMode) -> Self {
        use windows::Win32::Graphics::Direct2D::*;
        match value {
            FrameInterpolationMode::NearestNeighbor => D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
            FrameInterpolationMode::Linear => D2D1_INTERPOLATION_MODE_LINEAR,
            FrameInterpolationMode::Cubic => D2D1_INTERPOLATION_MODE_CUBIC,
            FrameInterpolationMode::MultiSampleLinear => {
                D2D1_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR
            }
            FrameInterpolationMode::HighQualityCubic => D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC,
        }
    }
}

/// Specifies the pixel format for capture output.
///
/// Contains the underlying DirectX pixel format and the number of bytes per pixel.
/// Use the predefined constants [`PixelFormat::RGBA8`] or [`PixelFormat::BGRA8`]
/// for common formats, or create a custom format with [`PixelFormat::new`].
///
/// Buffers returned by [`Frame::pixels`](crate::Frame::pixels) and
/// [`Frame::pixels_fitted`](crate::Frame::pixels_fitted) use premultiplied
/// alpha for formats that contain an alpha channel. This is a Direct2D constraint, not an
/// arbitrary choice: for 32-bit formats such as `B8G8R8A8_UNORM` and `R8G8B8A8_UNORM`,
/// bitmaps and render targets only support premultiplied, ignore, or unknown alpha modes.
/// `D2D1_ALPHA_MODE_STRAIGHT` is valid only for `A8_UNORM`, so straight-alpha output cannot
/// be produced for these formats.
#[derive(Debug, Clone, Copy)]
pub struct PixelFormat {
    /// The underlying DirectX pixel format.
    format: DirectXPixelFormat,
    /// The number of bytes required to store a single pixel.
    bytes_per_pixel: u32,
}

impl PixelFormat {
    /// Creates a new custom pixel format.
    ///
    /// # Arguments
    ///
    /// * `format` - The underlying DirectX pixel format.
    /// * `bytes_per_pixel` - The number of bytes required to store a single pixel.
    ///
    /// # Example
    ///
    /// ```
    /// use wgc::settings::PixelFormat;
    /// use windows::Graphics::DirectX::DirectXPixelFormat;
    ///
    /// let custom_format = PixelFormat::new(
    ///     DirectXPixelFormat::R8G8B8A8UIntNormalized,
    ///     4,
    /// );
    /// ```
    pub fn new(format: DirectXPixelFormat, bytes_per_pixel: u32) -> Self {
        Self {
            format,
            bytes_per_pixel,
        }
    }

    /// RGBA 8-bit per channel format (32 bits per pixel).
    ///
    /// Each pixel consists of 4 bytes: Red, Green, Blue, and Alpha, in that order.
    /// This is a common format for capture output.
    pub const RGBA8: Self = Self {
        format: DirectXPixelFormat::R8G8B8A8UIntNormalized,
        bytes_per_pixel: 4,
    };

    /// BGRA 8-bit per channel format (32 bits per pixel).
    ///
    /// Each pixel consists of 4 bytes: Blue, Green, Red, and Alpha, in that order.
    /// This format is commonly used in Windows GDI and DirectComposition.
    pub const BGRA8: Self = Self {
        format: DirectXPixelFormat::B8G8R8A8UIntNormalized,
        bytes_per_pixel: 4,
    };

    pub fn format(&self) -> DirectXPixelFormat {
        self.format
    }
    pub fn bytes_per_pixel(&self) -> u32 {
        self.bytes_per_pixel
    }
}

impl From<PixelFormat> for DirectXPixelFormat {
    fn from(pixel_format: PixelFormat) -> Self {
        pixel_format.format
    }
}

impl From<PixelFormat> for DXGI_FORMAT {
    fn from(pixel_format: PixelFormat) -> Self {
        Self(pixel_format.format.0)
    }
}
