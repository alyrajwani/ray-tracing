use crate::hittable::*;
use crate::color::*;
use crate::ray::*;
use crate::point3d::*;
use std::fs::File;
use std::io::{self, Write};
use crate::random::*;
use crate::material::*;

pub struct Camera {
    center: Point3D,
    pixel00_loc: Point3D,
    pixel_delta_u: Point3D,
    pixel_delta_v: Point3D,
    image_height: f64,
    image_width: f64,
    aspect_ratio: f64,
    pixel_samples_scale: f64,
    max_depth: usize,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: f64, samples_per_pixel: f64, max_depth: usize) -> Camera {
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

        let pixel_samples_scale = 1.0 / samples_per_pixel;

        Camera {
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            image_height,
            image_width,
            aspect_ratio,
            pixel_samples_scale,
            max_depth,
        }
    }

    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        let output_dir: &String = &"output".to_string();
        let file_name: &String = &"output".to_string();
        let path: String = format!("{}/{}.ppm", output_dir, file_name);
        let mut file = File::create(path)?;

        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        let samples_per_pixel = (1.0 / self.pixel_samples_scale) as usize;
        for j in 0..self.image_height as usize {
            for i in 0..self.image_width as usize {
                let mut pixel_color = Point3D::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let r = self.get_ray(i as f64, j as f64);
                    pixel_color = pixel_color + Camera::ray_color(&r, self.max_depth, world);
                }

                write_color(&file, pixel_color * self.pixel_samples_scale)?;
            }
        }

        Ok(())
    }

    fn ray_color(r: &Ray, max_depth: usize, world: &dyn Hittable) -> Point3D { 
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if max_depth == 0 {
            Point3D::new(0.0, 0.0, 0.0)
        } else if let Some(rec) = world.hit(r, 0.001, f64::MAX) { 
            if let Some((scattered, accentuation)) = Scatterable::scatter(&rec.material, r, &rec) {
                return accentuation * Camera::ray_color(&scattered, max_depth - 1, world);
            } else {
                return Point3D::new(0.0, 0.0, 0.0)
            }
        } else { 
            let unit_direction = r.direction().unit_vector();
            let a: f64 = 0.5 * (unit_direction.y() + 1.0);
            Point3D::new(1.0, 1.0, 1.0) * (1.0 - a) + Point3D::new(0.5, 0.7, 1.0) * a 
        } 
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc  
            + (self.pixel_delta_u * (i + offset.x()))
            + (self.pixel_delta_v * (j + offset.y()));
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Point3D {
        Point3D::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }
}
