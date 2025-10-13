use crate::{
    bvh::Aabb,
    hit::{HitRecord, Hittable},
    interval,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{dot, Vec3},
};

#[derive(Debug)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
    pub mat: Box<dyn Material>,
}

#[allow(dead_code)]
impl Sphere {
    fn new(centre: Vec3, radius: f64, mat: Box<dyn Material>) -> Sphere {
        Sphere {
            centre,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
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
        let mat = &(*self.mat);

        let mut rec = HitRecord {
            t,
            p,
            norm,
            mat,
            front_face: false,
        };

        // assert!(self.bound().ray_hit(r).is_some());

        rec.set_face_norm(r, &outward_norm);

        Some(rec)
    }

    fn bound(&self) -> Aabb {
        Aabb {
            x: interval![
                self.centre.e[0] - self.radius,
                self.centre.e[0] + self.radius
            ],
            y: interval![
                self.centre.e[1] - self.radius,
                self.centre.e[1] + self.radius
            ],
            z: interval![
                self.centre.e[2] - self.radius,
                self.centre.e[2] + self.radius
            ],
        }
    }
}
