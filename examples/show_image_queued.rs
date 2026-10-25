use std::{thread::sleep, time::Duration};

use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use show_image::{ImageInfo, ImageView};
use wgc::*;

#[show_image::main]
fn main() -> anyhow::Result<()> {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("wgc=trace"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let supports_dirty_regions = capabilities::is_dirty_region_mode_configurable().unwrap_or(false);

    let mut builder = Wgc::new_from_picker(None)?;

    if capabilities::is_cursor_configurable()? {
        builder = builder.capture_cursor(false);
    }
    const FRAME_QUEUE_LENGTH: i32 = 10;
    let wgc = builder.frame_queue_length(FRAME_QUEUE_LENGTH).build()?;

    let title = wgc
        .item()
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
            if window.set_image(title.clone(), view).is_err() {
                break;
            }
        } else {
            let image = ImageView::new(
                ImageInfo::rgba8_premultiplied(frame_size.width, frame_size.height),
                &buffer,
            );
            if window.set_image(title.clone(), image).is_err() {
                break;
            }
        }
        sleep(Duration::from_secs(3) / FRAME_QUEUE_LENGTH as u32);
    }
    println!("Bye!");
    Ok(())
}
