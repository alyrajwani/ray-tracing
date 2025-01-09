mod color;
mod hittable;
mod point3d;
mod ray;
mod sphere;
mod camera;

use std::io;
use std::rc::Rc;
use crate::point3d::*;
use crate::hittable::*;
use crate::sphere::*;
use crate::camera::*;

fn main() -> io::Result<()> {
    let mut world = HittableList::new(Vec::new());
    world.list.push(Rc::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5)));
    world.list.push(Rc::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0)));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 400.0;
    let camera = Camera::new(aspect_ratio, image_width);

    let _ = camera.render(&world);

    Ok(())
}
