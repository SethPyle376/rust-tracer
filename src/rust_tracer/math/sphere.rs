use crate::rust_tracer::math::point::Point;
use crate::rust_tracer::math::ray::Ray;
use crate::rust_tracer::math::vector::Vec4;
use crate::rust_tracer::intersection::Intersection;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub id: i32
}

impl Sphere {
    pub fn new(center: Point, radius: f32, id: i32) -> Sphere { Sphere {center, radius, id}}

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = vec!{};
        let sphere_to_ray = &ray.origin - &self.center;
        let a = ray.direction.dot(&(ray.direction));
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - self.radius.powi(2);

        let discriminant =  b.powi(2) - 4.0 * a * c;

        return if discriminant < 0.0 {
            intersections
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            if t1 > t2 {
                intersections.push(Intersection { t: t2, object_intersected: self.id });
                intersections.push(Intersection { t: t1, object_intersected: self.id });
            } else {
                intersections.push(Intersection { t: t1, object_intersected: self.id });
                intersections.push(Intersection { t: t2, object_intersected: self.id });
            }
            intersections
        }

    }
}