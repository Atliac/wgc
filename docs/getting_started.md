# Getting Started

This chapter covers adding `wgc` to your Rust project and writing a basic screen capture script.

## Adding `wgc` to `Cargo.toml`

Add `wgc` to your `Cargo.toml` dependencies:

```toml
[dependencies]
wgc = "2.0"
```

If you plan to process or save captured images, you might also want helper crates like `image` or `anyhow`:

```toml
[dependencies]
wgc = "2.0"
anyhow = "1.0"
image = "0.25"
```

## Basic Usage Example

Below is a complete minimal example showing how to open the system picker dialog, capture a single frame, and inspect its dimensions and raw pixel data.

```rust,ignore
use wgc::{new_item_with_picker, Wgc};

fn main() -> anyhow::Result<()> {
    // 1. Prompt the user to select a window or monitor to capture
    let item = new_item_with_picker(None)?;

    // 2. Create a Wgc capture session with default settings
    let wgc = Wgc::new(item.clone(), Default::default())?;

    // 3. Iterate over captured frames (taking 1 frame here)
    for frame in wgc.take(1) {
        let frame = frame?;
        let size = frame.size()?;
        println!(
            "Captured frame from '{}' with size {}x{}",
            item.DisplayName()?,
            size.width,
            size.height
        );

        // Access raw RGBA pixel buffer
        let pixels: Vec<u8> = frame.pixels()?;
        println!("Buffer size in bytes: {}", pixels.len());
    }

    Ok(())
}
```

## How It Works

1. `new_item_with_picker(None)` opens the Windows system picker dialog allowing the user to pick any window or display.
2. `Wgc::new(item, settings)` initializes the Direct3D device, capture session, and frame pool.
3. `Wgc` implements `Iterator<Item = Result<Frame, WgcError>>`, yielding available frames sequentially.
