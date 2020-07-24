use std::num;

const EPSILON: f32 = 0.00001;

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