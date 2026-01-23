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

feature_mod!(settings, frame, capture);

mod utils {
    pub mod picker;
    pub use picker::*;
}
pub use utils::*;
