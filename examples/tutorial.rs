//! Comprehensive tutorial demonstrating the main features of the `wgc` crate.
//!
//! # Prerequisites
//! It is recommended to complete the `save_image` and `show_image` examples first
//! to familiarize yourself with the basic concepts.
//!
//! # Running this example
//! ```
//! cargo run --example tutorial --features tracing
//! ```
//! Set the `RUST_LOG` environment variable to control log verbosity:
//! ```
//! set RUST_LOG=trace
//! cargo run --example tutorial --features tracing
//! ```

use windows::Win32::{
    Graphics::Gdi::MonitorFromWindow, UI::WindowsAndMessaging::GetForegroundWindow,
};

fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber for logging and debugging output.
    // The log level can be controlled via the RUST_LOG environment variable.
    // If not set, defaults to "debug" level.
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // ========================================================================
    // Step 1: Create a GraphicsCaptureItem
    // ========================================================================
    // A GraphicsCaptureItem represents the source to capture frames from.
    // The `wgc` crate provides several helper functions to create items:
    //
    // - `new_item_with_picker(None)`: Opens the Windows picker UI for user selection.
    // - `new_item_from_hwnd(hwnd)`: Captures from a specific window handle.
    // - `new_item_from_monitor(hmonitor)`: Captures from a specific monitor.
    //
    // In this example, we capture from the monitor containing the foreground window.
    let item = wgc::new_item_from_monitor(unsafe {
        MonitorFromWindow(
            GetForegroundWindow(),
            windows::Win32::Graphics::Gdi::MONITOR_DEFAULTTONEAREST,
        )
    })?;

    // Display basic information about the capture item.
    // `DisplayName()` returns a human-readable name (e.g., monitor name or window title).
    // `Size()` returns the resolution of the capture source.
    println!(
        "Item name: {:?}, Item Size: {:?}",
        item.DisplayName()?,
        item.Size()?
    );

    // ========================================================================
    // Step 2: Configure WgcSettings
    // ========================================================================
    // `WgcSettings` allows you to customize the capture behavior.
    // Common settings include:
    // - `pixel_format`: The pixel format of captured frames (e.g., RGBA8, BGRA8).
    // - `frame_queue_length`: Number of frames to buffer. Higher values provide
    //   more buffering but increase latency.
    //
    let settings = wgc::WgcSettings {
        // pixel_format: wgc::PixelFormat::BGRA8, // Uncomment to set a specific format
        frame_queue_length: 3,
        ..Default::default() // Use default values for all other settings
    };

    // Print the current settings configuration for reference.
    println!("{:?}", settings);

    // ========================================================================
    // Step 3: Initialize the Wgc capture session
    // ========================================================================
    // Create a new `Wgc` instance with the configured item and settings.
    // This sets up the Windows Graphics Capture pipeline and prepares for
    // frame acquisition.
    let wgc = wgc::Wgc::new(item, settings)?;

    // ========================================================================
    // Step 4: Capture and process frames
    // ========================================================================
    // The `Wgc` instance is an iterator that yields captured frames.
    // We use `.take(3)` to capture only 3 frames for this demonstration.
    //
    // Each frame contains metadata such as `render_time`, which indicates
    // when the frame was rendered by the GPU. This is useful for measuring
    // capture latency and synchronization.
    for (i, frame) in wgc.take(3).enumerate() {
        let frame = frame?;
        let render_time = frame.render_time()?;

        // Calculate and display how long ago this frame was rendered.
        // Tip: Adjust `frame_queue_length` in Step 2 to see how buffering
        // affects the time difference between render and capture.
        println!("Frame {} rendered {:?} ago", i, render_time.elapsed());

        // Wait 100ms between frames to simulate processing time and make
        // the output easier to read.
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
