use windows::{Graphics::DirectX::DirectXPixelFormat, Win32::Graphics::Dxgi::Common::DXGI_FORMAT};

#[derive(smart_default::SmartDefault, Debug, Clone, Copy)]
pub struct WgcSettings {
    #[default(PixelFormat::RGBA8)]
    pub pixel_format: PixelFormat,
    #[default(1)]
    pub frame_queue_length: i32,
}

/// Specifies the pixel format for capture output.
///
/// Contains the underlying DirectX pixel format and the number of bytes per pixel.
/// Use the predefined constants [`PixelFormat::RGBA8`] or [`PixelFormat::BGRA8`]
/// for common formats, or create a custom format with [`PixelFormat::new`].
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
