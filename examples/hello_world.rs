use image::{ImageBuffer, Rgb, Rgba, buffer::ConvertBuffer};
use wgc::*;

fn main() -> anyhow::Result<()> {
    // run with `cargo run --example hello_world --features tracing` to see debug output,
    // set `RUST_LOG=trace` environment variable to see verbose output
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // pick an item to capture
    let item = new_item_with_picker(None)?;

    // set up wgc
    let settings = WgcSettings {
        frame_queue_length: 1,
        ..Default::default()
    };

    let wgc = Wgc::new(item.clone(), settings)?;

    // wgc is an iterator, let's take 3 frames
    for frame in wgc.take(1) {
        let frame = frame?;
        println!("{} {:?}", item.clone().DisplayName()?, frame.size()?);
        let time = std::time::Instant::now();
        let buffer = frame.read_pixels(frame.size()?)?;
        let size = frame.size()?;
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(size.width, size.height, buffer).unwrap();

        println!("time: {:?}", time.elapsed());
        let time = std::time::Instant::now();
        image.save("target/a.png").unwrap();
        println!("time: {:?}", time.elapsed());
    }
    Ok(())
}
