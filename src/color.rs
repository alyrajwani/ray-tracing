use crate::point3d::*;
use std::fs::File;
use std::io::{self, Write};

pub fn write_color(mut file: &File, pixel_color: Point3D) -> io::Result<()> {
    let r: f64 = pixel_color.x();
    let g: f64 = pixel_color.y();
    let b: f64 = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    let min: f64 = 0.000;
    let max: f64 = 0.999;
    let rbyte: usize = (256.0 * clamp(r, min, max)) as usize;
    let gbyte: usize = (256.0 * clamp(g, min, max)) as usize;
    let bbyte: usize = (256.0 * clamp(b, min, max)) as usize;

    // Write out the pixel color components.
    writeln!(file, "{} {} {}", rbyte, gbyte, bbyte)?;
    Ok(())
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
