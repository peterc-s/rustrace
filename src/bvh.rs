use crate::{
    aabb::{Aabb, SplitAxis},
    hit::{HitRecord, Hittable},
    hit_list::HittableList,
    interval::Interval,
    ray::Ray,
};

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

    // DEBUG: verify that AABBs surround their object's AABBs
    // pub fn verify(&self) -> bool {
    //     for object in &self.objects.objects {
    //         if !self.aabb.surrounds(&object.bound()) {
    //             return false;
    //         }
    //     }
    //
    //     if let Some(left) = &self.left {
    //         if !Self::verify(left) {
    //             return false;
    //         }
    //     }
    //
    //     if let Some(right) = &self.right {
    //         if !Self::verify(right) {
    //             return false;
    //         }
    //     }
    //     true
    // }
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
        let subtree_hit = match (left_t, right_t) {
            (None, None) => None,
            (Some(_), None) => left?.hit(r, ray_t),
            (None, Some(_)) => right?.hit(r, ray_t),
            (Some(l_t), Some(r_t)) => {
                let (first, second) = if l_t < r_t {
                    (left, right)
                } else {
                    (right, left)
                };
                first?.hit(r, ray_t).or_else(|| second?.hit(r, ray_t))
            }
        };

        match (self_hit, subtree_hit) {
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

    fn bound(&self) -> Aabb {
        self.aabb
    }
}
