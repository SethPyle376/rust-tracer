use crate::rust_tracer::math::point::Point;
use std::f32::consts::PI;
use crate::rust_tracer::math::{Vec4, Mat4};
use nalgebra::Vector3;

#[test]
fn new_point_test() {
    let point = Point::new(Vec4::new(1.2, 2.2, 3.2, 1.0));
    assert_eq!(point.point.x, 1.2);
    assert_eq!(point.point.y, 2.2);
    assert_eq!(point.point.z, 3.2);
    assert_eq!(point.point.w, 1.0);
}

#[test]
fn point_equal_test() {
    let point1 = Point::new(Vec4::new(1.2, 3.2, 4.2, 1.0));
    let point2 = Point::new(Vec4::new(1.2, 3.2, 4.2, 1.0));
    assert!(point1.equal(&point2));
}

#[test]
fn point_near_equal_test() {
    let point1 = Point::new(Vec4::new(1.200001, 3.200001, 4.200001, 1.0));
    let point2 = Point::new(Vec4::new(1.2, 3.2, 4.2, 1.0));
    assert!(point1.equal(&point2));
}

#[test]
fn point_not_equal_test() {
    let point1 = Point::new(Vec4::new(1.3, 3.2, 4.2, 1.0));
    let point2 = Point::new(Vec4::new(1.2, 3.2, 4.2, 1.0));
    assert!(!point1.equal(&point2));
}

#[test]
fn point_add_test() {
    let point1 = Point::new(Vec4::new(1.0, 2.0, 3.0, 1.0));
    let point2 = Point::new(Vec4::new(1.0, 2.0, 3.0, 1.0));

    let point3 = point1 + point2;

    assert_eq!(point3.point.x, 2.0);
    assert_eq!(point3.point.y, 4.0);
    assert_eq!(point3.point.z, 6.0);
}

#[test]
fn point_add_vec_test() {
    let point1 = Point::new(Vec4::new(1.0, 2.0, 3.0, 1.0));
    let vector1 = Vec4::new(1.0, 2.0, 3.0, 0.0);
    let point2 = point1 + vector1;

    assert_eq!(point2.point.x, 2.0);
    assert_eq!(point2.point.y, 4.0);
    assert_eq!(point2.point.z, 6.0);
}

#[test]
fn point_sub_test() {
    let point1 = Point::new(Vec4::new(3.0, 2.0, 3.0, 1.0));
    let point2 = Point::new(Vec4::new(1.0, 2.0, 2.0, 1.0));
    let vector1 = &point1 - &point2;

    assert_eq!(vector1.x, 2.0);
    assert_eq!(vector1.y, 0.0);
    assert_eq!(vector1.z, 1.0);
}

#[test]
fn point_sub_vec_test() {
    let point1 = Point::new(Vec4::new(4.0, 1.0, 5.0, 1.0));
    let vector1 = Vec4::new(1.0, 2.0, 3.0, 0.0);
    let point2 = point1 - vector1;

    assert_eq!(point2.point.x, 3.0);
    assert_eq!(point2.point.y, -1.0);
    assert_eq!(point2.point.z, 2.0);
}

#[test]
fn point_scalar_mul_test() {
    let point1 = Point::new(Vec4::new(0.0, 2.0, -3.0, 1.0));
    let point2 = point1 * 2.5;

    assert_eq!(point2.point.x, 0.0);
    assert_eq!(point2.point.y, 5.0);
    assert_eq!(point2.point.z, -7.5);
}

#[test]
fn point_scalar_div_test() {
    let point1 = Point::new(Vec4::new(0.0, 2.0, -3.0, 1.0));
    let point2 = point1 / 2.0;

    assert_eq!(point2.point.x, 0.0);
    assert_eq!(point2.point.y, 1.0);
    assert_eq!(point2.point.z, -1.5);
}

#[test]
fn point_translation_test() {
    let translation_matrix = Mat4::new_translation(&Vector3::new(5.0, -3.0, 2.0));
    let point1 = Point::new(Vec4::new(-3.0, 4.0, 5.0, 1.0));

    let point2 = Point { point: &translation_matrix * &point1.point };

    let point3 = Point::new(Vec4::new(2.0, 1.0, 7.0, 1.0));

    assert_eq!(point2.point.x, point3.point.x);
    assert_eq!(point2.point.y, point3.point.y);
    assert_eq!(point2.point.z, point3.point.z);
}

#[test]
fn point_scaling_test() {
    let scaling_matrix = Mat4::scale(&Mat4::identity(), 2.0);
    let point1 = Point::new(Vec4::new(-4.0, 6.0, 8.0, 1.0));

    let point2 = Point { point: &scaling_matrix * &point1.point };

    let point3 = Point::new(Vec4::new(-8.0, 12.0, 16.0, 1.0));

    assert_eq!(point2.point.x, point3.point.x);
    assert_eq!(point2.point.y, point3.point.y);
    assert_eq!(point2.point.z, point3.point.z);
}

#[test]
fn point_x_rotation_test() {
    let point1 = Point::new(Vec4::new(0.0, 1.0, 0.0, 1.0));
    let rotation_matrix = Mat4::from_euler_angles(PI / 4.0, 0.0, 0.0);

    let point2 = Point { point: &rotation_matrix * &point1.point };

    assert_eq!(point2.point.x, 0.0);
    assert_eq!(point2.point.y, 2.0_f32.sqrt() / 2.0);
    assert_eq!(point2.point.z, 2.0_f32.sqrt() / 2.0);
}

#[test]
fn point_chained_transformation_test() {
    let point1 = Point::new(Vec4::new(1.0, 0.0, 1.0, 1.0));
    let rotation_matrix = Mat4::new_rotation(Vector3::new(1.0, 0.0, 0.0) * (PI / 2.0));
    let scaling_matrix = Mat4::new_scaling(5.0);
    let translation_matrix = Mat4::new_translation(&Vector3::new(10.0, 5.0, 7.0));

    let chained_matrix = translation_matrix * scaling_matrix;
    let chained_matrix = chained_matrix * rotation_matrix;

    let point2 = Point { point: &chained_matrix * &point1.point };

    let rotated_vector = rotation_matrix * point1.point;


    assert_eq!(point2.point, Vec4::new(15.0, 0.0, 7.0, 1.0));
}