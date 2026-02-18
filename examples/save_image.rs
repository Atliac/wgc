use image::{ImageBuffer, Rgba};
use wgc::*;

fn main() -> anyhow::Result<()> {
    // run with `cargo run --example save_image --features tracing` to see debug output,
    // set `RUST_LOG=trace` environment variable to see verbose output
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // pick an item to capture
    let item = new_item_with_picker(None)?;

    let wgc = Wgc::new(item.clone(), Default::default())?;
    let image_path = "target/a.png";

    // wgc is an iterator
    for frame in wgc.take(1) {
        let frame = frame?;
        println!("{} {:?}", item.clone().DisplayName()?, frame.size()?);
        let time = std::time::Instant::now();
        let frame_size = frame.size()?;
        let buffer = frame.read_pixels(frame_size)?;

        // use image crate to save the image
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(frame_size.width, frame_size.height, buffer).unwrap();

        println!("wgc: Read pixels in {:?}", time.elapsed());
        let time = std::time::Instant::now();
        image.save(image_path).unwrap();
        println!(
            "image: Saved in {:?}, Saved to `{}`. This can be slow in debug builds",
            time.elapsed(),
            image_path
        );
    }
    Ok(())
}
