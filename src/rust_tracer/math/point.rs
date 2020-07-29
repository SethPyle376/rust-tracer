use std::ops::{Add, Sub, Mul, Div};

use crate::rust_tracer::math::EPSILON;
use crate::rust_tracer::math::vector::Vec4;

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Point {
    pub fn new(x : f32, y : f32, z : f32) -> Point {
        Point{x, y, z, w: 1.0}
    }

    pub fn equal(&self, other: &Point) -> bool {
        return (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON &&
            (self.z - other.z).abs() < EPSILON && (self.w - other.w).abs() < EPSILON;
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: 1.0}
    }
}

impl Add<Vec4> for Point {
    type Output = Point;

    fn add(self, other: Vec4) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: 1.0}
    }
}

impl Add<Vec4> for &Point {
    type Output = Point;

    fn add(self, other: Vec4) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: 1.0}
    }
}

impl Sub<&Point> for &Point {
    type Output = Vec4;

    fn sub(self, other: &Point) -> Vec4 {
        Vec4 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: 0.0}
    }
}

impl Sub<Vec4> for Point {
    type Output = Point;

    fn sub(self, other: Vec4) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: 1.0}
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, other: f32) -> Point {
        Point {x: self.x * other, y: self.y * other, z: self.z * other, w: 1.0}
    }
}

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, other: f32) -> Point {
        let reciprocal = 1.0 / other;
        Point {x: self.x * reciprocal, y: self.y * reciprocal, z: self.z * reciprocal, w: 1.0}
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w;
    }
}