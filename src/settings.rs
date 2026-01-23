use windows::Graphics::DirectX::DirectXPixelFormat;

#[derive(smart_default::SmartDefault, Debug)]
pub struct WgcSettings {
    #[default(DirectXPixelFormat::B8G8R8A8UIntNormalized)]
    pub pixel_format: DirectXPixelFormat,
    #[default(1)]
    pub frame_queue_length: i32,
}
