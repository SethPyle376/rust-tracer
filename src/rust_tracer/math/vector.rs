use std::num;
use std::ops::{Add,Sub,Mul,Div,Neg};

use crate::rust_tracer::math::EPSILON;
use crate::rust_tracer::math::point::Point;

pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4{x, y, z, w}
    }

    pub fn equal(&self, other: &Vec4) -> bool {
        return (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON &&
            (self.z - other.z).abs() < EPSILON && (self.w - other.w).abs() < EPSILON;
    }

    pub fn magnitude(&self) -> f32 {
        let x_value = self.x * self.x;
        let y_value = self.y * self.y;
        let z_value = self.z * self.z;

        return (x_value + y_value + z_value).sqrt();
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.x = self.x / magnitude;
        self.y = self.y / magnitude;
        self.z = self.z / magnitude;
    }

    pub fn dot(&self, other: &Vec4) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(&self, other: &Vec4) -> Vec4 {
        Vec4 {x: self.y * other.z - self.z * other.y, y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x, w: 0.0}
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: 0.0}
    }
}

impl Add<Point> for Vec4 {
    type Output = Vec4;

    fn add(self, other: Point) -> Vec4 {
        Vec4 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: 0.0}
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Vec4 {
        Vec4 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: 0.0}
    }
}

impl Sub<Point> for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Point) -> Vec4 {
        Vec4 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: 0.0}
    }
}

impl Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Vec4 {
        Vec4 {x: -self.x, y: -self.y, z: -self.z, w: 0.0}
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: f32) -> Vec4 {
        Vec4 {x: self.x * other, y: self.y * other, z: self.z * other, w: 0.0}
    }
}

impl Mul<f32> for &Vec4 {
    type Output = Vec4;

    fn mul(self, other: f32) -> Vec4 {
        Vec4 {x: self.x * other, y: self.y * other, z: self.z * other, w: 0.0}
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, other: f32) -> Vec4 {
        let reciprocal = 1.0 / other;
        Vec4 {x: self.x * reciprocal, y: self.y * reciprocal, z: self.z * reciprocal, w: 0.0}
    }
}