use crate::{interval::Interval, vec3::Vec3};
use std::io::{Result, Write};

pub type Color = Vec3;

impl Color {
    pub fn write_color<W: Write>(out: &mut W, c: Color) -> Result<()> {
        let ir = (256.0 * Interval::INTENSITY.clamp(c.x)) as u8;
        let ig = (256.0 * Interval::INTENSITY.clamp(c.y)) as u8;
        let ib = (256.0 * Interval::INTENSITY.clamp(c.z)) as u8;

        writeln!(out, "{ir} {ig} {ib}")?;
        Ok(())
    }
}
