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

/// A helper macro to define modules and re-export their contents,
/// optionally gated by a compiler feature.
macro_rules! feature_mod {
    ($feature:literal $($mod:ident),+) => {$(
        #[cfg(feature = $feature)]
        pub mod $mod;
        #[cfg(feature = $feature)]
        pub use $mod::*;
    )+};
    ($($mod:ident),+) => {$(
        pub mod $mod;
        pub use $mod::*;
    )+};
}

feature_mod!(settings, frame, capture);

mod utils {
    pub mod picker;
    pub use picker::*;
}
pub use utils::*;
