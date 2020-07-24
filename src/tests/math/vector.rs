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