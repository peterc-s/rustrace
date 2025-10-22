//! This module contains the [`Material`] trait and a few materials such as
//! [`Metal`], [`Lambertian`], and [`Dielectric`].

use std::fmt::Debug;

use crate::{
    hit::HitRecord,
    ray,
    ray::Ray,
    vec3,
    vec3::{dot, Vec3},
};
use anyhow::Result;
use rand::{rngs::SmallRng, Rng};

/// This trait indicates a struct is a material and allows the material to
/// [scatter](method@Material::scatter()) incident light.
pub trait Material: Debug + Sync + Send {
    /// Scatter incident light/[ray](Ray)s according to the materials properties.
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: Option<&mut SmallRng>,
    ) -> Result<(Ray, Vec3)>;

    /// Clones a [boxed](Box) material.
    fn clone_box(&self) -> Box<dyn Material>;
}

/// A [`Lambertian`] diffuse [material](Material) with true Lambertian reflection.
/// See [wikipedia](https://en.wikipedia.org/wiki/Lambertian_reflectance).
#[derive(Copy, Clone, Debug, Default)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    /// Create a new [`Lambertian`] with the given `albedo` - the amount of each colour
    /// reflected from the surface as a [`Vec3`].
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    /// Scatter light [ray](Ray)s with true Lambertian reflectance.
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        rng: Option<&mut SmallRng>,
    ) -> Result<(Ray, Vec3)> {
        let mut scatter_dir = rec.norm + Vec3::random_unit(rng.unwrap());

        if scatter_dir.near_zero() {
            scatter_dir = rec.norm
        }

        Ok((ray![rec.p, scatter_dir], self.albedo))
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}

/// The [`Metal`] [material](Material) with perfect reflectance when made with
/// a `fuzz` of `0`.
#[derive(Copy, Clone, Debug, Default)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    /// Create a new [`Metal`] with the given `fuzz` (must be inclusively between 0 and 1).
    pub fn new(albedo: Vec3, mut fuzz: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&fuzz),
            "Fuzz may not exceed 0.0 to 0.1"
        );
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    /// Scatter light [ray](Ray)s with metal reflectance. Perfect reflectance if `fuzz`
    /// is `0`.
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: Option<&mut SmallRng>,
    ) -> Result<(Ray, Vec3)> {
        let mut reflected = r_in.direction.reflect(&rec.norm);
        reflected = reflected.unit() + (Vec3::random_unit(rng.unwrap()) * self.fuzz);
        Ok((ray![rec.p, reflected], self.albedo))
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}

/// Dielectric [material](Material) that uses Snell's law for refraction
/// and the Schlick approximation for reflectance.
#[derive(Copy, Clone, Debug, Default)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    /// Create a new [`Dielectric`] with the given refraction index.
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Dielectric {
    /// The Schlick approximation to model reflectance at near-grazing incidence.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    /// Scatter light [ray](Ray)s with refraction and reflection based on Snell's law and
    /// the Schlick approximation.
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: Option<&mut SmallRng>,
    ) -> Result<(Ray, Vec3)> {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = r_in.direction.unit();
        let cos_theta = dot(&-unit_dir, &rec.norm).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = match cannot_refract
            || Self::reflectance(cos_theta, ri) > rng.unwrap().random_range(0.0..=1.0)
        {
            true => unit_dir.reflect(&rec.norm),
            false => unit_dir.refract(&rec.norm, ri),
        };

        Ok((ray![rec.p, direction], vec3![1.0, 1.0, 1.0]))
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}
