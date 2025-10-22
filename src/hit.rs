//! This module contains the [`HitRecord`] struct and [`Hittable`] trait, used
//! when a [ray](Ray) hits a surface.

use core::fmt;

use crate::{
    aabb::Aabb,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{dot, Vec3},
};

/// A record of a [ray](Ray) hitting a surface.
#[derive(Debug)]
pub struct HitRecord<'a> {
    /// The point at which the hit occurred in world space.
    pub p: Vec3,
    /// The normal of the surface that was hit.
    pub norm: Vec3,
    /// The material of the surface that was hit.
    pub mat: &'a dyn Material,
    /// The `t` parameter of the [ray](Ray) when the hit occurred.
    pub t: f64,
    /// Whether the hit was on the front face or not.
    pub front_face: bool,
}

impl HitRecord<'_> {
    /// Set [`self.front_face`](field@HitRecord::front_face) based on the
    /// outward normal and the incident [ray](Ray).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{ray::Ray, ray, vec3::Vec3, vec3, material::Dielectric, hit::HitRecord};
    ///
    /// let mut mat = Dielectric::new(1.5);
    /// let mut rec = HitRecord {
    ///     p: vec3![1.0, 1.0, 1.0],
    ///     norm: vec3![1.0, 0.0, 0.0],
    ///     mat: &mat,
    ///     t: 1.0,
    ///     front_face: true,
    /// };
    ///
    /// rec.set_face_norm(
    ///     &ray!(
    ///         vec3![2.0, 0.0, 0.0],
    ///         vec3![-1.0, 0.0, 0.0]
    ///     ),
    ///     &vec3![-1.0, 0.0, 0.0],
    /// );
    ///
    /// // Should no longer be front facing as the ray direction and outward
    /// // norm are parallel in the same direction.
    /// assert!(!rec.front_face);
    /// ```
    pub fn set_face_norm(&mut self, r: &Ray, outward_norm: &Vec3) {
        self.front_face = dot(&r.direction, outward_norm) < 0.0;
        self.norm = if self.front_face {
            *outward_norm
        } else {
            -*outward_norm
        }
    }
}

/// This trait indicates that a struct represents something physical that can
/// be hit by a [ray](Ray) and bounded with an [`Aabb`].
pub trait Hittable: fmt::Debug + Sync + Send {
    /// Function that should be called to check if a [ray](Ray) hits the surface
    /// within the [interval](Interval) `ray_t`. Returns a [`Some(HitRecord)`](Option<HitRecord>)
    /// if a hit occurred, otherwise [`None`].
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>>;

    /// Get the bounds of a [`Hittable`] object as an [`Aabb`].
    fn bound(&self) -> Aabb;
}
