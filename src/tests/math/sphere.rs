use crate::rust_tracer::math::sphere::Sphere;
use crate::rust_tracer::math::point::Point;
use crate::rust_tracer::math::ray::Ray;
use crate::rust_tracer::intersection::Intersection;
use crate::rust_tracer::math::{Vec4, Mat4};
use nalgebra::Vector3;
use std::f32::consts::PI;

#[test]
pub fn sphere_intersection_test() {
    let mut sphere1 = Sphere::new(Mat4::identity(), 0);
    let ray1 = Ray::new(Point::new(Vec4::new(0.0, 0.0, -5.0, 1.0)), Vec4::new(0.0, 0.0, 1.0, 0.0));

    let scaling_matrix = Mat4::new_scaling(2.0);
    sphere1.transform = scaling_matrix;

    let intersections1 = sphere1.intersect(&ray1);

    assert_eq!(intersections1[0].t, 4.0);
    assert_eq!(intersections1[1].t, 6.0)
}

#[test]
pub fn sphere_nearest_intersection_test() {
    let sphere1 = Sphere::new(Mat4::identity(), 0);
    let ray1 = Ray::new(Point::new(Vec4::new(0.0, 0.0, -8.0, 1.0)), Vec4::new(0.0, 0.0, 1.0, 0.0));

    let intersections1 = sphere1.intersect(&ray1);

    let nearest_intersection = Intersection::nearest_intersection(&intersections1);

    assert_eq!(nearest_intersection.t, 7.0);
}

#[test]
pub fn sphere_normal_at_test() {
    let sphere1 = Sphere::new(Mat4::identity(), 0);

    let normal_vector = sphere1.normal_at(&Point { point: Vec4::new(1.0, 0.0, 0.0, 1.0) });

    assert_eq!(normal_vector, Vec4::new(1.0, 0.0, 0.0, 0.0));
}

#[test]
pub fn sphere_transformed_normal_at_test() {
    let scaling_matrix = Mat4::new_nonuniform_scaling(&Vector3::new(1.0, 0.5, 1.0));
    let rotation_matrix = Mat4::new_rotation(Vector3::new(0.0, 0.0, 1.0) * (PI / 5.0));
    let transformation_matrix = scaling_matrix * rotation_matrix;

    let sphere = Sphere::new(transformation_matrix, 0);

    let normal = sphere.normal_at(&Point::new(Vec4::new(0.0, 2.0_f32.sqrt() / 2.0, -(2.0_f32.sqrt()) / 2.0, 1.0)));

    assert_eq!(normal, Vec4::new(-0.000000020444226, 0.97014254, -0.24253564, 0.0));
}
