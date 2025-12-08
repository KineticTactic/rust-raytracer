use std::sync::Arc;

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub pos: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    // TODO: I DONT LIKE THE MESS OF PARAMETERS HERE
    pub fn new(
        pos: Vec3,
        outward_normal: Vec3,
        t: f64,
        ray: Ray,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = Vec3::dot(ray.dir, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            pos,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_range: Interval) -> Option<HitRecord>;
}
