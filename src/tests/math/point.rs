use crate::rust_tracer::math::point::Point;
use crate::rust_tracer::math::vector::Vec4;

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
    let vector1 = Vec4::new(1.0, 2.0, 3.0);
    let point2 = point1 + vector1;

    assert_eq!(point2.x, 2.0);
    assert_eq!(point2.y, 4.0);
    assert_eq!(point2.z, 6.0);
}