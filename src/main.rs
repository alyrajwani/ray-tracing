mod color;
mod hittable;
mod point3d;
mod ray;
mod sphere;
mod camera;
mod random;
mod material;

use std::io;
use std::rc::Rc;
use crate::point3d::*;
use crate::hittable::*;
use crate::sphere::*;
use crate::camera::*;
use crate::material::*;

fn main() -> io::Result<()> {
    let mut world = HittableList::new(Vec::new());
    
    let material_ground = Material::Lambertian(Lambertian{ albedo: Point3D::new(0.8, 0.8, 0.0) });
    let material_center = Material::Lambertian(Lambertian{ albedo: Point3D::new(0.1, 0.2, 0.5) });
    let material_left = Material::Dielectric(Dielectric{ refraction_index: 1.50 });
    let material_right = Material::Metal(Metal{ albedo: Point3D::new(0.8, 0.6, 0.2), fuzz: 1.0 });

    world.list.push(Rc::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.list.push(Rc::new(Sphere::new(Point3D::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.list.push(Rc::new(Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.list.push(Rc::new(Sphere::new(Point3D::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 400.0;
    let samples_per_pixel: f64 = 100.0;
    let max_depth: usize = 50;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    let _ = camera.render(&world);

    Ok(())
}

