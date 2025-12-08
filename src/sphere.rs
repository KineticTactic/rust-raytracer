use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            pos,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_range: Interval) -> Option<HitRecord> {
        let oc = self.pos - ray.origin;
        let a = ray.dir.mag_sq();
        let h = Vec3::dot(ray.dir, oc);
        let c = oc.mag_sq() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !t_range.surrounds(root) {
            root = (h + sqrtd) / a;
            if !t_range.surrounds(root) {
                return None;
            }
        }

        Some(HitRecord::new(
            ray.at(root),
            (ray.at(root) - self.pos) / self.radius,
            root,
            ray,
            self.material.clone(),
        ))
    }
}
