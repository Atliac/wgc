# Capturing Frames

`Wgc` is an iterator over captured frames. When iterating over `Wgc`, each step yields a `Result<Frame, WgcError>`.

## The `Frame` Type

Each `Frame` provides information about the captured frame and methods for extracting raw pixel data or accessing the underlying Direct3D surface texture.

### Frame Properties

- `frame.size()`: Returns `FrameSize { width, height }` of the captured frame in pixels.
- `frame.system_relative_time()`: Returns the capture timestamp (`Duration` since system startup via QueryPerformanceCounter).

### Accessing Pixels

#### 1. Native Size Pixels (`pixels`)

Reads the raw pixel buffer at the captured frame's native resolution.

```rust,ignore
use wgc::*;

fn main() -> anyhow::Result<()> {
    let item = new_item_with_picker(None)?;
    let wgc = Wgc::new(item, Default::default())?;

    for frame in wgc.take(1) {
        let frame = frame?;
        let size = frame.size()?;
        let pixels: Vec<u8> = frame.pixels()?;

        println!("Read {} bytes (width: {}, height: {})", pixels.len(), size.width, size.height);
    }
    Ok(())
}
```

#### 2. Resolution-Fitted Pixels (`pixels_fitted`)

Scales the frame to fit a target `FrameSize` while preserving aspect ratio. Any remaining space is letterboxed with gray borders. This is ideal for Machine Learning (e.g. YOLO/ResNet) and computer vision pipelines that require constant input dimensions.

```rust,ignore
use wgc::*;

fn main() -> anyhow::Result<()> {
    let item = new_item_with_picker(None)?;
    let wgc = Wgc::new(item, Default::default())?;
    let target_size = FrameSize { width: 512, height: 512 };

    for frame in wgc.take(1) {
        let frame = frame?;
        let fitted_pixels: Vec<u8> = frame.pixels_fitted(target_size)?;

        // Guaranteed buffer length: width * height * 4 (RGBA8/BGRA8)
        assert_eq!(fitted_pixels.len(), (512 * 512 * 4) as usize);
    }
    Ok(())
}
```

### Direct3D 11 Surface Access (Zero-Copy)

For low-latency GPU workflows (e.g., Direct3D rendering, video encoding with NVENC/AMF, or Direct2D drawing), you can access the underlying `ID3D11Texture2D` texture directly:

```rust,ignore
use wgc::*;

fn main() -> anyhow::Result<()> {
    let item = new_item_with_picker(None)?;
    let wgc = Wgc::new(item, Default::default())?;

    for frame in wgc.take(1) {
        let frame = frame?;

        // Direct3D 11 surface access
        let surface = frame.surface()?; // Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface
        let texture = frame.texture()?; // windows::Win32::Graphics::Direct3D11::ID3D11Texture2D
    }
    Ok(())
}
```
