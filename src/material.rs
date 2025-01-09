use crate::point3d::*;
use crate::ray::*;
use crate::hittable::*;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Point3D)>;
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Point3D
}

impl Scatterable for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Point3D)> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec),
            Material::Metal(m) => m.scatter(r_in, rec),
        }
    }
}

impl Lambertian {
    pub fn new(albedo: Point3D) -> Lambertian {
        Lambertian{ albedo }
    }

    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Point3D)> {
        let mut scatter_direction = rec.normal + Point3D::random_unit_vector();
        // Catch degenerate scatter direction
        if Point3D::near_zero(&scatter_direction) {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let accentuation = self.albedo;
        Some((scattered, accentuation))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Point3D
}

impl Metal {
    pub fn new(albedo: Point3D) -> Metal {
        Metal{ albedo }
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Point3D)> {
        let reflected = Point3D::reflect(&r_in.direction(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected); 
        let accentuation = self.albedo;
        Some((scattered, accentuation))
    }
}
