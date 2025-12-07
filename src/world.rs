use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: Ray, t_range: Interval) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|obj| obj.hit(ray, t_range.clone()))
            .min_by(|a, b| a.t.partial_cmp(&b.t).expect("Wtf?"))
    }
}
