use image::{ImageBuffer, Luma};
use std::path::Path;

fn generate_depth_map(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let width = 64;
    let height = 64;
    let mut img: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::new(width, height);
    let max_den = (width + height - 2) as u32;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let val = ((x + y) as u32 * 10_000 / max_den) as u16;
        *pixel = Luma([val]);
    }

    img.save(path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Path::new("depth_16bit.png");
    if !input.exists() {
        generate_depth_map(input)?;
    }

    let stats = depth_map_processor::process_depth_map(
        input,
        Path::new("depth_gray.png"),
        Some(Path::new("depth_turbo.png")),
    )?;

    println!("{}x{}", stats.width, stats.height);
    Ok(())
}
