# Getting Started

Add `wgc` to your `Cargo.toml`:

```toml
[dependencies]
wgc = "2.0"
```

## Minimal Example

Prompt the user to pick a target and capture a frame:

```rust,ignore
use wgc::*;

fn main() -> anyhow::Result<()> {
    // 1. Prompt user to select a window or monitor
    let item = new_item_with_picker(None)?;

    // 2. Start a capture session with default settings
    let wgc = Wgc::new(item.clone(), Default::default())?;

    // 3. Wgc implements Iterator over Result<Frame, WgcError>
    for frame in wgc.take(1) {
        let frame = frame?;
        let size = frame.size()?;
        println!("Captured frame: {}x{}", size.width, size.height);

        // Access raw RGBA pixel buffer (width * height * 4 bytes)
        let pixels: Vec<u8> = frame.pixels()?;
        println!("Buffer length: {} bytes", pixels.len());
    }

    Ok(())
}
```

For more in-depth examples with image saving and GUI display, see the [Examples chapter](examples.md) or clone the repository to run them directly.
