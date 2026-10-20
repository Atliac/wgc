# wgc - Windows Graphics Capture Wrapper

[![Crates.io](https://img.shields.io/crates/v/wgc)](https://crates.io/crates/wgc)
[![Documentation](https://docs.rs/wgc/badge.svg)](https://docs.rs/wgc)
[![License](https://img.shields.io/crates/l/wgc)](https://github.com/Atliac/wgc/blob/master/LICENSE)
[![Rust CI](https://github.com/Atliac/wgc/actions/workflows/ci.yml/badge.svg)](https://github.com/Atliac/wgc/actions/workflows/ci.yml)
[![Stability: Stable](https://img.shields.io/badge/stability-stable-brightgreen)](https://github.com/Atliac/wgc)
[![Maintenance: Active](https://img.shields.io/badge/maintenance-active-blue)](https://github.com/Atliac/wgc)

A simple and ergonomic Rust wrapper for Windows.Graphics.Capture API, enabling screen/window capture on Windows 10/11.

**Realtime & AI-optimized** capture for ML and computer vision workflows.

## Features

- **Hardware-Accelerated Capture**: High-performance screen and window capture powered directly by Windows Direct3D 11 and Desktop Window Manager (DWM).
- **Flexible Target Selection**: Capture entire displays, specific application windows, or let users choose interactively via the native Windows picker.
- **Idiomatic Streaming**: Stream frames sequentially using standard Rust iterator semantics with automatic resource management.
- **AI & Computer Vision Ready**: Real-time hardware letterboxing and scaling for uniform machine learning tensor inputs without distortion.
- **Dirty Region Optimization**: Track changed sub-rectangles between frames to avoid re-processing static visual content.
- **Adaptive Resolution Handling**: Automatically recreates internal frame buffers on-the-fly when windows resize, move across monitors, or change DPI.
- **Comprehensive Visual Controls**: Toggle cursor visibility, capture borders, secondary popups/menus, and throttle capture rates.
- **Configurable Pixel Formats**: Choose between standard color formats like RGBA and BGRA with customizable channel layouts.

## Requirements

- Windows 10 October 2018 Update (version 1809) or later
- Windows 11 (recommended)

## Usage

Here's a basic example of how to capture frames:

```rust
use wgc::*;

fn main() -> anyhow::Result<()> {
    // Pick an item to capture and configure capture settings with builder
    let wgc = Wgc::new_from_picker(None)?
        .frame_queue_length(2)
        .capture_cursor(false)
        .build()?;

    // Iterate over captured frames
    for frame in wgc.take(1) {
        let frame = frame?;
        println!("Captured frame with size {:?}", frame.size()?);

        let buffer: Vec<u8> = frame.pixels()?;
        println!("Buffer length: {} bytes", buffer.len());
    }
    Ok(())
}
```

## Examples

Clone the repository to explore and run the examples:

```bash
git clone https://github.com/Atliac/wgc.git
cd wgc
```

- [**`show_image`**](./examples/show_image.rs): Real-time screen/window viewer with dirty region highlighting.
  ```bash
  cargo run --example show_image
  ```
- [**`show_dirty`**](./examples/show_dirty.rs): Captures and displays only changed pixels between frames.
  ```bash
  cargo run --example show_dirty
  ```
- [**`save_image`**](./examples/save_image.rs): Captures a frame and saves native and letterboxed PNGs to disk.
  ```bash
  cargo run --example save_image
  ```
- [**`capabilities`**](./examples/capabilities.rs): Queries supported Windows Graphics Capture features on the host OS.
  ```bash
  cargo run --example capabilities
  ```

## Documentation

- [Migration guide](./MIGRATION.md): how to upgrade from `wgc` v1.0 → v2.x → v3.0.
- [docs.rs](https://docs.rs/wgc): API reference.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
