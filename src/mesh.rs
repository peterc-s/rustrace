use std::sync::Arc;

use crate::{
    aabb::Aabb,
    bvh::BVHTree,
    hit::{HitRecord, Hittable},
    interval,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{cross, dot, Vec3},
};

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    indices: Vec<[usize; 3]>,
    normal_indices: Vec<[usize; 3]>,
    mat: Box<dyn Material>,
    bvh: BVHTree,
}

impl Mesh {
    pub fn from_obj(path: &str, mat: Box<dyn Material>) -> Self {
        todo!()
    }
}

impl Hittable for Mesh {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        // defer to mesh bvh
        self.bvh.hit(r, ray_t)
    }

    fn bound(&self) -> Aabb {
        // root bvh bounding box should encapsulate the mesh
        self.bvh.aabb
    }
}

#[derive(Debug)]
pub struct MeshTri {
    mesh: Arc<Mesh>,
    index: usize,
}

impl MeshTri {
    fn get_norm(&self, u: f64, v: f64) -> Vec3 {
        let mesh = &self.mesh;
        let [i0, i1, i2] = mesh.indices[self.index];
        let normals = [mesh.normals[i0], mesh.normals[i1], mesh.normals[i2]];
        let w = 1.0 - u - v;
        (normals[0] * w + normals[1] * u + normals[2] * v).unit()
    }
}

impl Hittable for MeshTri {
    fn hit(&self, r: &Ray, _ray_t: Interval) -> Option<HitRecord<'_>> {
        let mesh = &self.mesh;
        let [i0, i1, i2] = mesh.indices[self.index];
        let vertices = [mesh.vertices[i0], mesh.vertices[i1], mesh.vertices[i2]];

        // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
        let e1 = vertices[1] - vertices[0];
        let e2 = vertices[2] - vertices[0];
        let ray_cross_e2 = cross(&r.direction, &e2);
        let det = dot(&e1, &ray_cross_e2);

        if det > -f64::EPSILON && det < f64::EPSILON {
            return None;
        }

        let inv_det = 1. / det;
        let s = r.origin - vertices[0];
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
            let p = r.at(t);
            let mat = &(*mesh.mat);

            // TODO: front face
            let rec = HitRecord {
                p,
                norm: self.get_norm(u, v),
                mat,
                t,
                front_face: true,
            };

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

        let mesh = &self.mesh;
        let [i0, i1, i2] = mesh.indices[self.index];
        let vertices = [mesh.vertices[i0], mesh.vertices[i1], mesh.vertices[i2]];

        Aabb {
            x: min_max_axis(vertices, 0),
            y: min_max_axis(vertices, 1),
            z: min_max_axis(vertices, 2),
        }
    }
}
