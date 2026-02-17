use std::time::{Duration, Instant};

use crate::*;
use windows::{
    Graphics::{Capture::Direct3D11CaptureFrame, SizeInt32},
    Win32::{
        Graphics::{
            Direct2D::{
                Common::{D2D_POINT_2U, D2D_SIZE_U, D2D1_PIXEL_FORMAT},
                D2D1_BITMAP_PROPERTIES1, D2D1_DEVICE_CONTEXT_OPTIONS_NONE,
                D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_MAPPED_RECT, D2D1CreateFactory,
                ID2D1Bitmap1, ID2D1Device, ID2D1DeviceContext, ID2D1Factory1,
            },
            Dxgi::IDXGISurface,
        },
        System::WinRT::Direct3D11::IDirect3DDxgiInterfaceAccess,
    },
    core::Interface,
};
#[derive(Debug)]
pub struct Frame {
    frame: Direct3D11CaptureFrame,
    d2d1_context: ID2D1DeviceContext,
    pixel_format: PixelFormat,
}
impl Frame {
    pub fn new(
        frame: Direct3D11CaptureFrame,
        d2d1_context: ID2D1DeviceContext,
        pixel_format: PixelFormat,
    ) -> Self {
        Self {
            frame,
            d2d1_context,
            pixel_format,
        }
    }

    /// Returns the time at which the frame was rendered
    pub fn render_time(&self) -> std::result::Result<Instant, WgcError> {
        let frame_delay_since_boot =
            Duration::from_nanos(self.frame.SystemRelativeTime()?.Duration as u64 * 100);
        let system_uptime = elapsed_since_system_boot();
        Ok(Instant::now() - system_uptime + frame_delay_since_boot)
    }

    /// Returns the size of the frame
    pub fn size(&self) -> std::result::Result<FrameSize, WgcError> {
        let size = self.frame.ContentSize()?;
        Ok(size.into())
    }

    pub fn read_pixels(&self, desired_size: FrameSize) -> std::result::Result<Vec<u8>, WgcError> {
        let mut buffer = vec![
            0;
            (desired_size.width * desired_size.height * self.pixel_format.bytes_per_pixel())
                as usize
        ];

        let frame_size = self.size()?;
        if frame_size != desired_size {
            let size = D2D_SIZE_U {
                width: desired_size.width,
                height: desired_size.height,
            };
            let bitmap_properties = D2D1_BITMAP_PROPERTIES1 {
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: self.pixel_format.into(),
                    alphaMode:
                        windows::Win32::Graphics::Direct2D::Common::D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                bitmapOptions: windows::Win32::Graphics::Direct2D::D2D1_BITMAP_OPTIONS_TARGET,
                ..Default::default()
            };
            let bitmap = unsafe {
                self.d2d1_context
                    .CreateBitmap(size, None, 0, &bitmap_properties)
            }?;
            todo!();
        } else {
            let bitmap = self.create_bitmap_from_frame()?;
            self.read_pixels_from_bitmap(&mut buffer, frame_size, bitmap)?;
        }
        Ok(buffer)
    }

    fn read_pixels_from_bitmap<'a>(
        &self,
        buffer: &'a mut [u8],
        desired_size: FrameSize,
        bitmap: ID2D1Bitmap1,
    ) -> std::result::Result<(), WgcError> {
        let size = D2D_SIZE_U {
            width: desired_size.width,
            height: desired_size.height,
        };
        let bitmap_properties = D2D1_BITMAP_PROPERTIES1 {
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: self.pixel_format.into(),
                alphaMode:
                    windows::Win32::Graphics::Direct2D::Common::D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            bitmapOptions: windows::Win32::Graphics::Direct2D::D2D1_BITMAP_OPTIONS_CPU_READ
                | windows::Win32::Graphics::Direct2D::D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
            ..Default::default()
        };
        let bitmap_cpu_read = unsafe {
            self.d2d1_context
                .CreateBitmap(size, None, 0, &bitmap_properties)
        }?;
        unsafe { bitmap_cpu_read.CopyFromBitmap(None, &bitmap, None) }?;
        let mapped_rect = unsafe {
            bitmap_cpu_read.Map(windows::Win32::Graphics::Direct2D::D2D1_MAP_OPTIONS_READ)
        }?;
        let pitch = mapped_rect.pitch as usize;
        let data_ptr = mapped_rect.bits;

        let row_bytes = (desired_size.width * self.pixel_format.bytes_per_pixel()) as usize;

        for (i, dst_row) in buffer.chunks_exact_mut(row_bytes).enumerate() {
            let src_ptr = unsafe { data_ptr.add(i * pitch) };
            let src_row = unsafe { std::slice::from_raw_parts(src_ptr, row_bytes) };
            dst_row.copy_from_slice(src_row);
        }
        unsafe { bitmap_cpu_read.Unmap() }?;
        Ok(())
    }

    fn create_bitmap_from_frame(&self) -> std::result::Result<ID2D1Bitmap1, WgcError> {
        let surface = self.frame.Surface()?;
        let dxgi_access: IDirect3DDxgiInterfaceAccess = surface.cast()?;
        let dxgi_surface: IDXGISurface = unsafe { dxgi_access.GetInterface() }?;
        let bitmap = unsafe {
            self.d2d1_context
                .CreateBitmapFromDxgiSurface(&dxgi_surface, None)
        }?;
        Ok(bitmap)
    }
}

impl From<Frame> for Direct3D11CaptureFrame {
    fn from(frame: Frame) -> Self {
        frame.frame
    }
}
#[derive(Debug, smart_default::SmartDefault, Clone, Copy, PartialEq, Eq)]
pub struct FrameSize {
    #[default(0)]
    pub width: u32,
    #[default(0)]
    pub height: u32,
}

impl From<SizeInt32> for FrameSize {
    fn from(size: SizeInt32) -> Self {
        let SizeInt32 {
            Width: width,
            Height: height,
        } = size;
        let width = width.max(0) as u32;
        let height = height.max(0) as u32;
        Self { width, height }
    }
}

fn create_d2d_factory() -> std::result::Result<ID2D1Factory1, WgcError> {
    unsafe {
        D2D1CreateFactory::<ID2D1Factory1>(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)
            .map_err(|e| e.into())
    }
}
