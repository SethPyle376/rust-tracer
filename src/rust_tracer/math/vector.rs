use std::num;
use std::ops::{Add,Sub};

use crate::rust_tracer::math::EPSILON;
use crate::rust_tracer::math::point::Point;


pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec4 {
        Vec4{x, y, z, w: 0.0}
    }

    pub fn equal(&self, other: &Vec4) -> bool {
        return (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON &&
            (self.z - other.z).abs() < EPSILON && (self.w - other.w).abs() < EPSILON;
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