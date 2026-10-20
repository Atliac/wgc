# Configuration & Capabilities

Capture sessions can be customized with [`WgcSettings`](https://docs.rs/wgc/latest/wgc/settings/struct.WgcSettings.html).

Because `WgcSettings` is `#[non_exhaustive]`, construct it from `WgcSettings::default()` and configure the desired fields:

```rust,ignore
use std::time::Duration;
use wgc::*;

let mut settings = WgcSettings::default();

// Pixel format (RGBA8 or BGRA8)
settings.pixel_format = PixelFormat::RGBA8;

// Buffer queue depth
settings.frame_queue_length = 2;

// Optional Windows 10/11 features (check capability before setting!)
if capabilities::is_cursor_configurable()? {
    settings.capture_cursor = Some(false); // Hide cursor
}
if capabilities::is_border_configurable()? {
    settings.display_border = Some(false); // Hide yellow capture border
}
if capabilities::is_include_secondary_windows_configurable()? {
    settings.include_secondary_windows = Some(true); // Include popup & dropdown menus
}
if capabilities::is_dirty_region_mode_configurable()? {
    // Some(true) = ReportAndRender (render only changed pixels)
    // Some(false) = ReportOnly (render full frame + report dirty regions)
    settings.dirty_region_mode = Some(false);
}
```

## Runtime Capability Queries

The `capabilities` module lets you check if optional capture features are supported on the current Windows build:

```rust,ignore
use wgc::capabilities;

let is_supported = capabilities::is_wgc_supported()?;
let can_hide_cursor = capabilities::is_cursor_configurable()?;
let can_hide_border = capabilities::is_border_configurable()?;
let can_include_secondary = capabilities::is_include_secondary_windows_configurable()?;
let can_dirty_regions = capabilities::is_dirty_region_mode_configurable()?;
let can_throttle = capabilities::is_min_update_interval_configurable()?;
```

Run `cargo run --example capabilities` to inspect which features your system supports.
