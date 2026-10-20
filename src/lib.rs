//! An ergonomic, high-level Rust wrapper for the Windows.Graphics.Capture API.
//!
//! This crate provides safe and idiomatic Rust bindings for screen and window capture
//! functionality on Windows.
//!
//! **Repository:** [GitHub](https://github.com/Atliac/wgc)
//!
//! # Quick Start
//!
//! Capture sessions are created using the fluent builder API via `Wgc::new_from_*` constructors:
//! - [`Wgc::new_from_picker`]: Interactively select a window or monitor via the Windows picker UI.
//! - [`Wgc::new_from_hwnd`]: Capture a specific window by its `HWND` handle.
//! - [`Wgc::new_from_monitor`]: Capture a specific monitor by its `HMONITOR` handle.
//! - [`Wgc::new_from_item`]: Capture an existing `GraphicsCaptureItem`.
//!
//! ### Basic Capture
//!
//! ```no_run
//! use wgc::*;
//!
//! fn main() -> anyhow::Result<()> {
//!     // 1. Pick a window or monitor and configure capture
//!     let wgc = Wgc::new_from_picker(None)?
//!         .frame_queue_length(2)
//!         .capture_cursor(false)
//!         .build()?;
//!
//!     // 2. Iterate over captured frames
//!     for frame in wgc.take(10) {
//!         let frame = frame?;
//!         println!("Captured frame: {:?}", frame.size()?);
//!
//!         // Get RGBA pixel buffer
//!         let buffer: Vec<u8> = frame.pixels()?;
//!         println!("Buffer size: {} bytes", buffer.len());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ### Custom Configuration
//!
//! The builder allows fine-tuning performance and visual properties:
//!
//! ```no_run
//! use std::time::Duration;
//! use wgc::*;
//!
//! # fn example() -> anyhow::Result<()> {
//! let wgc = Wgc::new_from_picker(None)?
//!     .pixel_format(PixelFormat::BGRA8)
//!     .frame_queue_length(3)
//!     .capture_cursor(true)
//!     .display_border(false)
//!     .include_secondary_windows(true)
//!     .dirty_region_mode(true)
//!     .min_update_interval(Duration::from_millis(16))
//!     .frame_interpolation_mode(FrameInterpolationMode::HighQualityCubic)
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Frame Resizing & Normalization
//!
//! Use [`Frame::pixels_fitted`] to scale captured frames into a fixed dimension
//! with letterboxing (preserving aspect ratio), ideal for machine learning pipelines:
//!
//! ```no_run
//! # use wgc::*;
//! # fn example(frame: Frame) -> anyhow::Result<()> {
//! let fitted_size = FrameSize { width: 512, height: 512 };
//! let fitted_pixels: Vec<u8> = frame.pixels_fitted(fitted_size)?;
//! # Ok(())
//! # }
//! ```
//!

pub mod settings;
pub use settings::*;
pub mod builder;
pub use builder::*;
pub mod frame;
pub use frame::*;
pub mod capture;
pub use capture::*;
pub mod error;
pub use error::*;
pub mod capabilities;
pub use capabilities::*;

pub(crate) mod utils {
    pub(crate) mod picker;
    pub(crate) use picker::*;
    pub(crate) mod window;
    pub(crate) use window::*;
    pub(crate) mod monitor;
    pub(crate) use monitor::*;
    pub(crate) mod qpc;
    pub(crate) use qpc::*;
}
pub(crate) use utils::*;
