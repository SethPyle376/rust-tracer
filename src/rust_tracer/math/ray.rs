use crate::rust_tracer::math::point::Point;
use crate::rust_tracer::math::vector::Vec4;
use crate::rust_tracer::math::mat4::Mat4;


pub struct Ray {
    pub origin: Point,
    pub direction: Vec4
}

impl Ray {
    pub fn new(origin: Point, direction: Vec4) -> Ray {
        Ray {origin, direction}
    }

    pub fn position(&self, t: f32) -> Point {
        let magnitude = &self.direction * t;
        return &self.origin + magnitude;
    }

    pub fn transform(ray: &Ray, matrix: &Mat4) -> Ray {
        Ray {origin: matrix * &(ray.origin), direction: matrix * &(ray.direction)}
    }
}

