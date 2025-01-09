use crate::hittable::*;
use crate::color::*;
use crate::ray::*;
use crate::point3d::*;
use std::fs::File;
use std::io::{self, Write};

pub struct Camera {
    center: Point3D,
    pixel00_loc: Point3D,
    pixel_delta_u: Point3D,
    pixel_delta_v: Point3D,
    image_height: f64,
    image_width: f64,
    aspect_ratio: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: f64) -> Camera {
        let image_height = if image_width / aspect_ratio < 1.0 {
            1.0 
        } else {
            image_width / aspect_ratio
        };
        let center = Point3D::new(0.0, 0.0, 0.0);
        // Determine viewport dimensions.
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width / image_height);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Point3D::new(viewport_width, 0.0, 0.0);
        let viewport_v = Point3D::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - Point3D::new(0.0, 0.0, focal_length) 
            - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            image_height,
            image_width,
            aspect_ratio
        }
    }

    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        let output_dir: &String = &"output".to_string();
        let file_name: &String = &"chapter1".to_string();
        let path: String = format!("{}/{}.ppm", output_dir, file_name);
        let mut file = File::create(path)?;


        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        for j in 0..self.image_height as usize {
            for i in 0..self.image_width as usize {
                let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(&r, world);
                write_color(&file, pixel_color)?;
            }
        }

        Ok(())
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Point3D { 
        if let Some(rec) = world.hit(r, 0.0, f64::MAX) { 
            return (rec.normal + Point3D::new(0.0, 0.0, 0.0)) * 0.5;
        } else { 
            let unit_direction = r.direction().unit_vector();
            let a: f64 = 0.5 * (unit_direction.y() + 1.0);
            Point3D::new(1.0, 1.0, 1.0) * (1.0 - a) + Point3D::new(0.5, 0.7, 1.0) * a 
        } 
    }
}
