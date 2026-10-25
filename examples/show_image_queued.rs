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
    const FPS: u32 = 3;
    let wgc = builder.frame_queue_length(FRAME_QUEUE_LENGTH).build()?;

    let title = wgc
        .item()
        .DisplayName()
        .unwrap_or_default()
        .to_string_lossy();
    let title = format!("{title} - queue length ({FRAME_QUEUE_LENGTH})/FPS ({FPS})");
    let window_options = show_image::WindowOptions::default().set_default_controls(false);
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
        let buffer = frame.pixels_fitted(frame_size)?;

        if supports_dirty_regions {
            let mut image: RgbaImage =
                ImageBuffer::from_raw(frame_size.width, frame_size.height, buffer).ok_or_else(
                    || anyhow::anyhow!("buffer size does not match frame dimensions"),
                )?;

            if let Ok(regions) = frame.dirty_regions_fitted(frame_size) {
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
        sleep(Duration::from_secs(1) / FPS);
    }
    println!("Bye!");
    Ok(())
}
