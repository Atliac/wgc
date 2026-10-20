use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use show_image::{ImageInfo, ImageView};
use wgc::*;

#[show_image::main]
fn main() -> anyhow::Result<()> {
    // run with `cargo run --example show_image` to see debug output,
    // set `RUST_LOG=wgc=trace` environment variable to see verbose output
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("wgc=debug"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let supports_dirty_regions = capabilities::is_dirty_region_mode_configurable().unwrap_or(false);

    // pick an item to capture
    let item = new_item_with_picker(None)?;

    let mut settings = WgcSettings::default();

    if capabilities::is_cursor_configurable()? {
        settings.capture_cursor = Some(false);
    }

    let wgc = Wgc::new(item.clone(), settings)?;

    let title = item
        .clone()
        .DisplayName()
        .unwrap_or_default()
        .to_string_lossy();
    let window = show_image::create_window(title.clone(), Default::default())?;

    for frame in wgc {
        let frame = frame?;
        let frame_size = frame.size()?;
        let buffer = frame.pixels()?;

        if supports_dirty_regions {
            let mut image: RgbaImage =
                ImageBuffer::from_raw(frame_size.width, frame_size.height, buffer).ok_or_else(
                    || anyhow::anyhow!("buffer size does not match frame dimensions"),
                )?;

            if let Ok(regions) = frame.dirty_regions() {
                let border_color = Rgba([0, 255, 0, 255]); // Bright green
                for region in regions {
                    if region.width > 0 && region.height > 0 {
                        let rect =
                            Rect::at(region.x, region.y).of_size(region.width, region.height);
                        draw_hollow_rect_mut(&mut image, rect, border_color);
                    }
                }
            }

            let view = ImageView::new(
                ImageInfo::rgba8_premultiplied(frame_size.width, frame_size.height),
                image.as_raw(),
            );
            window.set_image(title.clone(), view)?;
        } else {
            let image = ImageView::new(
                ImageInfo::rgba8_premultiplied(frame_size.width, frame_size.height),
                &buffer,
            );
            window.set_image(title.clone(), image)?;
        }
    }
    Ok(())
}
