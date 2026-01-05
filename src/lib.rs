use colorgrad::Gradient;
use image::{ImageBuffer, Luma, RgbImage};
use std::path::Path;

/// Processing stats from a depth-map normalization run.
pub struct DepthMapStats {
    pub width: u32,
    pub height: u32,
    pub min_val: u16,
    pub max_val: u16,
    pub sample_depth_m: f32,
}

/// Normalize a 16-bit depth map to 8-bit grayscale and optionally save a
/// Turbo colormap visualization.
pub fn process_depth_map(
    input_path: &Path,
    output_path: &Path,
    viz_path: Option<&Path>,
) -> Result<DepthMapStats, Box<dyn std::error::Error>> {
    // The image crate automatically handles formats.
    // .into_luma16() ensures we treat it as a single-channel 16-bit image (uint16).
    let img = image::open(input_path)
        .map_err(|_| {
            format!(
                "Could not open or find the image at: {}",
                input_path.display()
            )
        })?
        .into_luma16();

    let (width, height) = img.dimensions();

    // Accessing pixel at (0, 0). Note: image crate uses (x, y) indexing.
    let pixel_val = img.get_pixel(0, 0)[0];
    let depth_in_meters = pixel_val as f32 / 1000.0;

    // Normalization Logic (cv2.normalize with NORM_MINMAX)
    // First, find min and max values in the 16-bit buffer
    let (min_val, max_val) = img.pixels().fold((u16::MAX, u16::MIN), |(min, max), p| {
        (min.min(p[0]), max.max(p[0]))
    });

    // Create a new 8-bit grayscale image buffer
    let mut norm_img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Apply normalization: (val - min) / (max - min) * 255
    let range = (max_val - min_val) as f32;

    for (x, y, pixel) in img.enumerate_pixels() {
        let val = pixel[0];
        let normalized = if range > 0.0 {
            ((val as f32 - min_val as f32) / range * 255.0) as u8
        } else {
            0
        };
        norm_img.put_pixel(x, y, Luma([normalized]));
    }

    // Apply a colormap (mimicking cv2.applyColorMap(..., cv2.COLORMAP_TURBO))
    let grad = colorgrad::preset::turbo();
    let mut colored_img: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in norm_img.enumerate_pixels() {
        // colorgrad expects a float between 0.0 and 1.0
        let t = pixel[0] as f32 / 255.0f32;
        let rgba = grad.at(t).to_rgba8();
        colored_img.put_pixel(x, y, image::Rgb([rgba[0], rgba[1], rgba[2]]));
    }

    // Save to file (Visualization)
    if let Some(path) = viz_path {
        colored_img.save(path)?;
    }

    // Save to file with grayscale normalization
    norm_img.save(output_path)?;

    Ok(DepthMapStats {
        width,
        height,
        min_val,
        max_val,
        sample_depth_m: depth_in_meters,
    })
}
