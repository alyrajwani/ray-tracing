use crate::hittable::*;
use crate::color::*;
use crate::ray::*;
use crate::point3d::*;
use std::fs::{self, File};
use std::io::{self, Write};
use crate::random::*;
use crate::material::*;
use rayon::prelude::*;

pub struct Camera {
    stats: CameraStats,
    view: CameraView,
    focus: CameraFocus,
}

#[derive(Clone, Copy)]
pub struct CameraStats {
    aspect_ratio: f64, // Ratio of image width over height
    image_height: f64, // Rendered image height in pixel count
    image_width: f64, // Rendered image width in pixel count
    pixel_samples_scale: f64, // Count of random samples for each pixel
    max_depth: usize, // Maximum number of ray bounces into scene
}

#[derive(Clone, Copy)]
pub struct CameraView {
    center: Point3D, // Camera center
    pixel00_loc: Point3D, // Location of pixel 0, 0
    pixel_delta_u: Point3D, // Offset to pixel to the right
    pixel_delta_v: Point3D, // Offset to pixel below
    vfov: f64, // Vertical view angle (field of view)
    lookfrom: Point3D, // Point camera is looking from
    lookat: Point3D, // Point camera is looking at
    vup: Point3D, // Camera-relative "up" direction
    focus_dist: f64, // Distance from camera lookfrom point to plane of perfect focus
}

#[derive(Clone, Copy)]
pub struct CameraFocus {
    defocus_angle: f64, // Variation angle of rays through each pixel
    defocus_disk_u: Point3D, // Defocus disk horizontal radius 
    defocus_disk_v: Point3D, // Defocus disk vertical radius
}

impl CameraStats {
    pub fn new(aspect_ratio: f64, image_width: f64, samples_per_pixel: f64, max_depth: usize) -> CameraStats {
        let image_height = if image_width / aspect_ratio < 1.0 {
            1.0 
        } else {
            image_width / aspect_ratio
        };
        let pixel_samples_scale = 1.0 / samples_per_pixel;

        CameraStats { aspect_ratio, image_height, image_width, pixel_samples_scale, max_depth } 
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    pub fn height(&self) -> f64 {
        self.image_height
    }

    pub fn width(&self) -> f64 {
        self.image_width
    }

    pub fn samples_per_pixel(&self) -> usize {
        (1.0 / self.pixel_samples_scale) as usize
    }
}

impl CameraView {
    pub fn new(stats: CameraStats, vfov: f64, lookfrom: Point3D, lookat: Point3D, vup: Point3D, focus_dist: f64) -> CameraView {
        let center = lookfrom;

        // Determine viewport dimensions.
        let theta = Camera::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h * focus_dist;
        let viewport_width: f64 = viewport_height * (stats.image_width / stats.image_height);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u * viewport_width;     // Vector across viewport horizontal edge
        let viewport_v = -v * viewport_height;   // Vector down viewport vertical edge 

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / stats.image_width;
        let pixel_delta_v = viewport_v / stats.image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (w * focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Calculate the location of the upper left pixel.
        CameraView { center, pixel00_loc, pixel_delta_u, pixel_delta_v, vfov, lookfrom, lookat, vup, focus_dist }
    }
}

impl CameraFocus {
    pub fn new(view: CameraView, defocus_angle: f64) -> CameraFocus {
        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (view.lookfrom - view.lookat).unit_vector();
        let u = view.vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = view.focus_dist * Camera::degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        CameraFocus { defocus_angle, defocus_disk_u, defocus_disk_v }
    }
}

impl Camera {
    pub fn new(stats: CameraStats, view: CameraView, focus: CameraFocus) -> Camera {
        Camera { stats, view, focus }
    }

    pub fn stats(&self) -> CameraStats {
        self.stats
    }

    pub fn view(&self) -> CameraView {
        self.view
    }

    pub fn focus(&self) -> CameraFocus {
        self.focus
    }

    pub fn render(&self, world: &dyn Hittable, file_name: &str) -> io::Result<()> {
        let mut pic = format!("P3\n{} {}\n255\n", self.stats.image_width, self.stats.image_height);

        let samples_per_pixel = (1.0 / self.stats.pixel_samples_scale) as usize;
        let pixels = (0..self.stats.height() as usize).into_par_iter().map(|h| {
            (0..self.stats.width() as usize).into_par_iter().map(|w| {
                let mut pixel_color = Point3D::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let r = self.get_ray(w as f64, h as f64);
                    pixel_color = pixel_color + self.ray_color(&r, self.stats.max_depth, world);
                }
                pixel_color = pixel_color / samples_per_pixel as f64;
                pixel_color = Point3D::new(pixel_color.x().sqrt(), pixel_color.y().sqrt(), pixel_color.z().sqrt());
                let min: f64 = 0.000;
                let max: f64 = 0.999;
                let rbyte: usize = (256.0 * clamp(pixel_color.x(), min, max)) as usize;
                let gbyte: usize = (256.0 * clamp(pixel_color.y(), min, max)) as usize;
                let bbyte: usize = (256.0 * clamp(pixel_color.z(), min, max)) as usize;
                format!("{} {} {}\n", rbyte, gbyte, bbyte)
            }).collect::<Vec<String>>().join("")
        }).collect::<Vec<String>>().join("");
    
        pic = format!("{}{}", &pic, pixels);
        fs::write(file_name.to_string(), pic)?;
        Ok(())
    }

    pub fn ray_color(&self, r: &Ray, max_depth: usize, world: &dyn Hittable) -> Point3D { 
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if max_depth == 0 {
            Point3D::new(0.0, 0.0, 0.0)
        } else if let Some(rec) = world.hit(r, 0.001, f64::MAX) { 
            match Scatterable::scatter(&rec.material, r, &rec) {
                (Some(scattered), Some(accentuation)) => accentuation * self.ray_color(&scattered, max_depth - 1, world),
                _ => Point3D::new(0.0, 0.0, 0.0),
            }
        } else { 
            let unit_direction = r.direction().unit_vector();
            let a: f64 = 0.5 * (unit_direction.y() + 1.0);
            Point3D::new(1.0, 1.0, 1.0) * (1.0 - a) + Point3D::new(0.5, 0.7, 1.0) * a 
        } 
    }

    pub fn get_ray(&self, i: f64, j: f64) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.view.pixel00_loc  
            + (self.view.pixel_delta_u * (i + offset.x()))
            + (self.view.pixel_delta_v * (j + offset.y()));
        let ray_origin = if self.focus.defocus_angle <= 0.0 { self.view.center } else { self.defocus_disk_sample() }; 
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3D {
        let p = Point3D::random_in_unit_disk();
        self.view.center + (self.focus.defocus_disk_u * p.x()) + (self.focus.defocus_disk_v * p.y())
    }


    fn sample_square(&self) -> Point3D {
        Point3D::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    pub fn degrees_to_radians(deg: f64) -> f64 {
        deg * std::f64::consts::PI / 180.0
    }
}
