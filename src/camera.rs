use std::f64::INFINITY;

use image::RgbImage;
use anyhow::Result;

use crate::hit::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::interval::Interval;
use crate::{interval, vec3, ray};

#[derive(Debug, Clone, Copy)]
pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: u32,
    sample_grid_size: u8,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        CameraBuilder {
            aspect_ratio: 1.0,
            image_width: 100,
            sample_grid_size: 4,
        }
    }
}

#[allow(dead_code)]
impl CameraBuilder {
    pub fn set_aspect_ratio(self, aspect_ratio: f64) -> CameraBuilder {
        CameraBuilder {
            aspect_ratio,
            ..self
        }
    }

    pub fn set_image_width(self, image_width: u32) -> CameraBuilder {
        CameraBuilder {
            image_width,
            ..self
        }
    }

    pub fn set_sample_grid_size(self, sample_grid_size: u8) -> CameraBuilder {
        CameraBuilder {
            sample_grid_size,
            ..self
        }
    }

    pub fn build(self) -> Camera {
        let image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        let centre = vec3![0.0, 0.0, 0.0];
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        let viewport_u = vec3![viewport_width, 0.0, 0.0];
        let viewport_v = vec3![0.0, -viewport_height, 0.0];

        let pixel_delta_u = viewport_u / self.image_width;
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upper_left = centre
                                - vec3![0.0, 0.0, focal_length]
                                - viewport_u / 2
                                - viewport_v / 2;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let samples_scale = 1.0 / (self.sample_grid_size.pow(2) as f64);

        Camera {
            // aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            sample_grid_size: self.sample_grid_size,
            image_height,
            samples_scale,
            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }

    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    // pub aspect_ratio: f64,
    pub image_width: u32,
    pub sample_grid_size: u8,
    image_height: u32,
    samples_scale: f64,
    centre: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    fn ray_colour(r: &Ray, world: &dyn Hittable) -> Vec3 {
        if let Some(rec) = world.hit(r, interval![0.0, INFINITY]) {
            return (rec.norm + vec3![1.0, 1.0, 1.0]) * 0.5;
        }

        let unit_dir = r.direction.unit();
        let a = (unit_dir[1] + 1.0) * 0.5;
        vec3![1.0, 1.0, 1.0] * (1.0 - a) + vec3![0.5, 0.7, 1.0] * a
    }

    fn sample_grid(self, sample: u16) -> Vec3 {
        let grid_size = self.sample_grid_size as f64;
        return vec3![
            0.5 - (sample % self.sample_grid_size as u16) as f64 / (grid_size - 1.0),
            0.5 - (sample / self.sample_grid_size as u16) as f64 / (grid_size - 1.0),
            0.0
        ]
    }

    fn get_ray(self, i: u32, j: u32, sample: u16) -> Ray {
        let offset = self.sample_grid(sample);
        let pixel_sample = self.pixel00_loc
                         + (self.pixel_delta_u * (i as f64 + offset[0]))
                         + (self.pixel_delta_v * (j as f64 + offset[1]));

        let ray_direction = pixel_sample - self.centre;

        ray!(self.centre, ray_direction)
    }

    pub fn render(self, output: &str, world: &dyn Hittable) -> Result<()> {
        let mut img = RgbImage::new(self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rLines: {}/{}", j + 1, self.image_height);
            for i in 0..self.image_width {
                let mut pixel_colour = vec3![0.0, 0.0, 0.0];
                for sample in 0..self.sample_grid_size.pow(2) as u16 {
                    let r = self.get_ray(i, j, sample);
                    pixel_colour += Camera::ray_colour(&r, world);
                }

                img.put_pixel(i, j, (pixel_colour * self.samples_scale).to_rgb());
            }
        }

        eprintln!("\nSaving...");
        img.save(output)?;
        eprintln!("Saved to {}!", output);

        Ok(())
    }
}
