# Migration Guide

This document describes breaking API changes and how to update downstream code.
For the full history of changes, see [CHANGELOG.md](./CHANGELOG.md).

- [v1.x → v2.0](#v1x--v20)

## v1.x → v2.0

### Summary of breaking changes

| # | Change | What you need to do |
|---|--------|---------------------|
| 1 | `Frame::read_pixels(Option<FrameSize>)` split into `Frame::pixels()` and `Frame::pixels_fitted(FrameSize)` | Rename calls (see [below](#1-frameread_pixels--framepixels--framepixels_fitted)) |
| 2 | `WgcSettings` is now `#[non_exhaustive]` | Build it from `WgcSettings::default()` and assign fields (see [below](#2-wgcsettings-is-now-non_exhaustive)) |
| 3 | The `tracing` Cargo feature was removed; `tracing` is a required dependency | Drop `--features tracing` and `features = ["tracing"]` |
| 4 | The `tutorial` example was removed | Use the [examples](./examples/) and [DeepWiki docs](https://deepwiki.com/Atliac/wgc) |

### 1. `Frame::read_pixels` → `Frame::pixels` / `Frame::pixels_fitted`

The single method that took an `Option<FrameSize>` was split into two methods that
take no `Option`. The pixel formats, buffer sizes and letterboxing behavior are
unchanged — only the method names and signatures differ.

**Before (1.x)**

```rust
// Native size
let native: Vec<u8> = frame.read_pixels(None)?;

// Scaled to fit, letterboxed
let fitted: Vec<u8> = frame.read_pixels(Some(FrameSize {
    width: 512,
    height: 512,
}))?;
```

**After (2.0)**

```rust
// Native size
let native: Vec<u8> = frame.pixels()?;

// Scaled to fit, letterboxed
let fitted: Vec<u8> = frame.pixels_fitted(FrameSize {
    width: 512,
    height: 512,
})?;
```

Method mapping:

| 1.x | 2.0 | Notes |
|-----|-----|-------|
| `read_pixels(None)` | `pixels()` | Frame at its native size; buffer length is `width * height * bytes_per_pixel` |
| `read_pixels(Some(size))` | `pixels_fitted(size)` | Frame fitted into `size` with the aspect ratio preserved, centered with gray borders (letterboxed) |

A simple find-and-replace usually suffices:

```text
read_pixels(None)        → pixels()
read_pixels(Some(size))  → pixels_fitted(size)
```

> **Note:** `pixels()` is implemented in terms of `pixels_fitted(self.size()?)`, so the
> native-size path is unchanged in cost. Only the fitted path allocates a second bitmap
> and scales the frame.

### 2. `WgcSettings` is now `#[non_exhaustive]`

`WgcSettings` can no longer be constructed with a struct expression from outside the
crate. This lets new settings be added in future minor releases without another
breaking change.

**Before (1.x)**

```rust
let settings = WgcSettings {
    pixel_format: PixelFormat::RGBA8,
    frame_queue_length: 2,
};
```

**After (2.0)**

Start from `WgcSettings::default()` and assign the fields you care about:

```rust
let mut settings = WgcSettings::default();
settings.pixel_format = PixelFormat::RGBA8;
settings.frame_queue_length = 2;
```

> **Important:** adding `..Default::default()` to the struct literal does **not** fix
> this. Functional update syntax is still a struct expression, so it is rejected with
> [`E0639`](https://doc.rust-lang.org/error_codes/E0639.html) —
> `cannot create non-exhaustive struct using struct expression`. The compiler error is:
>
> ```text
> error[E0639]: cannot create non-exhaustive struct using struct expression
>  --> src/main.rs:1:21
>   |
> 1 |     let settings = WgcSettings {
>   |  ______________________^
> 2 | |         pixel_format: PixelFormat::RGBA8,
> 3 | |         frame_queue_length: 2,
> 4 | |         ..Default::default()
>   | |_____^
> ```
>
> The `..Default::default()` pattern only works *inside* the `wgc` crate itself, or for a
> non-`#[non_exhaustive]` struct.

If you only need the defaults, nothing changes:

```rust
let wgc = Wgc::new(item, Default::default())?;
// or, explicitly:
let wgc = Wgc::new(item, WgcSettings::default())?;
```

Reading fields and destructuring are unaffected. Both of these still compile:

```rust
let settings = WgcSettings::default();
println!("{:?}", settings.pixel_format);

let WgcSettings { pixel_format, .. } = settings;
```

### 3. The `tracing` Cargo feature was removed

`tracing` is now a required dependency of `wgc`, so logging is always compiled in and
controlled at runtime through the `RUST_LOG` environment variable. There is no longer a
way to compile the logging out.

Commands and manifests that opt in to the old feature must be updated; Cargo will fail
with `package 'wgc' does not have the feature 'tracing'` otherwise.

**Before (1.x)**

```toml
# Cargo.toml
[dependencies]
wgc = { version = "1", features = ["tracing"] }
```

```bash
cargo run --example save_image --features tracing
```

**After (2.0)**

```toml
# Cargo.toml
[dependencies]
wgc = { version = "2" }
```

```bash
cargo run --example save_image
```

Runtime verbosity is still configured the same way, for example:

```bash
# Windows
set RUST_LOG=wgc=trace
# or, with a tracing-subscriber based binary
RUST_LOG=wgc=debug cargo run --example save_image
```

### 4. The `tutorial` example was removed

`examples/tutorial.rs` was deleted and `wgc` no longer has an example with a
`required-features` entry. To migrate:

- Read the [save_image](./examples/save_image.rs) example, which now demonstrates both
  `pixels()` (native size) and `pixels_fitted()` (letterboxed scaling).
- Read the [show_image](./examples/show_image.rs) example for a continuous capture loop.
- Consult the [DeepWiki documentation](https://deepwiki.com/Atliac/wgc) for a
  narrative walkthrough of the crate.


### Verification checklist

- [ ] Replace `read_pixels` calls with `pixels()` / `pixels_fitted(...)`.
- [ ] Rewrite `WgcSettings { .. }` literals as `WgcSettings::default()` plus field
      assignment (do **not** use `..Default::default()`).
- [ ] Remove the `tracing` feature from `Cargo.toml` and any build/CI commands.
- [ ] Bump the `wgc` dependency to `2`.
- [ ] Confirm the output buffers are still sized as expected (`width * height * bytes_per_pixel`);
      for `pixels_fitted`, that is the *fitted* size, not the native size.

### Unchanged in 2.0

The following remain source-compatible with 1.x:

- The `Wgc` iterator API and frame iteration.
- `new_item_with_picker`, `new_item_from_hwnd`, `new_item_from_monitor`.
- `WgcSettings` field names, types, default values, and field access.
- The `capabilities` module.
- `Frame::size()` and `Frame::render_time()`.
