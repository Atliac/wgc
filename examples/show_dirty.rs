//! Example demonstrating `ReportAndRender` mode by displaying ONLY the dirty areas
//! of each frame against a dark background.
//!
//! # Running this example
//! ```
//! cargo run --example show_dirty
//! ```

use image::{GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use show_image::{ImageInfo, ImageView, WindowOptions};
use wgc::*;

#[show_image::main]
fn main() -> anyhow::Result<()> {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("wgc=trace"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    if !capabilities::is_dirty_region_mode_configurable()? {
        println!("Dirty region mode is not supported on this Windows version.");
        return Ok(());
    }

    let mut builder = Wgc::new_from_picker(None)?
        // Enable ReportAndRender mode: DWM only renders changed/dirty areas
        .dirty_region_mode(true);

    // Disable cursor capture if supported so mouse movements don't continuously
    // trigger dirty regions, allowing you to focus on actual window content changes.
    if capabilities::is_cursor_configurable()? {
        builder = builder.capture_cursor(false);
    }

    let wgc = builder.build()?;

    let title = wgc
        .item()
        .DisplayName()
        .unwrap_or_default()
        .to_string_lossy();
    let window_options = WindowOptions::default().set_default_controls(false);
    let window = show_image::create_window(title.clone(), window_options)?;

    for frame in wgc {
        let frame_size;
        if let Ok(window_inner_size) = window.run_function_wait(|window| window.inner_size()) {
            frame_size = FrameSize {
                width: window_inner_size.x,
                height: window_inner_size.y,
            };
        } else {
            break;
        }
        if frame_size.width == 0 || frame_size.height == 0 {
            continue;
        }

        let frame = frame?;
        let regions = frame.dirty_regions_fitted(frame_size)?;
        let frame_pixels = frame.pixels_fitted(frame_size)?;
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
        if window.set_image(title.clone(), image).is_err() {
            break;
        }
    }

    println!("Bye!");
    Ok(())
}
