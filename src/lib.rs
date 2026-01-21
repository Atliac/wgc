//! A simple wrapper for Windows Graphics Capture

pub mod error;
pub mod settings;
pub use error::*;
pub use settings::*;

/// Feature gate macro for modular inclusion.
///
/// This macro conditionally compiles and re-exports modules based on the specified feature flag.
/// Each module is only compiled and made available when the corresponding feature is enabled.
macro_rules! feature_mod {
    ($feature:literal $($mod:ident),+) => {$(
#[cfg(feature = $feature)]
pub mod $mod;
#[cfg(feature = $feature)]
pub use $mod::*;
    )+};
}

feature_mod!("sync" wgc,frame);
feature_mod!("async" wgc_async,frame_async);
