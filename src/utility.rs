use crate::vec3::Vec3;
use rand::Rng;

pub fn rand_sample_2d() -> Vec3 {
    let mut rng = rand::rng();
    Vec3::new(
        rng.random_range(-0.5..0.5),
        rng.random_range(-0.5..0.5),
        0.0,
    )
}

// TODO: PROFILE THIS RANDOM RNG
pub fn rand_sample_3d() -> Vec3 {
    let mut rng = rand::rng();
    Vec3::new(
        rng.random_range(-0.5..0.5),
        rng.random_range(-0.5..0.5),
        rng.random_range(-0.5..0.5),
    )
}

pub fn rand_unit_vector() -> Vec3 {
    loop {
        let p = rand_sample_3d();
        let mag_sq = p.mag_sq();
        if 1e-160 < mag_sq && mag_sq <= 1.0 {
            return p / mag_sq.sqrt();
        }
    }
}

pub fn rand_unit_vector_on_hemisphere(normal: Vec3) -> Vec3 {
    let rand_vec = rand_unit_vector();
    if Vec3::dot(rand_vec, normal) > 0.0 {
        rand_vec
    } else {
        -rand_vec
    }
}

pub fn rand_unit_circle() -> Vec3 {
    loop {
        let rand_vec = rand_sample_2d() * 2.0;
        if rand_vec.mag_sq() < 1.0 {
            return rand_vec;
        }
    }
}
