use windows::Graphics::Capture::Direct3D11CaptureFrame;

#[derive(Debug)]
pub struct Frame {
    frame: Direct3D11CaptureFrame,
}
impl Frame {
    pub fn new(frame: Direct3D11CaptureFrame) -> Self {
        Self { frame }
    }
}

impl From<Frame> for Direct3D11CaptureFrame {
    fn from(frame: Frame) -> Self {
        frame.frame
    }
}
