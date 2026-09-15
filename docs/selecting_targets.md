# Selecting Capture Targets

`wgc` provides multiple ways to select a target (`GraphicsCaptureItem`) for screen or window capture.

## 1. Using the Interactive Picker

The interactive picker displays a native Windows UI dialog that lets the user select any open window or monitor screen.

```rust,ignore
use wgc::new_item_with_picker;

fn main() -> anyhow::Result<()> {
    // Pass None for parent window handle, or Some(parent_hwnd) to center the picker over a specific window
    let item = new_item_with_picker(None)?;
    println!("Selected target: {}", item.DisplayName()?);
    Ok(())
}
```

## 2. Target by Window Handle (`HWND`)

If you know the window handle (`HWND`) of a specific application window, you can target it directly without showing a UI picker:

```rust,ignore
use wgc::new_item_for_window;
use windows::Win32::Foundation::HWND;

fn capture_window(hwnd: HWND) -> anyhow::Result<()> {
    let item = new_item_for_window(hwnd)?;
    println!("Capturing window: {}", item.DisplayName()?);
    Ok(())
}
```

## 3. Target by Monitor Handle (`HMONITOR`)

Similarly, you can capture an entire monitor display directly by passing its `HMONITOR` handle:

```rust,ignore
use wgc::new_item_for_monitor;
use windows::Win32::Graphics::Gdi::HMONITOR;

fn capture_monitor(hmonitor: HMONITOR) -> anyhow::Result<()> {
    let item = new_item_for_monitor(hmonitor)?;
    println!("Capturing monitor: {}", item.DisplayName()?);
    Ok(())
}
```

## Target Properties

The returned `GraphicsCaptureItem` is a WinRT object. You can query its properties such as display name or size:

```rust,ignore
let name = item.DisplayName()?;
let size = item.Size()?;
println!("Target name: {}, size: {}x{}", name, size.Width, size.Height);
```
