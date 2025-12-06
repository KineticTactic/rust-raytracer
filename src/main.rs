use raytracer::color::Color;
use raytracer::vec3::Vec3;
use std::io::{BufWriter, Write};
use std::{fs, io};

fn main() -> std::io::Result<()> {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let file = fs::File::create("image.ppm")?;
    let mut out = BufWriter::new(file);

    write!(out, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n")?;

    for j in 0..IMAGE_HEIGHT {
        println!("\rScanlines remaining: {} ", IMAGE_HEIGHT - j);
        io::stdout().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH as f64 - 1.0),
                j as f64 / (IMAGE_HEIGHT as f64 - 1.0),
                0.0,
            );

            Color::write_color(&mut out, pixel_color)?;
        }
    }
    println!("\rDone!                          ");

    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(3.0, 2.0, 3.0);
    println!("{:?}", a + b);

    Ok(())
}
