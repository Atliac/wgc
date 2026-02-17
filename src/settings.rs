use windows::{Graphics::DirectX::DirectXPixelFormat, Win32::Graphics::Dxgi::Common::DXGI_FORMAT};

#[derive(smart_default::SmartDefault, Debug, Clone, Copy)]
pub struct WgcSettings {
    #[default(PixelFormat::RGBA8)]
    pub pixel_format: PixelFormat,
    #[default(1)]
    pub frame_queue_length: i32,
}

/// The pixel format of the captured frames.
#[derive(Debug, Clone, Copy)]
pub struct PixelFormat {
    pub(crate) format: DirectXPixelFormat,
    pub(crate) bytes_per_pixel: u32,
}

impl PixelFormat {
    /// create a custom pixel format
    ///
    /// # example
    /// ```
    /// use wgc::PixelFormat;
    /// let pixel_format = PixelFormat::new(windows::Graphics::DirectX::DirectXPixelFormat::R8G8B8A8UIntNormalized, 4);
    /// ```
    pub fn new(format: DirectXPixelFormat, bytes_per_pixel: u32) -> Self {
        Self {
            format,
            bytes_per_pixel,
        }
    }
    pub const RGBA8: Self = Self {
        format: DirectXPixelFormat::R8G8B8A8UIntNormalized,
        bytes_per_pixel: 4,
    };
    pub const BGRA8: Self = Self {
        format: DirectXPixelFormat::B8G8R8A8UIntNormalized,
        bytes_per_pixel: 4,
    };
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
