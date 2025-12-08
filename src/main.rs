use std::sync::Arc;

use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::sphere::Sphere;
use raytracer::vec3::Vec3;
use raytracer::world::World;

fn main() {
    let camera = Camera::new(
        Vec3::new(-1.0, 2.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        2.0,
        10.0,
        800,
        450,
        90.0,
        2,
        10,
    );

    let material_ground = Arc::new(Lambertian::new(Color::new(0.3, 0.8, 0.1)));
    let material_red = Arc::new(Lambertian::new(Color::new(0.8, 0.1, 0.2)));
    let material_blue = Arc::new(Lambertian::new(Color::new(0.1, 0.3, 0.7)));
    let material_metal1 = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.8), 0.1));
    let material_metal2 = Arc::new(Metal::new(Color::new(0.8, 0.9, 0.58), 0.5));
    let material_dielectric = Arc::new(Dielectric::new(1.5));

    let mut world = World::default();
    world.add(Box::new(Sphere::new(
        Vec3::new(-0.9, 0.0, -1.0),
        0.3,
        material_dielectric,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-0.5, 0.8, -1.5),
        0.5,
        material_metal2,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.5, 0.2, -1.0),
        0.6,
        material_blue,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_metal1,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    camera.render(&world);
}
