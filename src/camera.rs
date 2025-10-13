use std::f64::INFINITY;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use image::RgbImage;
use rand::SeedableRng;
use rand::{rngs::SmallRng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::hit::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::deg_to_rad;
use crate::vec3::{cross, Vec3};
use crate::{interval, ray, vec3};

#[derive(Debug, Clone, Copy)]
pub enum AntiAliasing {
    Grid(u16),
    Random(u16),
}

trait AntiAliasingGrid {
    fn sample_grid(self, sample: u16) -> Result<Vec3>;
    fn get_ray_grid(self, i: u32, j: u32, sample: u16, rng: &mut SmallRng) -> Result<Ray>;
}

trait AntiAliasingRandom {
    fn sample_random(self, rng: &mut SmallRng) -> Result<Vec3>;
    fn get_ray_random(self, i: u32, j: u32, rng: &mut SmallRng) -> Result<Ray>;
}

trait Defocus {
    fn defocus_disc_sample(&self, rng: &mut SmallRng) -> Vec3;
}

#[derive(Debug, Clone, Copy)]
pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: u32,
    anti_aliasing: AntiAliasing,
    max_depth: u32,
    vfov: u16,
    look_from: Vec3,
    look_at: Vec3,
    v_up: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        CameraBuilder {
            aspect_ratio: 1.0,
            image_width: 100,
            anti_aliasing: AntiAliasing::Grid(4),
            max_depth: 10,
            vfov: 90,
            look_from: vec3![0.0, 0.0, 0.0],
            look_at: vec3![0.0, 0.0, -1.0],
            v_up: vec3![0.0, 1.0, 0.0],
            defocus_angle: 0.0,
            focus_dist: 10.0,
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

    pub fn set_anti_aliasing(self, anti_aliasing: AntiAliasing) -> CameraBuilder {
        CameraBuilder {
            anti_aliasing,
            ..self
        }
    }

    pub fn set_max_depth(self, max_depth: u32) -> CameraBuilder {
        CameraBuilder { max_depth, ..self }
    }

    pub fn set_vfov(self, vfov: u16) -> CameraBuilder {
        CameraBuilder { vfov, ..self }
    }

    pub fn set_look_from(self, look_from: Vec3) -> CameraBuilder {
        CameraBuilder { look_from, ..self }
    }

    pub fn set_look_at(self, look_at: Vec3) -> CameraBuilder {
        CameraBuilder { look_at, ..self }
    }

    pub fn set_v_up(self, v_up: Vec3) -> CameraBuilder {
        CameraBuilder { v_up, ..self }
    }

    pub fn set_defocus_angle(self, defocus_angle: f64) -> CameraBuilder {
        CameraBuilder {
            defocus_angle,
            ..self
        }
    }

    pub fn set_focus_dist(self, focus_dist: f64) -> CameraBuilder {
        CameraBuilder { focus_dist, ..self }
    }

    pub fn build(self) -> Camera {
        let mut image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let samples_scale = match self.anti_aliasing {
            AntiAliasing::Grid(size) => 1.0 / (size.pow(2) as f64),
            AntiAliasing::Random(number) => 1.0 / (number as f64),
        };

        let centre = self.look_from;

        let theta = deg_to_rad(self.vfov as f64);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        let w = (self.look_from - self.look_at).unit();
        let u = (cross(&self.v_up, &w)).unit();
        let v = cross(&w, &u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / self.image_width;
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upper_left = centre - (w * self.focus_dist) - viewport_u / 2 - viewport_v / 2;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_rad = self.focus_dist * deg_to_rad(self.defocus_angle / 2.0).tan();
        let defocus_disc_u = u * defocus_rad;
        let defocus_disc_v = v * defocus_rad;

        Camera {
            // aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            anti_aliasing: self.anti_aliasing,
            max_depth: self.max_depth,
            defocus_angle: self.defocus_angle,
            image_height,
            samples_scale,
            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            // u,
            // v,
            // w,
            defocus_disc_u,
            defocus_disc_v,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    // pub aspect_ratio: f64,
    pub anti_aliasing: AntiAliasing,
    pub image_width: u32,
    image_height: u32,
    samples_scale: f64,
    max_depth: u32,
    centre: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    // u: Vec3,
    // v: Vec3,
    // w: Vec3,
    defocus_angle: f64,
    defocus_disc_u: Vec3,
    defocus_disc_v: Vec3,
}

impl AntiAliasingGrid for Camera {
    fn sample_grid(self, sample: u16) -> Result<Vec3> {
        if let AntiAliasing::Grid(size) = self.anti_aliasing {
            let grid_size = size as f64;
            Ok(vec3![
                0.5 - (sample % size) as f64 / (grid_size - 1.0),
                0.5 - (sample / size) as f64 / (grid_size - 1.0),
                0.0
            ])
        } else {
            Err(anyhow!(
                "Sample grid called when AntiAliasing mode is not Grid."
            ))
        }
    }

    fn get_ray_grid(self, i: u32, j: u32, sample: u16, rng: &mut SmallRng) -> Result<Ray> {
        let offset = self.sample_grid(sample)?;
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset[0]))
            + (self.pixel_delta_v * (j as f64 + offset[1]));

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.centre
        } else {
            self.defocus_disc_sample(rng)
        };

        let ray_direction = pixel_sample - ray_origin;

        Ok(ray!(ray_origin, ray_direction))
    }
}

impl AntiAliasingRandom for Camera {
    fn sample_random(self, rng: &mut SmallRng) -> Result<Vec3> {
        match self.anti_aliasing {
            AntiAliasing::Random(_) => {}
            _ => {
                return Err(anyhow!(
                    "Sample random called when AntiAliasing mode is not Random."
                ))
            }
        }

        Ok(vec3![
            rng.random_range(-0.5..=0.5),
            rng.random_range(-0.5..=0.5),
            0.0
        ])
    }

    fn get_ray_random(self, i: u32, j: u32, rng: &mut SmallRng) -> Result<Ray> {
        let offset = self.sample_random(rng)?;
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset[0]))
            + (self.pixel_delta_v * (j as f64 + offset[1]));

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.centre
        } else {
            self.defocus_disc_sample(rng)
        };

        let ray_direction = pixel_sample - ray_origin;

        Ok(ray!(ray_origin, ray_direction))
    }
}

impl Defocus for Camera {
    fn defocus_disc_sample(&self, rng: &mut SmallRng) -> Vec3 {
        let p = Vec3::random_in_unit_disc(rng);
        self.centre + (self.defocus_disc_u * p[0]) + (self.defocus_disc_v * p[1])
    }
}

impl Camera {
    fn ray_colour(r: &Ray, depth: u32, world: &dyn Hittable, rng: &mut SmallRng) -> Vec3 {
        if depth <= 0 {
            return vec3![0.0, 0.0, 0.0];
        }

        if let Some(rec) = world.hit(r, interval![0.001, INFINITY]) {
            if let Ok((scattered, attenuation)) = rec.mat.scatter(r, &rec, Some(rng)) {
                return attenuation * Camera::ray_colour(&scattered, depth - 1, world, rng);
            }
            return vec3![0.0, 0.0, 0.0];
        }

        let unit_dir = r.direction.unit();
        let a = (unit_dir[1] + 1.0) * 0.5;
        vec3![1.0, 1.0, 1.0] * (1.0 - a) + vec3![0.5, 0.7, 1.0] * a
    }

    pub fn render(self, output: &str, world: &dyn Hittable) -> Result<()> {
        let img = Arc::new(Mutex::new(RgbImage::new(
            self.image_width,
            self.image_height,
        )));
        let lines_done = Arc::new(AtomicUsize::new(0));

        (0..self.image_height).into_par_iter().for_each(|j| {
            let mut rng = SmallRng::from_os_rng();
            let mut row = vec![];
            for i in 0..self.image_width {
                let mut pixel_colour = vec3![0.0, 0.0, 0.0];

                match self.anti_aliasing {
                    AntiAliasing::Grid(size) => {
                        for sample in 0..size.pow(2) {
                            let r = self.get_ray_grid(i, j, sample, &mut rng).unwrap();
                            pixel_colour += Camera::ray_colour(&r, self.max_depth, world, &mut rng);
                        }
                    }
                    AntiAliasing::Random(number) => {
                        for _ in 0..number {
                            let r = self.get_ray_random(i, j, &mut rng).unwrap();
                            pixel_colour += Camera::ray_colour(&r, self.max_depth, world, &mut rng);
                        }
                    }
                }

                row.push((pixel_colour * self.samples_scale).to_rgb());
            }

            eprint!(
                "\rLines: {}/{}",
                lines_done.load(Ordering::SeqCst) + 1,
                self.image_height
            );
            let mut img = img.lock().unwrap();
            for (i, pixel) in row.into_iter().enumerate() {
                img.put_pixel(i as u32, j, pixel);
            }

            lines_done.fetch_add(1, Ordering::SeqCst);
        });

        eprintln!("\nSaving...");
        {
            let img = img.lock().unwrap();
            img.save(output)?;
        }
        eprintln!("Saved to {output}!");

        Ok(())
    }
}
