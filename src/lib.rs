//! A simple wrapper for Windows Graphics Capture

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

//feature_mod!("sync" wgc_sync, frame_sync);
feature_mod!(error, settings, wgc, frame);

mod utils {
    pub mod picker;
}
