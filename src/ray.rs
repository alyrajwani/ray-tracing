use crate::point3d::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
   pub orig: Point3D,
   pub dir: Point3D,
}

impl Ray {
    pub fn new(orig: Point3D, dir: Point3D) -> Ray {
        Ray{ orig, dir }
    }

    pub fn origin(&self) -> Point3D {
        self.orig
    }

    pub fn direction(&self) -> Point3D {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.orig + self.dir * t
    }
}
