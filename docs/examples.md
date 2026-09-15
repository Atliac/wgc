# Examples & Practical Use

This chapter provides complete runnable examples showing how to integrate `wgc` into real applications.

## Example 1: Saving Captured Frames to PNG

In this example, we capture a single frame from a selected window or monitor, save the native image to disk as `native.png`, and save a letterboxed version scaled to 512x512 as `fitted.png`.

```rust,ignore
use image::{ImageBuffer, Rgba};
use wgc::*;

fn main() -> anyhow::Result<()> {
    // 1. Prompt user to select target
    let item = new_item_with_picker(None)?;

    // 2. Initialize Wgc session
    let wgc = Wgc::new(item.clone(), Default::default())?;

    let fitted_size = FrameSize {
        width: 512,
        height: 512,
    };

    // 3. Process 1 frame
    for frame in wgc.take(1) {
        let frame = frame?;
        let native_size = frame.size()?;
        println!("Capturing target: {}", item.DisplayName()?);

        // Native size frame
        let native_pixels = frame.pixels()?;
        save_png("native.png", native_size, native_pixels)?;

        // Resolution-fitted frame (letterboxed)
        let fitted_pixels = frame.pixels_fitted(fitted_size)?;
        save_png("fitted.png", fitted_size, fitted_pixels)?;
    }

    Ok(())
}

fn save_png(path: &str, size: FrameSize, pixels: Vec<u8>) -> anyhow::Result<()> {
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(size.width, size.height, pixels)
            .ok_or_else(|| anyhow::anyhow!("pixel buffer size mismatch"))?;
    image.save(path)?;
    println!("Saved image to '{path}'");
    Ok(())
}
```

---

## Example 2: Displaying Captured Video in a Window

You can pair `wgc` with windowing and image display crates like `show-image` to build real-time screen viewers or streaming clients.

```rust,ignore
use show_image::{create_window, ImageInfo, ImageView};
use wgc::*;

#[show_image::main]
fn main() -> anyhow::Result<()> {
    let item = new_item_with_picker(None)?;
    let wgc = Wgc::new(item.clone(), Default::default())?;

    let title = item.DisplayName()?.to_string_lossy();
    let window = create_window(title.clone(), Default::default())?;

    for frame in wgc {
        let frame = frame?;
        let size = frame.size()?;
        let buffer = frame.pixels()?;

        let image = ImageView::new(
            ImageInfo::rgba8_premultiplied(size.width, size.height),
            &buffer,
        );
        window.set_image(title.clone(), image)?;
    }

    Ok(())
}
```

---

## Example 3: Running Existing Examples from Repository

The `wgc` repository includes ready-to-run examples:

- **Save Image**:
  ```bash
  cargo run --example save_image
  ```

- **Show Image (Real-time GUI viewer)**:
  ```bash
  cargo run --example show_image
  ```

- **Check System Capabilities**:
  ```bash
  cargo run --example capabilities
  ```
