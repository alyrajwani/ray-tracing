use crate::point3d::*;
use std::fs::File;
use std::io::{self, Write};

pub fn write_color(mut file: &File, pixel_color: Point3D) -> io::Result<()> {
    let r_lin: f64 = pixel_color.x();
    let g_lin: f64 = pixel_color.y();
    let b_lin: f64 = pixel_color.z();
    
    // Apply a linear to gamma transform for gamma 2
    let r_gam = linear_to_gamma(r_lin);
    let g_gam = linear_to_gamma(g_lin);
    let b_gam = linear_to_gamma(b_lin);

    // Translate the [0,1] component values to the byte range [0,255].
    let min: f64 = 0.000;
    let max: f64 = 0.999;
    let rbyte: usize = (256.0 * clamp(r_gam, min, max)) as usize;
    let gbyte: usize = (256.0 * clamp(g_gam, min, max)) as usize;
    let bbyte: usize = (256.0 * clamp(b_gam, min, max)) as usize;

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

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        f64::sqrt(linear_component)
    } else {
        0.0
    }
}
