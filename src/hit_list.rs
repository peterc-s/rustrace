use crate::{hit::{HitRecord, Hittable}, interval::Interval, interval};
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn clear(mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
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
