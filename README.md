# wgc - Windows Graphics Capture Wrapper

[![Crates.io](https://img.shields.io/crates/v/wgc)](https://crates.io/crates/wgc)
[![Documentation](https://docs.rs/wgc/badge.svg)](https://docs.rs/wgc)
[![License](https://img.shields.io/crates/l/wgc)](https://github.com/atliac/wgc/blob/master/LICENSE)
[![Rust (Windows)](https://github.com/Atliac/wgc/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/Atliac/wgc/actions/workflows/rust.yml)

A simple and ergonomic Rust wrapper for Windows.Graphics.Capture API, enabling screen/window capture on Windows 10/11.

> ⚠️ **Note**: This crate is currently under active development and may not yet be feature-complete or stable for production use.

## Features

- Easy-to-use iterator interface for capturing frames
- Support for capturing windows, monitors
- Automatic handling of resolution changes during capture
- Optional tracing support for debugging
- Low-level access to Direct3D11 frames when needed

## Requirements

- Windows 10 October 2018 Update (version 1809) or later
- Windows 11 (recommended)
- Rust 2024 edition

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wgc = "*"
```

## Usage

Here's a basic example of how to capture frames:

```rust
use wgc::*;

fn main() -> anyhow::Result<()> {
    // Pick an item to capture (window or monitor)
    let item = new_item_with_picker(None)?;

    // Configure capture settings
    let settings = WgcSettings {
        frame_queue_length: 1,
        ..Default::default()
    };

    // Create the capture instance
    let wgc = Wgc::new(item.clone(), settings)?;

    // Iterate over captured frames
    for frame in wgc.take(3) {
        let frame = frame?;
        println!("Captured frame from {} with size {:?}",
                 item.clone().DisplayName()?,
                 frame.size()?);
    }
    Ok(())
}
```

To enable tracing for debugging output, add the `tracing` feature and initialize a subscriber:

```toml
[dependencies]
wgc = { version = "*", features = ["tracing"] }
tracing-subscriber = "0.3"
```

```rust
use wgc::*;
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Rest of your code...
    let item = new_item_with_picker(None)?;
    let settings = WgcSettings::default();
    let wgc = Wgc::new(item, settings)?;

    for frame in wgc {
        let _frame = frame?;
        // Process frame...
    }
    Ok(())
}
```

Run with environment variable for verbose output:

```cmd
set RUST_LOG=trace
cargo run --example hello_world --features tracing

```

Or in PowerShell:

```powershell
$env:RUST_LOG="trace"
cargo run --example hello_world --features tracing
```

## Examples

Check out the [examples](./examples/) directory for more detailed usage examples:

- `hello_world.rs`: Basic usage example
- More examples coming soon...

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