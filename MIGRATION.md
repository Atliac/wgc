# Migration Guide

This document describes breaking API changes and how to update downstream code.

- [v2.x → v3.0](#v2x--v30)
- [v1.x → v2.0](#v1x--v20)

## v2.x → v3.0

### Summary of breaking changes

| # | Change | What you need to do |
|---|--------|---------------------|
| 1 | `Wgc::new(item, settings)` removed in favor of `Wgc::new_from_*().<config>().build()?` | Use `Wgc::new_from_picker`, `new_from_hwnd`, `new_from_monitor`, or `new_from_item` and call `.build()?` |
| 2 | `WgcSettings` struct is now crate-internal (`pub(crate)`) | Configure options using builder methods (`.capture_cursor(bool)`, `.dirty_region_mode(bool)`, etc.) |
| 3 | Target acquisition functions (`new_item_*`) are now internal | Call `Wgc::new_from_*` directly instead of manually creating a `GraphicsCaptureItem` |
| 4 | `Frame::new` is now crate-internal (`pub(crate)`) | Obtain `Frame` instances by iterating over `Wgc` |
| 5 | External mdBook documentation removed | Refer to crate rustdoc on [docs.rs](https://docs.rs/wgc) or the Quick Start guide in `lib.rs` |

---

### 1. `Wgc::new(item, settings)` → `Wgc::new_from_*` Builder API

The `Wgc::new(item, settings)` constructor has been replaced by a unified, fluent builder pattern. Target selection and capture configuration are now consolidated into builder entry points.

**Before (2.x)**

```rust
use wgc::*;

// 1. Manually obtain capture item
let item = new_item_with_picker(None)?;

// 2. Configure settings struct
let mut settings = WgcSettings::default();
settings.capture_cursor = Some(false);
settings.dirty_region_mode = Some(true);

// 3. Initialize capture
let wgc = Wgc::new(item, settings)?;
```

**After (3.0)**

```rust
use wgc::*;

let wgc = Wgc::new_from_picker(None)?
    .capture_cursor(false)
    .dirty_region_mode(true)
    .build()?;
```

If you already have a `GraphicsCaptureItem` (e.g., from an external interop call), use `Wgc::new_from_item`:

```rust
let wgc = Wgc::new_from_item(item)
    .capture_cursor(false)
    .build()?;
```

Constructor mapping:

| 2.x | 3.0 |
|---|---|
| `new_item_with_picker(owner)` + `Wgc::new(item, ..)` | `Wgc::new_from_picker(owner)?.<config>().build()?` |
| `new_item_from_hwnd(hwnd)` + `Wgc::new(item, ..)` | `Wgc::new_from_hwnd(hwnd)?.<config>().build()?` |
| `new_item_from_monitor(hmonitor)` + `Wgc::new(item, ..)` | `Wgc::new_from_monitor(hmonitor)?.<config>().build()?` |
| `Wgc::new(item, settings)` | `Wgc::new_from_item(item).<config>().build()?` |

---

### 2. `WgcSettings` Removed From Public API

`WgcSettings` was previously `#[non_exhaustive]` and configured via mutable field assignment. In 3.0, it is internal. All configuration is done via builder methods taking concrete types:

| 2.x Field Assignment | 3.0 Builder Method |
|---|---|
| `settings.pixel_format = format` | `.pixel_format(format)` |
| `settings.frame_queue_length = n` | `.frame_queue_length(n)` |
| `settings.capture_cursor = Some(bool)` | `.capture_cursor(bool)` |
| `settings.display_border = Some(bool)` | `.display_border(bool)` |
| `settings.include_secondary_windows = Some(bool)` | `.include_secondary_windows(bool)` |
| `settings.dirty_region_mode = Some(bool)` | `.dirty_region_mode(bool)` |
| `settings.min_update_interval = Some(duration)` | `.min_update_interval(duration)` |
| `settings.frame_interpolation_mode = mode` | `.frame_interpolation_mode(mode)` |

If you do not call an optional configuration method, it defaults to the Windows OS default behavior (equivalent to `None` in 2.x).

---

### 3. Target Acquisition Helpers (`new_item_*`) Are Internal

`new_item_with_picker`, `new_item_from_hwnd`, and `new_item_from_monitor` are no longer exported by `wgc`. Instead, use `Wgc::new_from_*` directly.

If you need to access properties of the captured target (e.g. `item.DisplayName()`), call `.item()` on the `Wgc` or `WgcBuilder` instance:

```rust
let wgc = Wgc::new_from_picker(None)?.build()?;
let title = wgc.item().DisplayName()?;
```

---

### Verification checklist (v3.0)

- [ ] Replace `Wgc::new(...)` with `Wgc::new_from_picker`, `new_from_hwnd`, `new_from_monitor`, or `new_from_item` followed by `.build()?`.
- [ ] Remove all references to `WgcSettings` and chained assignments; use builder methods instead.
- [ ] Replace calls to standalone `new_item_with_picker`, `new_item_from_hwnd`, and `new_item_from_monitor` with the corresponding `Wgc::new_from_*` methods.
- [ ] Replace `item.DisplayName()` calls with `wgc.item().DisplayName()`.
- [ ] Bump the `wgc` dependency to `3`.

---

## v1.x → v2.0

### Summary of breaking changes

| # | Change | What you need to do |
|---|--------|---------------------|
| 1 | `Frame::read_pixels(Option<FrameSize>)` split into `Frame::pixels()` and `Frame::pixels_fitted(FrameSize)` | Rename calls (see [below](#1-frameread_pixels--framepixels--framepixels_fitted)) |
| 2 | `WgcSettings` is now `#[non_exhaustive]` | Build it from `WgcSettings::default()` and assign fields (see [below](#2-wgcsettings-is-now-non_exhaustive)) |
| 3 | The `tracing` Cargo feature was removed; `tracing` is a required dependency | Drop `--features tracing` and `features = ["tracing"]` |
| 4 | The `tutorial` example was removed | Use the [examples](./examples/) |

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
