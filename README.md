# wgc - Windows Graphics Capture Wrapper

[![Crates.io](https://img.shields.io/crates/v/wgc)](https://crates.io/crates/wgc)
[![Documentation](https://docs.rs/wgc/badge.svg)](https://docs.rs/wgc)
[![License](https://img.shields.io/crates/l/wgc)](https://github.com/atliac/wgc/blob/master/LICENSE)
[![Rust (Windows)](https://github.com/Atliac/wgc/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/Atliac/wgc/actions/workflows/rust.yml)

A simple and ergonomic Rust wrapper for Windows.Graphics.Capture API, enabling screen/window capture on Windows 10/11.

**Realtime & AI-optimized** capture for ML and computer vision workflows.

## Features
- Realtime & AI-optimized: Capture any window or monitor at any resolution and resize in real-time using letterbox scaling. Ideal for ML pipelines, streaming, and computer vision applications.
- Ergonomic iterator-based API for capturing frames via the `Wgc` struct
- Interactive picker dialog for selecting windows or monitors to capture
- Configurable pixel formats (currently `RGBA8` and `BGRA8`, with more formats planned) via `WgcSettings`
- Automatic buffer recreation when capture resolution changes
- Frame size normalization with letterboxing for consistent output dimensions
- Optional `tracing` feature for debug logging
- Zero-copy frame access with efficient DirectX/Direct2D integration

## Requirements

- Windows 10 October 2018 Update (version 1809) or later
- Windows 11 (recommended)
- Rust 2024 edition

## Usage

Here's a basic example of how to capture frames:

```rust
use wgc::*;

fn main() -> anyhow::Result<()> {
    // Pick an item to capture (window or monitor)
    let item = new_item_with_picker(None)?;

    // Create the capture instance
    let wgc = Wgc::new(item.clone(), Default::default())?;

    // Iterate over captured frames
    for frame in wgc.take(1) {
        let frame = frame?;
        println!("Captured frame from {} with size {:?}",
                 item.clone().DisplayName()?,
                 frame.size()?);
        
        let frame_size = frame.size()?;
        let buffer:Vec<u8> = frame.read_pixels(frame_size)?;
    }
    Ok(())
}
```

## Examples

Check out the [examples](./examples/) directory for more detailed usage examples:

- [save_image](./examples/save_image.rs): Captures a screen item and saves it as an image file to disk.
- [show_image](./examples/show_image.rs): Captures a screen item and displays it in a window.
- [tutorial](./examples/tutorial.rs): Provides a comprehensive tutorial on wgc for those who wish to leverage most of its features.

## Development Status

This crate is currently under active development. Features may be added, removed, or changed in future releases. Please check back regularly for updates.

API stability is not guaranteed until version 1.0.0 is released.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
