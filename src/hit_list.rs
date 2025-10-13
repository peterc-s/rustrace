use crate::bvh::Aabb;
use crate::interval;
use crate::{
    hit::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Debug, Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn clear(self) {
        drop(self);
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn combine(&mut self, mut other: Self) {
        self.objects.append(&mut other.objects);
    }
}

impl Hittable for HittableList {
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

    fn bound(&self) -> Aabb {
        let mut aabb = Aabb::new();
        for object in &self.objects {
            aabb.union(&object.bound());
        }
        aabb
    }
}
