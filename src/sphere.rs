use std::sync::Arc;

use crate::{
    hit::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{dot, Vec3},
};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

#[allow(dead_code)]
impl Sphere {
    fn new(centre: Vec3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            centre,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.centre - r.origin;
        let a = r.direction.length_squared();
        let h = dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(root);
        let outward_norm = (p - self.centre) / self.radius;
        let norm = (p - self.centre) / self.radius;
        let mat = self.mat.clone();

        let mut rec = HitRecord {
            t,
            p,
            norm,
            mat,
            front_face: false,
        };

        rec.set_face_norm(r, &outward_norm);

        Some(rec)
    }
}
