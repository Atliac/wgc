use image::{ImageBuffer, Rgba};
use wgc::*;

fn main() -> anyhow::Result<()> {
    // run with `cargo run --example save_image` to see debug output,
    // set `RUST_LOG=trace` environment variable to see verbose output
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // pick an item to capture
    let item = new_item_with_picker(None)?;

    let wgc = Wgc::new(item.clone(), Default::default())?;

    // The size to fit a captured frame into when using `pixels_fitted`.
    // Pick something with a different aspect ratio than your source to see
    // the letterboxing.
    let fitted_size = FrameSize {
        width: 512,
        height: 512,
    };

    // wgc is an iterator
    for frame in wgc.take(1) {
        let frame = frame?;
        let frame_size = frame.size()?;
        println!("{} {:?}", item.clone().DisplayName()?, frame_size);

        // `pixels` returns the frame at its native size
        let time = std::time::Instant::now();
        let native = frame.pixels()?;
        println!("wgc: Read {} bytes in {:?}", native.len(), time.elapsed());
        save_png("target/native.png", frame_size, native)?;

        // `pixels_fitted` scales the frame to fit within the given size. The
        // aspect ratio is preserved, so the image is letterboxed (centered
        // with gray borders) to fill the target size.
        let time = std::time::Instant::now();
        let fitted = frame.pixels_fitted(fitted_size)?;
        println!("wgc: Read {} bytes in {:?}", fitted.len(), time.elapsed());
        save_png("target/fitted.png", fitted_size, fitted)?;
    }
    Ok(())
}

/// Saves raw pixel data as a PNG using the `image` crate.
fn save_png(path: &str, size: FrameSize, pixels: Vec<u8>) -> anyhow::Result<()> {
    let time = std::time::Instant::now();
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(size.width, size.height, pixels)
            .ok_or_else(|| anyhow::anyhow!("buffer does not match {size:?}"))?;
    image.save(path)?;
    println!(
        "image: Saved to `{}` in {:?}. This can be slow in debug builds",
        path,
        time.elapsed()
    );
    Ok(())
}
