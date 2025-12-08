use crate::{interval::Interval, vec3::Vec3};
use std::io::{Result, Write};

pub type Color = Vec3;

impl Color {
    pub fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }

    pub fn write_color<W: Write>(out: &mut W, c: Color) -> Result<()> {
        let r = Self::linear_to_gamma(c.x);
        let g = Self::linear_to_gamma(c.y);
        let b = Self::linear_to_gamma(c.z);

        let ir = (256.0 * Interval::INTENSITY.clamp(r)) as u8;
        let ig = (256.0 * Interval::INTENSITY.clamp(g)) as u8;
        let ib = (256.0 * Interval::INTENSITY.clamp(b)) as u8;

        writeln!(out, "{ir} {ig} {ib}")?;
        Ok(())
    }
}
