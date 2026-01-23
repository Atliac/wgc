//! A simple wrapper for Windows Graphics Capture

// A macro that does nothing.
#[cfg(not(feature = "tracing"))]
macro_rules! noop_macro {
    ($($arg:tt)*) => {};
}

// A macro that conditionally uses tracing or noop_macro.
macro_rules! use_tracing_macros {
    ($($tracing_macro:ident),+) => {
        $(
#[cfg(feature = "tracing")]
pub(crate) use tracing::$tracing_macro;

#[cfg(not(feature = "tracing"))]
pub(crate) use noop_macro as $tracing_macro;
        )+
    };
}
use_tracing_macros!(debug, trace);

pub mod settings;
pub use settings::*;
pub mod frame;
pub use frame::*;
pub mod capture;
pub use capture::*;

mod utils {
    pub mod picker;
    pub use picker::*;
    pub(crate) mod qpc;
    pub(crate) use qpc::*;
}
pub use utils::*;
