use wgc::*;
use windows::Graphics::Capture::Direct3D11CaptureFrame;

fn main() -> anyhow::Result<()> {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let item = match new_item_with_picker() {
        Ok(val) => val,
        Err(CaptureItemPickerError::NoItemSelected) => {
            eprintln!("No item selected");
            return Ok(());
        }
        Err(err) => return Err(err.into()),
    };
    let settings = WgcSettings {
        frame_queue_length: 3,
        ..Default::default()
    };
    let wgc = Wgc::new(item, settings)?;
    for frame in wgc {
        let _frame: Direct3D11CaptureFrame = frame?.into();
        //println!("Frame: {:?}", frame.ContentSize());
    }

    Ok(())
}
