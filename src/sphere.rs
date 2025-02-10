use crate::{hit::{HitRecord, Hittable}, interval::Interval, ray::Ray, vec3::{dot, Vec3}};

#[derive(Debug, Default, Clone, Copy)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
}

#[allow(dead_code)]
impl Sphere {
    fn new(centre: Vec3, radius: f64) -> Sphere {
        Sphere {
            centre,
            radius: radius.max(0.0),
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

        let mut rec = HitRecord::default();

        rec.t = root;
        rec.p = r.at(root);
        let outward_norm = (rec.p - self.centre) / self.radius;
        
        rec.norm = (rec.p - self.centre) / self.radius;
        rec.set_face_norm(r, &outward_norm);

        Some(rec)
    }
}
