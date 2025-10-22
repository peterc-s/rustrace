//! This module contains [`HittableList`] which is basically an abstraction for a
//! [`Vec<Box<dyn Hittable>>`], which in itself is [hittable](Hittable).

use crate::{
    aabb::Aabb,
    hit::{HitRecord, Hittable},
    interval,
    interval::Interval,
    ray::Ray,
};

/// The [`HittableList`] struct itself. A [`Hittable`] abstraction over a [`Vec<Box<dyn Hittable>>`].
#[derive(Debug, Default)]
pub struct HittableList {
    /// The vector of [`Hittable`] objects in the list.
    pub objects: Vec<Box<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    /// Clear out the [`HittableList`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{hit_list::HittableList};
    ///
    /// let mut hit_list = HittableList::new();
    /// let inner = Box::new(HittableList::new());
    ///
    /// hit_list.add(inner);
    ///
    /// assert_eq!(hit_list.objects.len(), 1);
    ///
    /// hit_list.clear();
    ///
    /// assert_eq!(hit_list.objects.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// Push a [`Box<dyn Hittable>`] into [`self.objects`](field@HittableList::objects).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::hit_list::HittableList;
    /// let mut hit_list = HittableList::new();
    /// let inner = Box::new(HittableList::new());
    ///
    /// hit_list.add(inner);
    ///
    /// assert_eq!(hit_list.objects.len(), 1);
    /// ```
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    /// Create a new, empty [`HittableList`].
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    /// Basically [append](fn@Vec::append) for [`HittableList`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::hit_list::HittableList;
    /// let mut hit_list_0 = HittableList::new();
    /// let inner_0 = Box::new(HittableList::new());
    /// hit_list_0.add(inner_0);
    ///
    /// let mut hit_list_1 = HittableList::new();
    /// let inner_1 = Box::new(HittableList::new());
    /// hit_list_0.add(inner_1);
    ///
    /// hit_list_0.combine(hit_list_1);
    ///
    /// // Combined list should now contain both inner elements.
    /// assert_eq!(hit_list_0.objects.len(), 2);
    /// ```
    pub fn combine(&mut self, mut other: Self) {
        self.objects.append(&mut other.objects);
    }
}

impl Hittable for HittableList {
    /// Check if anything in the [`HittableList`] has been hit. If there is an intersection
    /// a [`Ok(HitRecord)`](Option<HitRecord>) will be returned, otherwise [`None`].
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let mut out_rec: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, interval![ray_t.min, closest_so_far]) {
                closest_so_far = rec.t;
                out_rec = Some(rec);
            }
        }

        out_rec
    }

    /// Returns the minimal [`Aabb`] that contains all of the objects in the [`HittableList`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{
    ///     hit::Hittable,
    ///     hit_list::HittableList,
    ///     sphere::Sphere,
    ///     material::Dielectric,
    ///     aabb::Aabb,
    ///     vec3::Vec3,
    ///     vec3,
    /// };
    ///
    /// let mut hit_list = HittableList::default();
    /// let mut mat = Box::new(Dielectric::new(1.5));
    ///
    /// hit_list.add(Box::new(Sphere {
    ///     centre: vec3![0.0, 0.0, 0.0],
    ///     radius: 1.0,
    ///     mat: mat,
    /// }));
    ///
    /// let bounds = hit_list.bound();
    /// assert_eq!(bounds, hit_list.objects[0].bound());
    /// ```
    fn bound(&self) -> Aabb {
        let mut aabb = Aabb::new();
        for object in &self.objects {
            aabb.union(&object.bound());
        }
        aabb
    }
}
