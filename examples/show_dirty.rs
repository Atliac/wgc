//! Example demonstrating `ReportAndRender` mode by displaying ONLY the dirty areas
//! of each frame against a dark background.
//!
//! # Running this example
//! ```
//! cargo run --example show_dirty
//! ```

use image::{GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use show_image::{ImageInfo, ImageView};
use wgc::*;

#[show_image::main]
fn main() -> anyhow::Result<()> {
    // Run with `cargo run --example show_dirty` to see debug output,
    // or set `RUST_LOG=wgc=trace` for verbose output.
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("wgc=debug"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    if !capabilities::is_dirty_region_mode_configurable()? {
        println!("Dirty region mode is not supported on this Windows version.");
        return Ok(());
    }

    // Pick a window or monitor to capture
    let item = new_item_with_picker(None)?;

    let mut settings = WgcSettings::default();
    // Enable ReportAndRender mode: DWM only renders changed/dirty areas
    settings.dirty_region_mode = Some(true);

    // Disable cursor capture if supported so mouse movements don't continuously
    // trigger dirty regions, allowing you to focus on actual window content changes.
    if capabilities::is_cursor_configurable()? {
        settings.capture_cursor = Some(false);
    }

    let wgc = Wgc::new(item.clone(), settings)?;

    let title = item.DisplayName().unwrap_or_default().to_string_lossy();
    let window =
        show_image::create_window(format!("{title} - Only Dirty Regions"), Default::default())?;

    for frame in wgc {
        let frame = frame?;
        let frame_size = frame.size()?;
        let regions = frame.dirty_regions()?;
        let frame_pixels = frame.pixels()?;
        let frame_image: RgbaImage =
            ImageBuffer::from_raw(frame_size.width, frame_size.height, frame_pixels)
                .ok_or_else(|| anyhow::anyhow!("buffer size does not match frame dimensions"))?;

        // Start with a dark canvas; only dirty regions will be drawn onto it
        let mut display_image =
            RgbaImage::from_pixel(frame_size.width, frame_size.height, Rgba([20, 20, 25, 255]));

        for region in &regions {
            let x = region.x.max(0) as u32;
            let y = region.y.max(0) as u32;
            let w = region.width.min(frame_size.width.saturating_sub(x));
            let h = region.height.min(frame_size.height.saturating_sub(y));
            if w > 0 && h > 0 {
                let sub = frame_image.view(x, y, w, h);
                display_image.copy_from(&*sub, x, y)?;
            }
        }

        let image = ImageView::new(
            ImageInfo::rgba8_premultiplied(frame_size.width, frame_size.height),
            display_image.as_raw(),
        );
        window.set_image(&title, image)?;
    }

    Ok(())
}
