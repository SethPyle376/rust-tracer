use crate::rust_tracer::math::sphere::Sphere;
use crate::rust_tracer::math::point::Point;
use crate::rust_tracer::math::vector::Vec4;
use crate::rust_tracer::math::ray::Ray;

#[test]
pub fn sphere_determinant_test() {
    let sphere1 = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0, 0);
    let ray1 = Ray::new(Point::new(0.0, 0.0, -8.0), Vec4::new(0.0, 0.0, 1.0, 0.0));
    let ray2 = Ray::new(Point::new(0.0, 0.0, -5.0), Vec4::new(0.0, 1.0, 0.0, 0.0));

    let intersections1 = sphere1.intersect(&ray1);
    let intersections2 = sphere1.intersect(&ray2);

    assert!(!intersections1.is_empty());
    assert!(intersections2.is_empty());
}
