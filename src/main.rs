use anyhow::Result;
use image::RgbImage;
use vec3::{dot, Vec3};
use ray::Ray;

mod vec3;
mod ray;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = {
    let height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    if height < 1 { 1 } else { height }
};

const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

const OUTPUT: &str = "output.png";

fn hit_sphere(centre: Vec3, radius: f64, r: Ray) -> Option<f64> {
    let oc = centre - r.origin;
    let a = dot(&r.direction, &r.direction);
    let b = -2.0 * dot(&r.direction, &oc);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }

}

fn ray_colour(r: &Ray) -> Vec3 {
    let hit = hit_sphere(vec3![0.0, 0.0, -1.0], 0.5, *r);
    match hit {
        Some(t) => {
            let norm = (r.at(t) - vec3![0.0, 0.0, -1.0]).unit();
            return vec3![norm[0] + 1.0, norm[1] + 1.0, norm[2] + 1.0] * 0.5;
        },
        None => {}
    }

    let unit_direction = r.direction.unit();
    let a = 0.5 * (unit_direction[1] + 1.0);
    vec3![1.0, 1.0, 1.0] * (1.0 - a) + vec3![0.5, 0.7, 1.0] * a
}

fn main() -> Result<()> {
    let camera_centre = vec3![0.0, 0.0, 0.0];
    let viewport_u = vec3![VIEWPORT_WIDTH, 0.0, 0.0];
    let viewport_v = vec3![0.0, -VIEWPORT_HEIGHT, 0.0];
    
    let pixel_delta_u = viewport_u / IMAGE_WIDTH;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT;

    let viewport_upper_left = camera_centre
                            - vec3![0.0, 0.0, FOCAL_LENGTH]
                            - viewport_u / 2
                            - viewport_v / 2;

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;


    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rLines: {}/{IMAGE_HEIGHT}", j + 1);
        for i in 0..IMAGE_WIDTH {
            let pixel_centre = pixel00_loc + (pixel_delta_u * i) + (pixel_delta_v * j);
            let ray_direction = pixel_centre - camera_centre;
            let r = ray!(camera_centre, ray_direction);
            let pixel_colour = ray_colour(&r).to_rgb();

            img.put_pixel(i, j, pixel_colour);
        }
    }

    eprintln!("\nSaving...");
    img.save(OUTPUT)?;
    eprintln!("Saved to {}!", OUTPUT);

    Ok(())
}
