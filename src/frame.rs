use std::time::{Duration, Instant};

use crate::*;
use windows::Graphics::Capture::Direct3D11CaptureFrame;
#[derive(Debug)]
pub struct Frame {
    frame: Direct3D11CaptureFrame,
}
impl Frame {
    pub fn new(frame: Direct3D11CaptureFrame) -> Self {
        Self { frame }
    }

    /// Returns the time at which the frame was rendered
    pub fn render_time(&self) -> std::result::Result<Instant, FrameError> {
        let frame_delay_since_boot =
            Duration::from_nanos(self.frame.SystemRelativeTime()?.Duration as u64 * 100);
        let system_uptime = elapsed_since_system_boot();
        Ok(Instant::now() - system_uptime + frame_delay_since_boot)
    }
}

impl From<Frame> for Direct3D11CaptureFrame {
    fn from(frame: Frame) -> Self {
        frame.frame
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FrameError {
    #[error("Wgc: {0}")]
    WindowsError(#[from] windows::core::Error),
}
