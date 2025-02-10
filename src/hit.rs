use core::fmt;

use crate::{interval::Interval, ray::Ray, vec3::{dot, Vec3}};

#[derive(Debug, Copy, Clone, Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub norm: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_norm(mut self, r: &Ray, outward_norm: &Vec3) {
        self.front_face = dot(&r.direction, outward_norm) < 0.0;
        self.norm = if self.front_face { *outward_norm } else { -*outward_norm }
    }
}

pub trait Hittable: fmt::Debug + Sync + Send {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
