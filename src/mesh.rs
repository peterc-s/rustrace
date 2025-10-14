use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    aabb::{Aabb, SplitAxis},
    bvh::BVHTree,
    hit::{HitRecord, Hittable},
    hit_list::HittableList,
    interval::Interval,
    material::Material,
    ray::Ray,
    triangle::Triangle,
    vec3::Vec3,
};

use crate::vec3;

use anyhow::Result;

#[derive(Debug)]
pub struct Mesh {
    bvh: BVHTree,
}

impl Mesh {
    // TODO: investigate moving elsewhere
    pub fn from_obj(path: &str, mat: Box<dyn Material>) -> Result<Self> {
        fn parse_face_vertex(s: &str) -> Result<(usize, usize)> {
            let parts: Vec<&str> = s.split("/").collect();
            let v_idx: usize = parts[0].parse::<usize>()? - 1;
            let n_idx = if parts.len() > 2 && !parts[2].is_empty() {
                parts[2].parse::<usize>()? - 1
            } else {
                v_idx
            };

            Ok((v_idx, n_idx))
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut triangles = HittableList::new();

        // read OBJ file
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            // comments and empties
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut parts = line.split_whitespace();
            let prefix = parts.next().unwrap();

            // parse lines accordingly
            match prefix {
                "v" => {
                    let x: f64 = parts.next().unwrap().parse()?;
                    let y: f64 = parts.next().unwrap().parse()?;
                    let z: f64 = parts.next().unwrap().parse()?;
                    vertices.push(vec3![x, y, z]);
                }
                "vn" => {
                    let x: f64 = parts.next().unwrap().parse()?;
                    let y: f64 = parts.next().unwrap().parse()?;
                    let z: f64 = parts.next().unwrap().parse()?;
                    normals.push(vec3![x, y, z]);
                }
                "f" => {
                    let face_verts: Vec<_> = parts.collect();
                    if face_verts.len() < 3 {
                        continue;
                    }

                    let (v0, n0) = parse_face_vertex(face_verts[0])?;
                    let (v1, n1) = parse_face_vertex(face_verts[1])?;
                    let (mut v2, mut n2) = parse_face_vertex(face_verts[2])?;

                    let tri_verts = [vertices[v0], vertices[v1], vertices[v2]];
                    let tri_normals = if !normals.is_empty() {
                        Some([normals[n0], normals[n1], normals[n2]])
                    } else {
                        None
                    };

                    triangles.add(Box::new(Triangle::new(
                        tri_verts,
                        tri_normals,
                        mat.clone_box(),
                    )));

                    // TODO: test
                    // poly -> tris
                    for i in 3..face_verts.len() {
                        let (v_new, n_new) = parse_face_vertex(face_verts[i])?;

                        let tri_verts = [vertices[v0], vertices[v2], vertices[v_new]];
                        let tri_normals = if !normals.is_empty() {
                            Some([normals[n0], normals[n1], normals[n2]])
                        } else {
                            None
                        };

                        triangles.add(Box::new(Triangle::new(
                            tri_verts,
                            tri_normals,
                            mat.clone_box(),
                        )));

                        v2 = v_new;
                        n2 = n_new;
                    }
                }
                _ => {}
            }
        }

        Ok(Self {
            bvh: BVHTree::from_hit_list(triangles, SplitAxis::Y),
        })
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
