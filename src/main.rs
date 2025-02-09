use anyhow::Result;
use image::{Rgb, RgbImage};


fn main() -> Result<()> {
    const IMAGE_WIDTH: u32 = 1920;
    const IMAGE_HEIGHT: u32 = 1080;

    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64 / (IMAGE_WIDTH - 1) as f64 * 255.999) as u8;
            let g = (j as f64 / (IMAGE_HEIGHT - 1) as f64 * 255.999) as u8;
            let b = 0;

            img.put_pixel(i, j, Rgb([r, g, b]));
        }
    }

    img.save("output.png")?;

    Ok(())
}
