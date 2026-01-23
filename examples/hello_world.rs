use std::{thread::sleep, time::Duration};

use wgc::*;

fn main() -> anyhow::Result<()> {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let item = new_item_with_picker()?;
    let settings = WgcSettings {
        frame_queue_length: 1,
        ..Default::default()
    };
    let mut wgc = Wgc::new(item, settings)?;
    {
        let frame = wgc.next().unwrap()?;
        sleep(Duration::from_secs(1));
        println!("{:?}", frame.render_time()?.elapsed());
    }
    let frame = wgc.next().unwrap()?;
    println!("{:?}", frame.render_time()?.elapsed());

    Ok(())
}
