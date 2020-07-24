use crate::rust_tracer::math::mat4::Mat4;

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
}