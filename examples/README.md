# Examples

This folder contains ready-to-run usage samples for the CLI and the library.

## CLI

```bash
depth-map-processor --input depth_16bit.png --output depth_gray.png --viz depth_turbo.png
```

## Library

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
