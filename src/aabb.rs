use crate::{
    interval::Interval,
    ray::Ray,
    vec3,
    vec3::{dot, Vec3},
};

#[derive(Debug, Clone, Copy)]
pub enum SplitAxis {
    X,
    Y,
    Z,
}

impl SplitAxis {
    pub fn next(self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::Z,
            Self::Z => Self::X,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn union(&mut self, other: &Self) {
        self.x.union(&other.x);
        self.y.union(&other.y);
        self.z.union(&other.z);
    }

    // DEBUG: only used in BVHTree::verify()
    // pub fn surrounds(self, other: &Self) -> bool {
    //     self.x.contains_interval(&other.x)
    //         && self.y.contains_interval(&other.y)
    //         && self.z.contains_interval(&other.z)
    // }

    pub fn contains(self, other: &Self) -> bool {
        self.x.overlaps(&other.x) && self.y.overlaps(&other.y) && self.z.overlaps(&other.z)
    }

    pub fn contains_point(self, point: Vec3) -> bool {
        self.x.contains(point.e[0]) && self.y.contains(point.e[1]) && self.z.contains(point.e[2])
    }

    pub fn split(&self, axis: SplitAxis) -> (Aabb, Aabb) {
        let mut left = *self;
        let mut right = *self;

        match axis {
            SplitAxis::X => {
                let split = self.x.mid();

                left.x.max = split;
                right.x.min = split;
            }
            SplitAxis::Y => {
                let split = self.y.mid();

                left.y.max = split;
                right.y.min = split;
            }
            SplitAxis::Z => {
                let split = self.z.mid();

                left.z.max = split;
                right.z.min = split;
            }
        }

        (left, right)
    }

    pub fn ray_hit(&self, ray: &Ray) -> Option<f64> {
        if self.contains_point(ray.origin) {
            return Some(0.);
        }

        fn plane_intersect(ray: &Ray, norm: &Vec3, offset: f64) -> Option<f64> {
            let n_d = dot(norm, &ray.direction);
            if n_d != 0. {
                let n_p = dot(norm, &ray.origin);
                Some((offset - n_p) / n_d)
            } else {
                None
            }
        }

        // 6 planes: (normal, offset, axis indices to check, axes)
        let planes = [
            (vec3![1., 0., 0.], self.x.min, (1, 2), &self.y, &self.z), // x-min: check y,z
            (vec3![1., 0., 0.], self.x.max, (1, 2), &self.y, &self.z), // x-max: check y,z
            (vec3![0., 1., 0.], self.y.min, (0, 2), &self.x, &self.z), // y-min: check x,z
            (vec3![0., 1., 0.], self.y.max, (0, 2), &self.x, &self.z), // y-max: check x,z
            (vec3![0., 0., 1.], self.z.min, (0, 1), &self.x, &self.y), // z-min: check x,y
            (vec3![0., 0., 1.], self.z.max, (0, 1), &self.x, &self.y), // z-max: check x,y
        ];

        let mut intersection_t = None;

        for (norm, offset, (idx1, idx2), range1, range2) in planes.iter() {
            if let Some(t) = plane_intersect(ray, norm, *offset) {
                let intersect_point = ray.at(t);

                // Check that intersection is within the face of the AABB
                if range1.contains(intersect_point[*idx1])
                    && range2.contains(intersect_point[*idx2])
                {
                    // Update the intersection_t if current intersection is closer
                    intersection_t = match intersection_t {
                        Some(existing_t) if t < existing_t => Some(t),
                        None => Some(t),
                        _ => intersection_t,
                    };
                }
            }
        }

        // Reject negative t
        intersection_t.filter(|&t| t > 0.)
    }
}
