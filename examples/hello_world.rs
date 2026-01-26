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
    for frame in wgc.take(3) {
        let frame = frame?;
        println!("{} {:?}", item.clone().DisplayName()?, frame.size()?);
    }
    Ok(())
}
