mod color;
mod hittable;
mod point3d;
mod ray;
mod sphere;

use std::io::{self, Write};
use std::fs::File;
use std::rc::Rc;
use crate::point3d::*;
use crate::ray::*;
use crate::hittable::*;
use crate::sphere::*;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Point3D {
    if let Some(rec) = world.hit(r, 0.0, f64::MAX) {
        return (rec.normal + Point3D::new(0.0, 0.0, 0.0)) * 0.5;
    } else {
        let unit_direction = r.direction().unit_vector();
        let a: f64 = 0.5 * (unit_direction.y() + 1.0);
        Point3D::new(1.0, 1.0, 1.0) * (1.0 - a) + Point3D::new(0.5, 0.7, 1.0) * a
    }
}

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 400.0;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height: f64 = if image_width / aspect_ratio < 1.0 {
        1.0
    } else {
        image_width / aspect_ratio
    };

    // World
    let mut world = HittableList::new(Vec::new());
    world.list.push(Rc::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5)));
    world.list.push(Rc::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width / image_height);
    let camera_center = Point3D::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Point3D::new(viewport_width, 0.0, 0.0);
    let viewport_v = Point3D::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Point3D::new(0.0, 0.0, focal_length)
        - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    let output_dir: &String = &"output".to_string();
    let file_name: &String = &"chapter1".to_string();
    let path: String = format!("{}/{}.ppm", output_dir, file_name);
    let mut file = File::create(path)?;

    writeln!(file, "P3\n{image_width} {image_height}\n255")?;

    for j in 0..image_height as usize {
        for i in 0..image_width as usize {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r, &world);
            color::write_color(&file, pixel_color)?; 
        }
    }

    Ok(())
}
