use crate::hittable::*;
use crate::color::*;
use crate::ray::*;
use crate::point3d::*;
use std::fs::File;
use std::io::{self, Write};
use crate::random::*;
use crate::material::*;
use rayon::prelude::*;
use rand::prelude::*;

pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

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
    vfov: f64, // Vertical view angle (field of view)
    lookfrom: Point3D, // Point camera is looking from
    lookat: Point3D, // Point camera is looking at
    vup: Point3D, // Camera-relative "up" direction
    defocus_angle: f64, // Variation angle of rays through each pixel
    focus_dist: f64, // Distance from camera lookfrom point to plane of perfect focus
    defocus_disk_u: Point3D, // Defocus disk horizontal radius 
    defocus_disk_v: Point3D, // Defocus disk vertical radius
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: f64, samples_per_pixel: f64, max_depth: usize, 
        vfov: f64, lookfrom: Point3D, lookat: Point3D, vup: Point3D, defocus_angle: f64, focus_dist: f64) -> Camera {
        let image_height = if image_width / aspect_ratio < 1.0 {
            1.0 
        } else {
            image_width / aspect_ratio
        };

        let center = lookfrom;

        // Determine viewport dimensions.
        let theta = Camera::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h * focus_dist;
        let viewport_width: f64 = viewport_height * (image_width / image_height);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u * viewport_width;     // Vector across viewport horizontal edge
        let viewport_v = -v * viewport_height;   // Vector down viewport vertical edge 

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (w * focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * Camera::degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        // Determine how many rays hit each point.
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
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle, 
            focus_dist,
            defocus_disk_u, 
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        let output_dir: &String = &"output".to_string();
        let file_name: &String = &"final_render".to_string();
        let path: String = format!("{}/{}.ppm", output_dir, file_name);
        let mut file = File::create(path)?;

        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        let samples_per_pixel = (1.0 / self.pixel_samples_scale) as usize;
        for j in 0..self.image_height as usize {
            for i in 0..self.image_width as usize {
                let mut pixel_color = Point3D::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let r = self.get_ray(i as f64, j as f64);
                    pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world);
                }

                write_color(&file, pixel_color * self.pixel_samples_scale)?;
            }
        }
        Ok(())
    }

    fn ray_color(&self, r: &Ray, max_depth: usize, world: &dyn Hittable) -> Point3D { 
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if max_depth == 0 {
            Point3D::new(0.0, 0.0, 0.0)
        } else if let Some(rec) = world.hit(r, 0.001, f64::MAX) { 
            if let Some((scattered, accentuation)) = Scatterable::scatter(&rec.material, r, &rec) {
                return accentuation * self.ray_color(&scattered, max_depth - 1, world);
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
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc  
            + (self.pixel_delta_u * (i + offset.x()))
            + (self.pixel_delta_v * (j + offset.y()));
        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() }; 
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3D {
        let p = Point3D::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y())
    }


    fn sample_square(&self) -> Point3D {
        Point3D::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    pub fn degrees_to_radians(deg: f64) -> f64 {
        deg * PI / 180.0
    }
}
