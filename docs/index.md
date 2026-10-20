# Introduction

**`wgc`** is a simple, ergonomic, and high-performance Rust wrapper for the **Windows Graphics Capture (WGC)** API (`Windows.Graphics.Capture`), enabling screen and window capture on Windows 10 and 11.

## Quick Tour via Examples

The fastest way to learn `wgc` is to clone the repository and run the included examples:

```bash
git clone https://github.com/Atliac/wgc.git
cd wgc

# 1. Real-time window/screen viewer with dirty region highlighting
cargo run --example show_image

# 2. View only dirty (changed) regions in ReportAndRender mode
cargo run --example show_dirty

# 3. Capture a frame and save both native and letterboxed PNGs
cargo run --example save_image

# 4. Inspect system capabilities supported on your Windows version
cargo run --example capabilities
```

## Key Highlights

- **Iterator-based API**: Capture frames sequentially using standard Rust iterator patterns via `Wgc`.
- **Target Selection**: Select targets interactively with the native Windows picker or directly by window handle (`HWND`) or monitor handle (`HMONITOR`).
- **AI & ML Friendly**: Automatic aspect-ratio preserving letterbox scaling via `frame.pixels_fitted(desired_size)`.
- **Dirty Region Support**: Inspect sub-frame regions that changed between consecutive frames via `frame.dirty_regions()`.
- **Runtime Capability Checks**: Query feature availability on the host Windows system at runtime via the `capabilities` module.
