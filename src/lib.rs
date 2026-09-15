//! An ergonomic, high-level Rust wrapper for the Windows.Graphics.Capture API.
//!
//! This crate provides safe and idiomatic Rust bindings for screen and window capture
//! functionality on Windows.
//!
//! **Repository:** [GitHub](https://github.com/Atliac/wgc)
//!

pub mod settings;
pub use settings::*;
pub mod frame;
pub use frame::*;
pub mod capture;
pub use capture::*;
pub mod error;
pub use error::*;
pub mod capabilities;
pub use capabilities::*;

mod utils {
    pub mod picker;
    pub use picker::*;
    pub mod window;
    pub use window::*;
    pub mod monitor;
    pub use monitor::*;
    pub(crate) mod qpc;
    pub(crate) use qpc::*;
}
pub use utils::*;
