use crate::rust_tracer::math::vector::Vec4;
use crate::rust_tracer::math::point::Point;

#[test]
fn new_vector_test() {
    let vector = Vec4::new(1.2, 3.2, 4.2);
    assert_eq!(vector.x, 1.2);
    assert_eq!(vector.y, 3.2);
    assert_eq!(vector.z, 4.2);
    assert_eq!(vector.w, 0.0);
}

#[test]
fn vector_equal_test() {
    let vector1 = Vec4::new(1.2, 3.2, 4.2);
    let vector2 = Vec4::new(1.2, 3.2, 4.2);
    assert!(vector1.equal(&vector2));
}

#[test]
fn vector_near_equal_test() {
    let vector1 = Vec4::new(1.200001, 3.200001, 4.200001);
    let vector2 = Vec4::new(1.2, 3.2, 4.2);
    assert!(vector1.equal(&vector2));
}

#[test]
fn vector_not_equal_test() {
    let vector1 = Vec4::new(1.3, 3.2, 4.2);
    let vector2 = Vec4::new(1.2, 3.2, 4.2);
    assert!(!vector1.equal(&vector2));
}

#[test]
fn vector_add_test() {
    let point1 = Vec4::new(1.0, 2.0, 3.0);
    let point2 = Vec4::new(1.0, 2.0, 3.0);

    let point3 = point1 + point2;

    assert_eq!(point3.x, 2.0);
    assert_eq!(point3.y, 4.0);
    assert_eq!(point3.z, 6.0);
}

#[test]
fn vector_add_point_test() {
    let point1 = Point::new(1.0, 2.0, 3.0);
    let vector1 = Vec4::new(1.0, 2.0, 3.0);
    let vector2 = vector1 + point1;

    assert_eq!(vector2.x, 2.0);
    assert_eq!(vector2.y, 4.0);
    assert_eq!(vector2.z, 6.0);
}

#[test]
fn vector_sub_test() {
    let vector1 = Vec4::new(3.0, 2.0, 5.0);
    let vector2 = Vec4::new(1.0, 2.0, 3.0);

    let vector3 = vector1 - vector2;

    assert_eq!(vector3.x, 2.0);
    assert_eq!(vector3.y, 0.0);
    assert_eq!(vector3.z, 2.0);
}

#[test]
fn vector_sub_point_test() {
    let vector1 = Vec4::new(1.0, 2.0, 8.0);
    let point1 = Point::new(4.0, 2.0, 3.0);
    let vector2 = vector1 - point1;

    assert_eq!(vector2.x, -3.0);
    assert_eq!(vector2.y, 0.0);
    assert_eq!(vector2.z, 5.0);
}

#[test]
fn vector_negate_test() {
    let vector1 = Vec4::new(0.0, 1.0, -1.0);
    let vector2 = -vector1;

    assert_eq!(vector2.x, 0.0);
    assert_eq!(vector2.y, -1.0);
    assert_eq!(vector2.z, 1.0);
}

#[test]
fn vector_scalar_mul_test() {
    let vector1 = Vec4::new(0.0, 2.0, -3.0);
    let vector2 = vector1 * 2.5;

    assert_eq!(vector2.x, 0.0);
    assert_eq!(vector2.y, 5.0);
    assert_eq!(vector2.z, -7.5);
}

#[test]
fn vector_scalar_div_test() {
    let vector1 = Vec4::new(0.0, 2.0, -3.0);
    let vector2 = vector1 / 2.0;

    assert_eq!(vector2.x, 0.0);
    assert_eq!(vector2.y, 1.0);
    assert_eq!(vector2.z, -1.5);
}

#[test]
fn vector_magnitude_test() {
    let vector1 = Vec4::new(1.0, 0.0, 0.0);
    let vector2 = Vec4::new(0.0, 1.0, 0.0);
    let vector3 = Vec4::new(0.0, 0.0, 1.0);
    let vector4 = Vec4::new(1.0, 2.0, 3.0);
    let vector5 = Vec4::new(-1.0, -2.0, -3.0);

    assert_eq!(vector1.magnitude(), 1.0);
    assert_eq!(vector2.magnitude(), 1.0);
    assert_eq!(vector3.magnitude(), 1.0);
    assert_eq!(vector4.magnitude(), 14.0_f32.sqrt());
    assert_eq!(vector5.magnitude(), 14.0_f32.sqrt());
}

#[test]
fn vector_normalize_test() {
    let mut vector1 = Vec4::new(1.0, 2.0, 3.0);
    vector1.normalize();

    assert_eq!(vector1.x, 1.0 / 14.0_f32.sqrt());
    assert_eq!(vector1.y, 2.0 / 14.0_f32.sqrt());
    assert_eq!(vector1.z, 3.0 / 14.0_f32.sqrt());
}

#[test]
fn vector_dot_product_test() {
    let vector1 = Vec4::new(1.0, 2.0, 3.0);
    let vector2 = Vec4::new(2.0, 3.0, 4.0);
    let dot_product = vector1.dot(&vector2);

    assert_eq!(dot_product, 20.0);
}

#[test]
fn vector_cross_product_test() {
    let vector1 = Vec4::new(1.0, 2.0, 3.0);
    let vector2 = Vec4::new(2.0, 3.0, 4.0);
    let vector3 = vector1.cross(&vector2);
    let vector4 = vector2.cross(&vector1);

    assert_eq!(vector3.x, -1.0);
    assert_eq!(vector3.y, 2.0);
    assert_eq!(vector3.z, -1.0);

    assert_eq!(vector4.x, 1.0);
    assert_eq!(vector4.y, -2.0);
    assert_eq!(vector4.z, 1.0)
}