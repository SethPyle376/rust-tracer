use crate::rust_tracer::math::point::Point;
use crate::rust_tracer::math::vector::Vec4;
use crate::rust_tracer::math::mat4::Mat4;
use std::f32::consts::PI;

#[test]
fn new_point_test() {
    let point = Point::new(1.2, 2.2, 3.2);
    assert_eq!(point.x, 1.2);
    assert_eq!(point.y, 2.2);
    assert_eq!(point.z, 3.2);
    assert_eq!(point.w, 1.0);
}

#[test]
fn point_equal_test() {
    let point1 = Point::new(1.2, 3.2, 4.2);
    let point2 = Point::new(1.2, 3.2, 4.2);
    assert!(point1.equal(&point2));
}

#[test]
fn point_near_equal_test() {
    let point1 = Point::new(1.200001, 3.200001, 4.200001);
    let point2 = Point::new(1.2, 3.2, 4.2);
    assert!(point1.equal(&point2));
}

#[test]
fn point_not_equal_test() {
    let point1 = Point::new(1.3, 3.2, 4.2);
    let point2 = Point::new(1.2, 3.2, 4.2);
    assert!(!point1.equal(&point2));
}

#[test]
fn point_add_test() {
    let point1 = Point::new(1.0, 2.0, 3.0);
    let point2 = Point::new(1.0, 2.0, 3.0);

    let point3 = point1 + point2;

    assert_eq!(point3.x, 2.0);
    assert_eq!(point3.y, 4.0);
    assert_eq!(point3.z, 6.0);
}

#[test]
fn point_add_vec_test() {
    let point1 = Point::new(1.0, 2.0, 3.0);
    let vector1 = Vec4::new(1.0, 2.0, 3.0, 0.0);
    let point2 = point1 + vector1;

    assert_eq!(point2.x, 2.0);
    assert_eq!(point2.y, 4.0);
    assert_eq!(point2.z, 6.0);
}

#[test]
fn point_sub_test() {
    let point1 = Point::new(3.0, 2.0, 3.0);
    let point2 = Point::new(1.0, 2.0, 2.0);
    let vector1 = &point1 - &point2;

    assert_eq!(vector1.x, 2.0);
    assert_eq!(vector1.y, 0.0);
    assert_eq!(vector1.z, 1.0);
}

#[test]
fn point_sub_vec_test() {
    let point1 = Point::new(4.0, 1.0, 5.0);
    let vector1 = Vec4::new(1.0, 2.0, 3.0, 0.0);
    let point2 = point1 - vector1;

    assert_eq!(point2.x, 3.0);
    assert_eq!(point2.y, -1.0);
    assert_eq!(point2.z, 2.0);
}

#[test]
fn point_scalar_mul_test() {
    let point1 = Point::new(0.0, 2.0, -3.0);
    let point2 = point1 * 2.5;

    assert_eq!(point2.x, 0.0);
    assert_eq!(point2.y, 5.0);
    assert_eq!(point2.z, -7.5);
}

#[test]
fn point_scalar_div_test() {
    let point1 = Point::new(0.0, 2.0, -3.0);
    let point2 = point1 / 2.0;

    assert_eq!(point2.x, 0.0);
    assert_eq!(point2.y, 1.0);
    assert_eq!(point2.z, -1.5);
}

#[test]
fn point_translation_test() {
    let translation_matrix = Mat4::translation(5.0, -3.0, 2.0);
    let point1 = Point::new(-3.0, 4.0, 5.0);

    let point2 = &translation_matrix * &point1;

    let point3 = Point::new(2.0, 1.0, 7.0);

    assert_eq!(point2.x, point3.x);
    assert_eq!(point2.y, point3.y);
    assert_eq!(point2.z, point3.z);
}

#[test]
fn point_scaling_test() {
    let scaling_matrix = Mat4::scale(2.0, 3.0, 4.0);
    let point1 = Point::new(-4.0, 6.0, 8.0);

    let point2 = &scaling_matrix * &point1;

    let point3 = Point::new(-8.0, 18.0, 32.0);

    assert_eq!(point2.x, point3.x);
    assert_eq!(point2.y, point3.y);
    assert_eq!(point2.z, point3.z);
}

#[test]
fn point_x_rotation_test() {
    let point1 = Point::new(0.0, 1.0, 0.0);
    let rotation_matrix = Mat4::rotation_x(PI / 4.0);

    let point2 = &rotation_matrix * &point1;

    assert_eq!(point2.x, 0.0);
    assert_eq!(point2.y, 2.0_f32.sqrt() / 2.0);
    assert_eq!(point2.z, 2.0_f32.sqrt() / 2.0);
}

#[test]
fn point_y_rotation_test() {
    let point1 = Point::new(0.0, 0.0, 1.0);
    let rotation_matrix = Mat4::rotation_y(PI / 4.0);

    let point2 = &rotation_matrix * &point1;

    assert_eq!(point2.x, 2.0_f32.sqrt() / 2.0);
    assert_eq!(point2.y, 0.0);
    assert_eq!(point2.z, 2.0_f32.sqrt() / 2.0);
}

#[test]
fn point_z_rotation_test() {
    let point1 = Point::new(0.0, 1.0, 0.0);
    let rotation_matrix = Mat4::rotation_z(PI / 4.0);

    let point2 = &rotation_matrix * &point1;

    assert_eq!(point2.x, -(2.0_f32.sqrt()) / 2.0);
    assert_eq!(point2.y, 2.0_f32.sqrt() / 2.0);
    assert_eq!(point2.z, 0.0);
}

#[test]
fn point_chained_transformation_test() {
    let point1 = Point::new(1.0, 0.0, 1.0);
    let rotation_matrix = Mat4::rotation_x(PI / 2.0);
    let scaling_matrix = Mat4::scale(5.0, 5.0, 5.0);
    let translation_matrix = Mat4::translation(10.0, 5.0, 7.0);

    let chained_matrix = translation_matrix * scaling_matrix;
    let chained_matrix = chained_matrix * rotation_matrix;

    let point2 = &chained_matrix * &point1;


    assert_eq!(point2.x, 15.0);
    assert_eq!(point2.y, 0.0);
    assert_eq!(point2.z, 7.0);
}