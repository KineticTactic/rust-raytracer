use crate::vec3::Vec3;
use std::io::{Result, Write};

pub type Color = Vec3;

impl Color {
    pub fn write_color<W: Write>(out: &mut W, c: Color) -> Result<()> {
        let ir = (255.999 * c.x) as u8;
        let ig = (255.999 * c.y) as u8;
        let ib = (255.999 * c.z) as u8;

        writeln!(out, "{ir} {ig} {ib}")?;
        Ok(())
    }
}
