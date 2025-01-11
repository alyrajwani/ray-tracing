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
use crate::random::*;
pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

fn main() -> io::Result<()> {
    let mut world = HittableList::new(Vec::new());
    
    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = random_f64();
            let center = Point3D::new(0.9 * random_f64() + (a as f64), 0.2, 0.9 * random_f64() + (b as f64));

            let sphere_material = if (center - Point3D::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse 
                    let albedo = Point3D::random() * Point3D::random();
                    Some(Material::Lambertian(Lambertian{ albedo } ))
                } else if choose_mat < 0.95 { 
                    // matte 
                    let albedo = Point3D::random_point_in_range(0.5, 1.0);
                    let fuzz = random::random_in_range(0.0, 0.5);
                    Some(Material::Metal(Metal { albedo, fuzz } ))
                } else {
                    // glass 
                    Some(Material::Dielectric(Dielectric{ refraction_index: 1.5 } ))
                }
            } else {
                None
            };
            
            if let Some(material) = sphere_material { 
                world.list.push(Rc::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material_one = Material::Dielectric(Dielectric { refraction_index: 1.5 } );
    world.list.push(Rc::new(Sphere::new(Point3D::new(0.0, 1.0, 0.0), 1.0, material_one)));

    let material_two = Material::Lambertian(Lambertian { albedo: Point3D::new(0.4, 0.2, 0.1) } );
    world.list.push(Rc::new(Sphere::new(Point3D::new(-4.0, 1.0, 0.0), 1.0, material_two)));

    let material_three = Material::Metal(Metal { albedo: Point3D::new(0.7, 0.6, 0.5), fuzz: 0.0 } );
    world.list.push(Rc::new(Sphere::new(Point3D::new(4.0, 1.0, 0.0), 1.0, material_three)));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 1200.0;
    let samples_per_pixel: f64 = 500.0;
    let max_depth: usize = 50;

    let vfov: f64 = 20.0;
    let lookfrom = Point3D::new(13.0, 2.0, 3.0);
    let lookat = Point3D::new(0.0, 0.0, 0.0);
    let vup = Point3D::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist);

    let _ = camera.render(&world);

    Ok(())
}

