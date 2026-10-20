# Capturing Frames

`Wgc` implements `Iterator<Item = Result<Frame, WgcError>>`.

## Frame Properties

- `frame.size()`: Returns `FrameSize { width, height }` of the captured frame in pixels.
- `frame.render_time()`: Returns the calculated render timestamp as `std::time::Instant`.
- `frame.dirty_regions()`: Returns a `Vec<DirtyRegion>` of the rectangular areas that changed since the previous frame.

## Accessing Pixels

### 1. Native Size Pixels (`pixels`)

Reads the raw pixel buffer at the frame's native resolution. Buffer size is `width * height * bytes_per_pixel`.

```rust,ignore
let pixels: Vec<u8> = frame.pixels()?;
```

### 2. Resolution-Fitted Pixels (`pixels_fitted`)

Scales the frame to fit a target `FrameSize` while preserving aspect ratio. Any remaining space is letterboxed with gray borders. This is ideal for Machine Learning (e.g. YOLO/ResNet) and computer vision pipelines that require constant input dimensions.

```rust,ignore
let target = FrameSize { width: 512, height: 512 };
let fitted_pixels: Vec<u8> = frame.pixels_fitted(target)?;
```

To see this in action, run:
```bash
cargo run --example save_image
```
