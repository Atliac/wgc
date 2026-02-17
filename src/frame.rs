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
}
impl Frame {
    pub fn new(frame: Direct3D11CaptureFrame, d2d1_context: ID2D1DeviceContext) -> Self {
        Self {
            frame,
            d2d1_context,
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
    pub fn get_with_buffer<'a>(
        &self,
        buffer: &'a mut [u8],
        desired_size: FrameSize,
    ) -> std::result::Result<(), WgcError> {
        let frame_size = self.size()?;
        if frame_size != desired_size {
            let size = D2D_SIZE_U {
                width: desired_size.width,
                height: desired_size.height,
            };
            let bitmap_properties = D2D1_BITMAP_PROPERTIES1 {
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM,
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
            self.get_with_buffer_from_bitmap(buffer, frame_size, bitmap)?;
        }
        Ok(())
    }

    pub fn get(&self, desired_size: FrameSize) -> std::result::Result<Vec<u8>, WgcError> {
        let mut buffer = vec![0; (desired_size.width * desired_size.height * 4) as usize];
        self.get_with_buffer(&mut buffer, desired_size)?;
        Ok(buffer)
    }

    fn get_with_buffer_from_bitmap<'a>(
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
                //format: windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM,
                format: windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT(
                    windows::Graphics::DirectX::DirectXPixelFormat::R8G8B8A8UIntNormalized.0,
                ),
                alphaMode:
                    windows::Win32::Graphics::Direct2D::Common::D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            bitmapOptions: windows::Win32::Graphics::Direct2D::D2D1_BITMAP_OPTIONS_CPU_READ
                | windows::Win32::Graphics::Direct2D::D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
            ..Default::default()
        };
        let bitmap_cpu = unsafe {
            self.d2d1_context
                .CreateBitmap(size, None, 0, &bitmap_properties)
        }?;
        unsafe { bitmap_cpu.CopyFromBitmap(None, &bitmap, None) }?;
        let mapped_rect =
            unsafe { bitmap_cpu.Map(windows::Win32::Graphics::Direct2D::D2D1_MAP_OPTIONS_READ) }?;
        let pitch = mapped_rect.pitch as usize;
        let data_ptr = mapped_rect.bits;

        const BYTES_PER_PIXEL: u32 = 4; // BGRA
        let row_bytes = (desired_size.width * BYTES_PER_PIXEL) as usize;

        for (i, dst_row) in buffer.chunks_exact_mut(row_bytes).enumerate() {
            let src_ptr = unsafe { data_ptr.add(i * pitch) };
            let src_row = unsafe { std::slice::from_raw_parts(src_ptr, row_bytes) };
            dst_row.copy_from_slice(src_row);
        }

        unsafe { bitmap_cpu.Unmap() }?;
        Ok(())
    }

    fn create_bitmap_from_frame(&self) -> std::result::Result<ID2D1Bitmap1, WgcError> {
        let surface = self.frame.Surface()?;
        let access: IDirect3DDxgiInterfaceAccess = surface.cast()?;
        let dxgi_surface: IDXGISurface = unsafe { access.GetInterface() }?;
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
