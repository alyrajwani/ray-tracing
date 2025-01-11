use crate::ray::*;
use crate::point3d::*;
use std::rc::Rc;
use crate::material::*;

pub struct HitRecord {
    pub p: Point3D,
    pub normal: Point3D,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HitRecord {
    pub fn new(p: Point3D, normal: Point3D, t: f64, front_face: bool, material: Material) -> HitRecord {
        HitRecord{ p, normal, t, front_face, material }
    }
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList { 
        HittableList { list } 
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut closest_so_far = ray_tmax;
        let mut hit_anything: Option<HitRecord> = None;
        for hittable in self.list.iter() {
            if let Some(hit) = hittable.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }
}
