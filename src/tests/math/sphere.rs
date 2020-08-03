use crate::rust_tracer::math::sphere::Sphere;
use crate::rust_tracer::math::point::Point;
use crate::rust_tracer::math::ray::Ray;
use crate::rust_tracer::intersection::Intersection;
use crate::rust_tracer::math::{Vec4, Mat4};

#[test]
pub fn sphere_intersection_test() {
    let mut sphere1 = Sphere::new(Mat4::identity(), 0);
    let ray1 = Ray::new(Point::new(Vec4(0.0, 0.0, -5.0, 1.0)), Vec4::new(0.0, 0.0, 1.0, 0.0));

    let scaling_matrix = Mat4::new_scaling(2.0);
    sphere1.transform = scaling_matrix;

    let intersections1 = sphere1.intersect(&ray1);

    assert_eq!(intersections1[0].t, 3.0);
    assert_eq!(intersections1[1].t, 7.0)
}

#[test]
pub fn sphere_nearest_intersection_test() {
    let sphere1 = Sphere::new(Mat4::identity(), 0);
    let ray1 = Ray::new(Point::new(0.0, 0.0, -8.0), Vec4::new(0.0, 0.0, 1.0, 0.0));

    let intersections1 = sphere1.intersect(&ray1);

    let nearest_intersection = Intersection::nearest_intersection(&intersections1);

    assert_eq!(nearest_intersection.t, 7.0);
}