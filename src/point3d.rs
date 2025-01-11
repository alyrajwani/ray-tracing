use std::ops::*;
use crate::random::*;

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        self.x() < s && self.y() < s && self.z() < s
    }

    pub fn random() -> Point3D {
        Point3D{ x: random_f64(), y: random_f64(), z: random_f64() }
    }

    pub fn random_point_in_range(min: f64, max: f64) -> Point3D {
        Point3D{ 
            x: random_in_range(min, max),
            y: random_in_range(min, max),
            z: random_in_range(min, max),
        }
    }

    pub fn unit_vector(&self) -> Point3D {
        let length = self.length();
        Point3D{ x: self.x / length, y: self.y / length, z: self.z / length } 
    }

    pub fn random_in_unit_disk() -> Point3D {
        Point3D::new(
            random_in_range(-1.0, 1.0),
            random_in_range(-1.0, 1.0),
            0.0
        )
    }

    pub fn random_unit_vector() -> Point3D {
        loop {
            let p = Point3D::random_point_in_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / f64::sqrt(lensq);
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Point3D) -> Point3D {
        let on_unit_sphere = Point3D::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 { // In the same hemisphere as the normal
            on_unit_sphere 
        } else {
            -on_unit_sphere
        }
    }

    pub fn reflect(&self, normal: &Point3D) -> Point3D {
        *self - (*normal * self.dot(normal) * 2.0) 
    }

    pub fn refract(&self, normal: &Point3D, etai_over_etat: f64) -> Point3D {
        let cos_theta = if -self.dot(normal) < 1.0 { -self.dot(normal) } else { 1.0 };
        let r_out_perp = (*self + *normal * cos_theta) * etai_over_etat;
        let r_out_parallel = *normal * -f64::sqrt(1.0 - r_out_perp.length_squared());
        r_out_perp + r_out_parallel
    }

    pub fn dot(&self, other: &Point3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Point3D) -> Point3D {
        Point3D{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl Sub for Point3D {
    type Output = Point3D;

    fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl Neg for Point3D {
    type Output = Point3D;

    fn neg(self) -> Point3D {
        Point3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Point3D> for Point3D {
    type Output = Point3D;

    fn mul(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x * other.x(),
            y: self.y * other.y(),
            z: self.z * other.z(),
        }
    }
}

impl Mul<f64> for Point3D {
    type Output = Point3D;

    fn mul(self, other: f64) -> Point3D {
        Point3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<Point3D> for Point3D {
    type Output = Point3D;

    fn div(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x / other.x(),
            y: self.y / other.y(),
            z: self.z / other.z(),
        }
    }
}

impl Div<f64> for Point3D {
    type Output = Point3D;

    fn div(self, other: f64) -> Point3D {
        Point3D {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Point3D) -> bool {
        self.x == other.x() && self.y == other.y() && self.z == other.z()
    }
}
