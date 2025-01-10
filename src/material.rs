use crate::point3d::*;
use crate::ray::*;
use crate::hittable::*;
use crate::random::*;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
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
            Material::Dielectric(d) => d.scatter(r_in, rec),
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
    pub albedo: Point3D,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Point3D, fuzz: f64) -> Metal {
        let f = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal{ albedo, fuzz: f }
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Point3D)> {
        let reflected = Point3D::reflect(&r_in.direction(), &rec.normal);
        let reflected_fuzz = Point3D::unit_vector(&reflected) + Point3D::random_unit_vector() * self.fuzz;
        let scattered = Ray::new(rec.p, reflected_fuzz); 
        let accentuation = self.albedo;
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((scattered, accentuation))
        } else { 
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric{ refraction_index }
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0_sqr = r0 * r0;
        r0_sqr + (1.0 - r0_sqr) * (1.0 - cosine).powi(5)
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Point3D)> {
        let attenuation = Point3D::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = if -unit_direction.dot(&rec.normal) < 1.0 { -unit_direction.dot(&rec.normal) } else { 1.0 };
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || self.reflectance(cos_theta, ri) > random_f64() { 
            unit_direction.reflect(&rec.normal)
        } else { 
            unit_direction.refract(&rec.normal, ri)
        };

        let scattered = Ray::new(rec.p, direction);
        Some((scattered, attenuation))
    }   
}
