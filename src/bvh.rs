use crate::{
    hit::{HitRecord, Hittable},
    hit_list::HittableList,
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
    fn next(self) -> Self {
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

    pub fn surrounds(self, other: &Self) -> bool {
        self.x.contains_interval(&other.x)
            && self.y.contains_interval(&other.y)
            && self.z.contains_interval(&other.z)
    }

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
                let intersect_point = (ray.direction * t) + ray.origin;

                if range1.contains(intersect_point[*idx1])
                    && range2.contains(intersect_point[*idx2])
                {
                    intersection_t = match intersection_t {
                        Some(existing_t) if t < existing_t => Some(t),
                        None => Some(t),
                        _ => intersection_t,
                    };
                }
            }
        }

        intersection_t.filter(|&t| t > 0.)
    }
}

#[derive(Debug)]
pub struct BVHTree {
    pub left: Option<Box<BVHTree>>,
    pub right: Option<Box<BVHTree>>,
    pub aabb: Aabb,
    pub objects: HittableList,
}

impl BVHTree {
    pub fn from_hit_list(hit_list: HittableList, split_axis: SplitAxis) -> Self {
        let aabb = hit_list.bound();

        let mut left = HittableList::new();
        let mut right = HittableList::new();
        let mut both = HittableList::new();

        let (left_aabb, right_aabb) = aabb.split(split_axis);

        for object in hit_list.objects {
            let object_aabb = object.bound();
            match (
                left_aabb.contains(&object_aabb),
                right_aabb.contains(&object_aabb),
            ) {
                (true, true) => both.add(object),
                (true, false) => left.add(object),
                (false, true) => right.add(object),
                (false, false) => panic!("Object not contained in any bounding box."),
            }
        }

        let split_axis = split_axis.next();

        match (left.objects.is_empty(), right.objects.is_empty()) {
            (true, true) => Self {
                aabb,
                objects: both,
                left: None,
                right: None,
            },
            (true, false) => Self {
                aabb,
                objects: both,
                left: None,
                right: Some(Box::new(Self::from_hit_list(right, split_axis))),
            },
            (false, true) => Self {
                aabb,
                objects: both,
                left: Some(Box::new(Self::from_hit_list(left, split_axis))),
                right: None,
            },
            (false, false) => Self {
                aabb,
                objects: both,
                left: Some(Box::new(Self::from_hit_list(left, split_axis))),
                right: Some(Box::new(Self::from_hit_list(right, split_axis))),
            },
        }
    }

    pub fn verify(&self) -> bool {
        for object in &self.objects.objects {
            if !self.aabb.surrounds(&object.bound()) {
                return false;
            }
        }

        match &self.left {
            Some(left) => {
                if !Self::verify(left) {
                    return false;
                }
            }
            None => {}
        }

        match &self.right {
            Some(right) => {
                if !Self::verify(right) {
                    return false;
                }
            }
            None => {}
        }
        true
    }
}

impl Hittable for BVHTree {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let left = self.left.as_ref();
        let left_t = if let Some(node) = left {
            node.aabb.ray_hit(r)
        } else {
            None
        };

        let right = self.right.as_ref();
        let right_t = if let Some(node) = right {
            node.aabb.ray_hit(r)
        } else {
            None
        };

        let self_hit = self.objects.hit(r, ray_t);

        fn compare_hits<'a>(
            left_hit: Option<HitRecord<'a>>,
            right_hit: Option<HitRecord<'a>>,
        ) -> Option<HitRecord<'a>> {
            match (left_hit, right_hit) {
                (None, None) => None,
                (None, Some(r)) => Some(r),
                (Some(l), None) => Some(l),
                (Some(l), Some(r)) => {
                    if l.t < r.t {
                        Some(l)
                    } else {
                        Some(r)
                    }
                }
            }
        }

        let subtree_hit = match (left_t, right_t) {
            (None, None) => None,
            (None, Some(_)) => {
                if let Some(node) = right {
                    node.hit(r, ray_t)
                } else {
                    None
                }
            }
            (Some(_), None) => {
                if let Some(node) = left {
                    node.hit(r, ray_t)
                } else {
                    None
                }
            }
            (Some(l_t), Some(r_t)) => {
                if l_t < r_t {
                    if let Some(node) = left {
                        match node.hit(r, ray_t) {
                            Some(hit) => Some(hit),
                            None => {
                                if let Some(node) = right {
                                    node.hit(r, ray_t)
                                } else {
                                    None
                                }
                            }
                        }
                    } else if let Some(node) = right {
                        node.hit(r, ray_t)
                    } else {
                        None
                    }
                } else {
                    if let Some(node) = right {
                        match node.hit(r, ray_t) {
                            Some(hit) => Some(hit),
                            None => {
                                if let Some(node) = left {
                                    node.hit(r, ray_t)
                                } else {
                                    None
                                }
                            }
                        }
                    } else if let Some(node) = left {
                        node.hit(r, ray_t)
                    } else {
                        None
                    }
                }
            }
        };

        compare_hits(self_hit, subtree_hit)
    }

    fn bound(&self) -> Aabb {
        self.aabb
    }
}
