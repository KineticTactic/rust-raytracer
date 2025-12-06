use std::fs;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let file = fs::File::create("image.ppm")?;
    let mut out = BufWriter::new(file);

    write!(out, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n")?;

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let g = j as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            writeln!(out, "{ir} {ig} {ib}")?;
        }
    }

    Ok(())
}
