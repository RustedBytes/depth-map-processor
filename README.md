# depth-map-processor

Normalize 16-bit PNG depth maps to 8-bit grayscale with an optional Turbo
colormap visualization.

## Install

```bash
cargo install depth-map-processor
```

## CLI Usage

```bash
depth-map-processor --input depth_16bit.png --output depth_gray.png --viz depth_turbo.png
```

## Library Usage

```rust
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stats = depth_map_processor::process_depth_map(
        Path::new("depth_16bit.png"),
        Path::new("depth_gray.png"),
        Some(Path::new("depth_turbo.png")),
    )?;

    println!("{}x{}", stats.width, stats.height);
    Ok(())
}
```

## Output

- Grayscale normalization uses min/max from the 16-bit input.
- Visualization uses the Turbo colormap.
