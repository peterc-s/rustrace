//! This crate contains the implementation of a [`BVHTree`] struct which supports being
//! created from a [`HittableList`]. Used to optimise intersection tests, see
//! [wikipedia](https://en.wikipedia.org/wiki/Bounding_volume_hierarchy) for more information.
//! This implementation also uses surface area heuristic splitting, see
//! [here](https://www.pbr-book.org/3ed-2018/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies)
//! for more information.

use crate::{
    aabb::{Aabb, SplitAxis},
    hit::{HitRecord, Hittable},
    hit_list::HittableList,
    interval::Interval,
    ray::Ray,
};

/// Used in [BVHTree::sah_split] to bucket objects.
#[derive(Debug, Clone, Copy)]
struct Bucket {
    count: usize,
    bounds: Aabb,
}

impl Bucket {
    /// Create a new bucket with a `count` of `0` and a `bounds` of [`Aabb::empty()`].
    fn new() -> Self {
        Self {
            count: 0,
            bounds: Aabb::new(),
        }
    }
}

/// The [`BVHTree`] struct itself. Has two possible child nodes `left` and `right`
/// which must be [boxed](Box). Has an [`aabb`](Aabb) which bounds the [`objects`][HittableList]
/// within the current node and its children.
#[derive(Debug)]
pub struct BVHTree {
    /// The left subtree.
    pub left: Option<Box<BVHTree>>,
    /// The right subtree.
    pub right: Option<Box<BVHTree>>,
    /// This [`Aabb`] bounds all the [`objects`](HittableList) in this node and its children.
    pub aabb: Aabb,
    /// The [objects](HittableList) contained within the current node.
    pub objects: HittableList,
}

impl BVHTree {
    /// Create a [`BVHTree`] from a [`HittableList`] using surface area heuristics to
    /// split effectively. [Read more](https://www.pbr-book.org/3ed-2018/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies).
    pub fn from_hit_list(hit_list: HittableList) -> Self {
        let aabb = hit_list.bound();

        let split_axis = SplitAxis::choose_from_aabb(aabb);
        let (left, right, both) = Self::sah_split(hit_list, &aabb, split_axis);

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
                right: Some(Box::new(Self::from_hit_list(right))),
            },
            (false, true) => Self {
                aabb,
                objects: both,
                left: Some(Box::new(Self::from_hit_list(left))),
                right: None,
            },
            (false, false) => Self {
                aabb,
                objects: both,
                left: Some(Box::new(Self::from_hit_list(left))),
                right: Some(Box::new(Self::from_hit_list(right))),
            },
        }
    }

    /// Splits a [`HittableList`] into three parts `(left, right, both)`
    /// according to a surface area heuristic cost. Uses [`BVHTree::partition_objects()`] to
    /// partition. [Read more](https://www.pbr-book.org/3ed-2018/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies).
    fn sah_split(
        hit_list: HittableList,
        aabb: &Aabb,
        split_axis: SplitAxis,
    ) -> (HittableList, HittableList, HittableList) {
        const NUM_BUCKETS: usize = 12;

        let axis_interval = match split_axis {
            SplitAxis::X => aabb.x,
            SplitAxis::Y => aabb.y,
            SplitAxis::Z => aabb.z,
        };

        let mut buckets = vec![Bucket::new(); NUM_BUCKETS];

        for object in &hit_list.objects {
            let object_aabb = object.bound();
            let centroid = object_aabb.centroid();
            let centroid_value = match split_axis {
                SplitAxis::X => centroid.e[0],
                SplitAxis::Y => centroid.e[1],
                SplitAxis::Z => centroid.e[2],
            };

            let bucket_idx = ((centroid_value - axis_interval.min) / axis_interval.size()
                * NUM_BUCKETS as f64)
                .floor()
                .min((NUM_BUCKETS - 1) as f64) as usize;

            buckets[bucket_idx].count += 1;
            buckets[bucket_idx].bounds.union(&object_aabb);
        }

        let mut costs = [0.; NUM_BUCKETS - 1];

        for (i, cost) in costs.iter_mut().enumerate().take(NUM_BUCKETS - 1) {
            let mut left_box = Aabb::new();
            let mut right_box = Aabb::new();
            let mut left_count = 0;
            let mut right_count = 0;

            for left_bucket in buckets.iter().take(i + 1) {
                left_box.union(&left_bucket.bounds);
                left_count += left_bucket.count;
            }

            for right_bucket in buckets.iter().take(NUM_BUCKETS).skip(i + 1) {
                right_box.union(&right_bucket.bounds);
                right_count += right_bucket.count;
            }

            *cost = left_box.surface_area() * left_count as f64
                + right_box.surface_area() * right_count as f64;
        }

        let min_cost_idx = costs
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(idx, _)| idx)
            .unwrap();

        let split_pos = axis_interval.min
            + (axis_interval.size() * (min_cost_idx + 1) as f64 / NUM_BUCKETS as f64);

        Self::partition_objects(hit_list, split_axis, split_pos, aabb)
    }

    /// Partitions [`hit_list`](HittableList) into three new [`HittableList`]s along the
    /// given [`split_axis`](SplitAxis) and `split_pos`.
    fn partition_objects(
        hit_list: HittableList,
        split_axis: SplitAxis,
        split_pos: f64,
        parent_aabb: &Aabb,
    ) -> (HittableList, HittableList, HittableList) {
        let mut left = HittableList::new();
        let mut right = HittableList::new();
        let mut both = HittableList::new();

        let (left_aabb, right_aabb) = parent_aabb.split_at(split_axis, split_pos);

        for object in hit_list.objects {
            let object_aabb = object.bound();
            let centroid = object_aabb.centroid();
            let centroid_value = match split_axis {
                SplitAxis::X => centroid.e[0],
                SplitAxis::Y => centroid.e[1],
                SplitAxis::Z => centroid.e[2],
            };

            match (
                left_aabb.overlaps(&object_aabb),
                right_aabb.overlaps(&object_aabb),
            ) {
                (true, true) => both.add(object),
                (true, false) if centroid_value < split_pos => left.add(object),
                (false, true) if centroid_value >= split_pos => right.add(object),
                _ => both.add(object), // Fallback for edge cases
            }
        }

        (left, right, both)
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
    /// Recursively check if the ray [`r`](Ray) hits any object in the current
    /// [`BVHTree`] node or any sub-trees and returns a [`Some(HitRecord)`](Option<HitRecord>)
    /// of the closest intersection if found. Otherwise, returns [`None`].
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

    /// Returns [`self.aabb`](field@BVHTree::aabb).
    fn bound(&self) -> Aabb {
        self.aabb
    }
}
