use crate::point3d::*;
use crate::ray::*;
use crate::hittable::*;
use crate::material::*;

pub struct Sphere {
    center: Point3D,
    radius: f64,
    material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> { 
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let b = r.direction().dot(&oc) * -2.0 ;
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
    
        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let root_near = ((-b) - sqrtd) / (2.0 * a);
            let root_far = ((-b) + sqrtd) / (2.0 * a);
            for root in [root_near, root_far].iter() {
                if *root < ray_tmax && *root > ray_tmin {
                    let p = r.at(*root);
                    let normal = (p - self.center) / self.radius;
                    let front_face = r.direction().dot(&normal) < 0.0;
                    return Some(HitRecord::new(
                        p,
                        if front_face { normal } else { -normal },
                        *root,
                        front_face,
                        self.material,
                    ));
                }
            }
        }
        None
    }
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64, material: Material) -> Sphere {
        let r = if radius < 0.0 { 0.0 } else { radius };
        Sphere{ center, radius: r, material }
    }
}
