use std::fmt::Debug;

use crate::{hit::HitRecord, ray::Ray, ray, vec3::Vec3};
use anyhow::Result;
use rand::rngs::SmallRng;

pub trait Material: Debug + Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: Option<&mut SmallRng>) -> Result<(Ray, Vec3)>;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, rng: Option<&mut SmallRng>) -> Result<(Ray, Vec3)> {
        let mut scatter_dir = rec.norm + Vec3::random_unit(rng.unwrap());

        if scatter_dir.near_zero() {
            scatter_dir = rec.norm
        }

        Ok((ray![rec.p, scatter_dir], self.albedo))
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, mut fuzz: f64) -> Self {
        fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: Option<&mut SmallRng>) -> Result<(Ray, Vec3)> {
        let mut reflected = r_in.direction.reflect(&rec.norm);
        reflected = reflected.unit() + (Vec3::random_unit(rng.unwrap()) * self.fuzz);
        Ok((ray![rec.p, reflected], self.albedo))
    }
}
