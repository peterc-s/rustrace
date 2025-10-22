use crate::{
    aabb::Aabb,
    hit::{HitRecord, Hittable},
    interval,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{cross, dot, Vec3},
};

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
    pub normals: [Vec3; 3],
    pub mat: Box<dyn Material>,
}

impl Triangle {
    pub fn new(vertices: [Vec3; 3], normals: Option<[Vec3; 3]>, mat: Box<dyn Material>) -> Self {
        let normals = match normals {
            Some(n) => n,
            None => {
                let e1 = vertices[1] - vertices[0];
                let e2 = vertices[2] - vertices[1];
                let n = cross(&e1, &e2);
                [n, n, n]
            }
        };

        Self {
            vertices,
            normals,
            mat,
        }
    }

    fn get_norm(&self, u: f64, v: f64) -> Vec3 {
        let w = 1.0 - u - v;
        (self.normals[0] * w + self.normals[1] * u + self.normals[2] * v).unit()
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, _ray_t: Interval) -> Option<HitRecord<'_>> {
        // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
        let e1 = self.vertices[1] - self.vertices[0];
        let e2 = self.vertices[2] - self.vertices[0];
        let ray_cross_e2 = cross(&r.direction, &e2);
        let det = dot(&e1, &ray_cross_e2);

        if det > -f64::EPSILON && det < f64::EPSILON {
            return None;
        }

        let inv_det = 1. / det;
        let s = r.origin - self.vertices[0];
        let u = dot(&s, &ray_cross_e2) * inv_det;
        if !(0. ..=1.).contains(&u) {
            return None;
        }

        let s_cross_e1 = cross(&s, &e1);
        let v = inv_det * dot(&r.direction, &s_cross_e1);
        if v < 0. || u + v > 1. {
            return None;
        }

        let t = inv_det * dot(&e2, &s_cross_e1);
        if t > f64::EPSILON {
            let p = r.at(t - f64::EPSILON);
            let mat = &(*self.mat);

            let mut rec = HitRecord {
                p,
                norm: self.get_norm(u, v),
                mat,
                t,
                front_face: true,
            };

            rec.set_face_norm(r, &self.get_norm(u, v));

            Some(rec)
        } else {
            None
        }
    }

    fn bound(&self) -> Aabb {
        fn min_max_axis(vertices: [Vec3; 3], axis: usize) -> Interval {
            let mut iter = vertices.iter().map(|v| v[axis]);
            let first = iter.next().expect("No vertices.");
            let (min, max) = iter.fold((first, first), |(min, max), val| {
                (min.min(val), max.max(val))
            });
            interval![min, max]
        }

        Aabb {
            x: min_max_axis(self.vertices, 0),
            y: min_max_axis(self.vertices, 1),
            z: min_max_axis(self.vertices, 2),
        }
    }
}
