use std::time::{Duration, Instant};

use crate::*;
use windows::{
    Graphics::{Capture::Direct3D11CaptureFrame, SizeInt32},
    Win32::{
        Graphics::{
            Direct2D::{
                Common::{D2D_RECT_F, D2D_SIZE_U, D2D1_COLOR_F, D2D1_PIXEL_FORMAT},
                D2D1_BITMAP_PROPERTIES1, D2D1_INTERPOLATION_MODE_LINEAR, ID2D1Bitmap1,
                ID2D1DeviceContext,
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
        let frame_bitmap = self.create_bitmap_from_frame()?;

        if frame_size != desired_size {
            let canvas_bitmap = self.create_canvas_bitmap(desired_size)?;

            unsafe {
                self.d2d1_context.BeginDraw();
                self.d2d1_context.SetTarget(&canvas_bitmap);
                let letterbox_color = D2D1_COLOR_F {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 1.0,
                };
                self.d2d1_context.Clear(Some(&letterbox_color));

                // Calculate the scale factor to maintain aspect ratio
                let src_w = frame_size.width as f32;
                let src_h = frame_size.height as f32;
                let dst_w = desired_size.width as f32;
                let dst_h = desired_size.height as f32;
                let scale = (dst_w / src_w).min(dst_h / src_h);

                // Determine the new dimensions
                let final_w = src_w * scale;
                let final_h = src_h * scale;

                let x = (dst_w - final_w) / 2.0;
                let y = (dst_h - final_h) / 2.0;

                // Create the destination rectangle
                let dest_rect = D2D_RECT_F {
                    left: x,
                    top: y,
                    right: x + final_w,
                    bottom: y + final_h,
                };

                self.d2d1_context.DrawBitmap(
                    &frame_bitmap,
                    Some(&dest_rect),
                    1.0, // Opacity
                    D2D1_INTERPOLATION_MODE_LINEAR,
                    None,
                    None,
                );

                self.d2d1_context.EndDraw(None, None)?;
            }
            self.read_pixels_from_bitmap(&mut buffer, desired_size, canvas_bitmap)?;
        } else {
            self.read_pixels_from_bitmap(&mut buffer, frame_size, frame_bitmap)?;
        }
        Ok(buffer)
    }

    fn create_canvas_bitmap(&self, size: FrameSize) -> std::result::Result<ID2D1Bitmap1, WgcError> {
        let size = D2D_SIZE_U {
            width: size.width,
            height: size.height,
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
        Ok(bitmap)
    }

    fn read_pixels_from_bitmap(
        &self,
        buffer: &mut [u8],
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
