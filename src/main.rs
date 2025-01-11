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

fn three_balls() -> HittableList {
    let mut world = HittableList::new(Vec::new());

    let material_ground = Material::Lambertian(Lambertian{ albedo: Point3D::new(0.8, 0.8, 0.0) });
    let material_center = Material::Lambertian(Lambertian{ albedo: Point3D::new(0.1, 0.2, 0.5) });
    let material_left = Material::Dielectric(Dielectric{ refraction_index: 1.50 });
    let material_bubble = Material::Dielectric(Dielectric{ refraction_index: 1.00 / 1.50 }); 
    let material_right = Material::Metal(Metal{ albedo: Point3D::new(0.8, 0.6, 0.2), fuzz: 1.0 });

    world.list.push(Rc::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.list.push(Rc::new(Sphere::new(Point3D::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.list.push(Rc::new(Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.list.push(Rc::new(Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.list.push(Rc::new(Sphere::new(Point3D::new(1.0, 0.0, -1.0), 0.5, material_right)));

    world
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new(Vec::new());
    
    let ground_material = Material::Lambertian(Lambertian { albedo: Point3D::new(0.5, 0.5, 0.5) } );
    world.list.push(Rc::new(Sphere::new(Point3D::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
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

    world
}

fn camera_three_ball() -> Camera {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 400.0;
    let samples_per_pixel: f64 = 100.0;
    let max_depth: usize = 50;
   
    let stats = CameraStats::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    let vfov: f64 = 20.0;
    let lookfrom = Point3D::new(-2.0, 2.0, 1.0);
    let lookat = Point3D::new(0.0, 0.0, -1.0);
    let vup = Point3D::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    
    let view = CameraView::new(stats, vfov, lookfrom, lookat, vup, focus_dist);

    let defocus_angle = 0.0;

    let focus = CameraFocus::new(view, defocus_angle);
    
    Camera::new(stats, view, focus)
}

fn camera_random() -> Camera {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 800.0;
    let samples_per_pixel: f64 = 10.0;
    let max_depth: usize = 20;

    let stats = CameraStats::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    let vfov: f64 = 20.0;
    let lookfrom = Point3D::new(13.0, 2.0, 3.0);
    let lookat = Point3D::new(0.0, 0.0, 0.0);
    let vup = Point3D::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;

    let view = CameraView::new(stats, vfov, lookfrom, lookat, vup, focus_dist);
    
    let defocus_angle = 0.6;
    
    let focus = CameraFocus::new(view, defocus_angle);
    
    Camera::new(stats, view, focus)
}

fn main() -> io::Result<()> {
    let world = random_scene(); 
    let camera = camera_random();

    let _ = camera.render(&world);

    Ok(())
}

