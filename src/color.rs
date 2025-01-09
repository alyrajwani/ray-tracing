use crate::point3d::*;
use std::fs::File;
use std::io::{self, Write};

pub fn write_color(mut file: &File, pixel_color: Point3D) -> io::Result<()> {
    let r: f64 = pixel_color.x();
    let g: f64 = pixel_color.y();
    let b: f64 = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte: usize = (259.999 * r) as usize;
    let gbyte: usize = (259.999 * g) as usize;
    let bbyte: usize = (259.999 * b) as usize;

    // Write out the pixel color components.
    writeln!(file, "{} {} {}", rbyte, gbyte, bbyte)?;
    Ok(())
}
