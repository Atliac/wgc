# Selecting Capture Targets

`wgc` provides three ways to select a target (`GraphicsCaptureItem`):

## 1. Interactive Picker UI

Opens the native Windows picker dialog. Pass `None` or an optional parent window `HWND`:

```rust,ignore
use wgc::new_item_with_picker;

let item = new_item_with_picker(None)?;
println!("Selected target: {}", item.DisplayName()?);
```

## 2. Window Handle (`HWND`)

Target a specific window directly without opening a picker dialog:

```rust,ignore
use wgc::new_item_from_hwnd;
use windows::Win32::Foundation::HWND;

let item = new_item_from_hwnd(hwnd)?;
```

## 3. Monitor Handle (`HMONITOR`)

Target a specific display monitor directly:

```rust,ignore
use wgc::new_item_from_monitor;
use windows::Win32::Graphics::Gdi::HMONITOR;

let item = new_item_from_monitor(hmonitor)?;
```
