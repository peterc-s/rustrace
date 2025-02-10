use std::fmt::Debug;

use crate::{hit::HitRecord, ray::Ray, ray, vec3::Vec3};
use anyhow::Result;
use rand::rngs::SmallRng;

pub trait Material: Debug + Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: Option<&mut SmallRng>) -> Result<(Ray, Vec3)>;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Lambertian {
    pub albedo: Vec3,
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
        let scatter_dir = rec.norm + Vec3::random_unit(rng.unwrap());
        Ok((ray![rec.p, scatter_dir], self.albedo))
    }
}
