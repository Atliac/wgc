# Configuration & Capabilities

`wgc` allows fine-grained customization of capture sessions using `WgcSettings`. Additionally, runtime capability functions in `wgc::capabilities` let you check which features are supported on the host Windows system.

## `WgcSettings` Configuration

`WgcSettings` controls frame formats, buffer queue length, scaling interpolation, and optional capture features.

```rust,ignore
use std::time::Duration;
use wgc::settings::{FrameInterpolationMode, PixelFormat, WgcSettings};

let mut settings = WgcSettings::default();

// 1. Pixel Format (RGBA8 or BGRA8)
settings.pixel_format = PixelFormat::RGBA8;

// 2. Buffer Queue Length (number of frames queued in memory)
settings.frame_queue_length = 2;

// 3. Scaling Interpolation Mode (for fitted letterbox scaling)
settings.frame_interpolation_mode = FrameInterpolationMode::Linear;

// 4. Optional Windows 10/11 features (must check capabilities first!)
settings.capture_cursor = Some(false);          // Hide mouse cursor
settings.display_border = Some(false);          // Hide yellow capture border
settings.include_secondary_windows = Some(true); // Include popups/child windows
settings.min_update_interval = Some(Duration::from_millis(16)); // Throttle frame rate (~60 FPS)
```

### Interpolation Modes

When using resolution scaling / letterboxing (`pixels_fitted`), you can set `frame_interpolation_mode` to one of the following:

- `NearestNeighbor`: Fastest processing, lower visual fidelity.
- `Linear`: Balanced performance and quality (default).
- `Cubic`: Smooth 16-sample interpolation.
- `MultiSampleLinear`: Anti-aliasing for small scale-downs.
- `HighQualityCubic`: Best visual quality for significant downscaling.

---

## Checking System Capabilities

Windows Graphics Capture added several settings in newer Windows updates (such as hiding the capture border or cursor). Attempting to enable an unsupported setting on older Windows builds will result in a runtime error.

You can inspect capabilities using the `capabilities` module:

```rust,ignore
use wgc::capabilities;

fn main() -> anyhow::Result<()> {
    if !capabilities::is_wgc_supported()? {
        println!("Windows Graphics Capture is not supported on this OS.");
        return Ok(());
    }

    if capabilities::is_cursor_configurable()? {
        println!("Cursor capture toggling is supported!");
    }

    if capabilities::is_border_configurable()? {
        println!("Border visibility toggling is supported!");
    }

    if capabilities::is_dirty_region_mode_configurable()? {
        println!("Dirty region tracking is supported!");
    }

    if capabilities::is_min_update_interval_configurable()? {
        println!("Minimum update interval configuration is supported!");
    }

    Ok(())
}
```
