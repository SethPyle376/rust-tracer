use crate::rust_tracer::math::point::Point;
use crate::rust_tracer::math::ray::Ray;
use crate::rust_tracer::math::vector::Vec4;
use crate::rust_tracer::intersection::Intersection;
use crate::rust_tracer::math::mat4::Mat4;

pub struct Sphere {
    pub transform: Mat4,
    pub id: i32
}

impl Sphere {
    pub fn new(transform: Mat4, id: i32) -> Sphere { Sphere {transform, id}}

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let transformed_ray = Ray::transform(&ray, &Mat4::inverse(&(self.transform)));
        let mut intersections = vec!{};
        let sphere_to_ray = &transformed_ray.origin - &Point::new(0.0, 0.0, 0.0);
        let a = transformed_ray.direction.dot(&(transformed_ray.direction));
        let b = 2.0 * transformed_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

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