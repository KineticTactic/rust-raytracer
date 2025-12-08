use rand::Rng;

use crate::{color::Color, hittable::HitRecord, ray::Ray, utility::rand_unit_vector, vec3::Vec3};

pub struct ScatterInfo {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterInfo>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit_record: &HitRecord) -> Option<ScatterInfo> {
        let mut scatter_direction = hit_record.normal + rand_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        Some(ScatterInfo {
            ray: Ray::new(hit_record.pos, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterInfo> {
        let mut reflected_direction = Vec3::reflect(ray.dir, hit_record.normal);
        reflected_direction = reflected_direction.normalize() + (self.fuzz * rand_unit_vector());
        if Vec3::dot(reflected_direction, hit_record.normal) > 0.0 {
            Some(ScatterInfo {
                ray: Ray::new(hit_record.pos, reflected_direction),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }

    fn reflectance(cosine: f64, ri: f64) -> f64 {
        let mut r0 = (1.0 - ri) / (1.0 + ri);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterInfo> {
        let ri = if hit_record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = ray.dir.normalize();
        let cos_theta = f64::min(Vec3::dot(-unit_direction, hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let rand_prob = rand::rng().random_range(0.0..1.0);
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, ri) > rand_prob {
            Vec3::reflect(unit_direction, hit_record.normal)
        } else {
            Vec3::refract(unit_direction, hit_record.normal, ri)
        };
        Some(ScatterInfo {
            ray: Ray::new(hit_record.pos, direction),
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}
