use wgc::*;

fn main() -> anyhow::Result<()> {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let item = new_item_with_picker(None)?;
    let settings = WgcSettings {
        frame_queue_length: 1,
        ..Default::default()
    };
    let mut wgc = Wgc::new(item, settings)?;
    if let Some(frame) = wgc.next() {
        println!("{:?}", frame?.size()?);
    }

    Ok(())
}
