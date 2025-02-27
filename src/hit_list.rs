use crate::interval;
use crate::{hit::{HitRecord, Hittable}, interval::Interval, ray::Ray};
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn clear(mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
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
}
