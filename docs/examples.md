# Examples & Practical Use

The repository includes ready-to-run examples in the [`examples/`](https://github.com/Atliac/wgc/tree/master/examples) directory.

Clone the repository to run them locally:

```bash
git clone https://github.com/Atliac/wgc.git
cd wgc
```

---

## 1. Live Screen Viewer with Dirty Regions (`show_image`)

Captures a window or screen and displays it in a live GUI window. On supported Windows 11 versions, it automatically highlights changed regions in real-time with green bounding boxes.

```bash
cargo run --example show_image
```

Source: [`examples/show_image.rs`](https://github.com/Atliac/wgc/blob/master/examples/show_image.rs)

---

## 2. Dirty-Region Only Viewer (`show_dirty`)

Demonstrates `ReportAndRender` mode (`dirty_region_mode = Some(true)`), where Windows DWM only renders the sub-areas of the screen that changed.

```bash
cargo run --example show_dirty
```

Source: [`examples/show_dirty.rs`](https://github.com/Atliac/wgc/blob/master/examples/show_dirty.rs)

---

## 3. Save Captured Images to PNG (`save_image`)

Captures a frame and saves two image files to disk:
- `target/native.png`: Captured frame at full native resolution.
- `target/fitted.png`: Scaled and letterboxed to 512×512 (ideal for machine learning inputs).

```bash
cargo run --example save_image
```

Source: [`examples/save_image.rs`](https://github.com/Atliac/wgc/blob/master/examples/save_image.rs)

---

## 4. Check Windows System Capabilities (`capabilities`)

Queries which Windows Graphics Capture features are supported on the current machine and Windows OS build.

```bash
cargo run --example capabilities
```

Source: [`examples/capabilities.rs`](https://github.com/Atliac/wgc/blob/master/examples/capabilities.rs)
