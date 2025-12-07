use raytracer::camera::Camera;
use raytracer::sphere::Sphere;
use raytracer::vec3::Vec3;
use raytracer::world::World;

fn main() {
    let camera = Camera::new(Vec3::zero(), 800, 450, 1.0, 10);

    let mut world = World::default();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -12.0, -1.0), 10.0)));

    camera.render(&world);
}
