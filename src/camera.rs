use crate::color::Color;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility::{self, rand_sample_2d};
use crate::{vec3::Vec3, world::World};
use std::f64::INFINITY;
use std::io::{BufWriter, Write};
use std::{fs, io};

pub struct Camera {
    pos: Vec3,
    focal_length: f64,
    image_width: u32,
    image_height: u32,
    pixel00_pos: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    viewport_height: f64,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,
}

impl Camera {
    pub fn new(
        pos: Vec3,
        image_width: u32,
        image_height: u32,
        focal_length: f64,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height / (image_height as f64) * (image_width as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let viewport_upper_left =
            pos - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);
        let pixel00_pos = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        //let mut rng = rand::rng();

        Camera {
            pos,
            focal_length,
            image_width,
            image_height,
            viewport_height,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_pos,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }

    fn ray_color(ray: Ray, depth: u32, world: &World) -> Color {
        if depth == 0 {
            return Color::zero();
        }

        if let Some(hit_record) = world.hit(ray, Interval::new(0.001, INFINITY)) {
            let new_dir = hit_record.normal + utility::rand_unit_vector();

            return 0.5 * Camera::ray_color(Ray::new(hit_record.pos, new_dir), depth - 1, world);
        }

        let unit_dir = ray.dir.normalize();
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = rand_sample_2d();

        let pixel_sample = self.pixel00_pos
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        Ray::new(self.pos, pixel_sample - self.pos)
    }

    pub fn render(&self, world: &World) {
        let file = fs::File::create("image.ppm").expect("Failed to create image file!");
        let mut out = BufWriter::new(file);
        write!(out, "P3\n{} {}\n255\n", self.image_width, self.image_height)
            .expect("Failed to write to file!");

        for j in 0..self.image_height {
            println!("\rScanlines remaining: {} ", self.image_height - j);
            io::stdout().flush().unwrap();

            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _sample in 0..self.samples_per_pixel {
                    pixel_color += Camera::ray_color(self.get_ray(i, j), self.max_depth, world);
                }
                Color::write_color(&mut out, self.pixel_samples_scale * pixel_color)
                    .expect("Failed to write to file!");
            }
        }
        println!("\rDone!");
    }
}
