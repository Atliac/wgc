use show_image::{ImageInfo, ImageView};
use wgc::*;

#[show_image::main]
fn main() -> anyhow::Result<()> {
    // run with `cargo run --example hello_world --features tracing` to see debug output,
    // set `RUST_LOG=wgc=trace` environment variable to see verbose output
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("wgc=debug"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // pick an item to capture
    let item = new_item_with_picker(None)?;

    // set up wgc
    let settings = WgcSettings {
        frame_queue_length: 1,
        ..Default::default()
    };

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
        let buffer = frame.read_pixels(frame_size)?;

        // use show_image crate to display the image
        // When closing the window, an "Error: invalid window ID: WindowId(...)" may appear.
        // This is expected behavior and does not indicate a bug in the application.
        let image = ImageView::new(
            ImageInfo::rgba8_premultiplied(frame_size.width, frame_size.height),
            &buffer,
        );
        window.set_image(title.clone(), image)?;
    }
    Ok(())
}
