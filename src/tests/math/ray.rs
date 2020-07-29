use crate::rust_tracer::math::ray::Ray;
use crate::rust_tracer::math::point::Point;
use crate::rust_tracer::math::vector::Vec4;
use crate::rust_tracer::math::mat4::Mat4;

#[test]
fn ray_new_test() {
    let origin = Point::new(1.0, 0.0, 1.0);
    let direction = Vec4::new(2.0, 1.0, 0.0, 0.0);
    let ray1 = Ray::new(origin, direction);

    assert_eq!(ray1.origin.x, 1.0);
    assert_eq!(ray1.direction.x, 2.0);
}

#[test]
fn ray_position_test() {
    let origin = Point::new(2.0, 3.0, 4.0);
    let direction = Vec4::new(1.0, 0.0, 0.0, 0.0);
    let ray1 = Ray::new(origin, direction);

    assert_eq!(ray1.position(0.0), Point::new(2.0, 3.0, 4.0));
    assert_eq!(ray1.position(1.0), Point::new(3.0, 3.0, 4.0));
    assert_eq!(ray1.position(-1.0), Point::new(1.0, 3.0, 4.0));
    assert_eq!(ray1.position(2.5), Point::new(4.5, 3.0, 4.0));
}

#[test]
fn ray_transform_test() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vec4::new(0.0, 1.0, 0.0, 0.0);
    let mut ray1 = Ray::new(origin, direction);

    let translation_matrix = Mat4::translation(3.0, 4.0, 5.0);

    let ray1 = Ray::transform(&ray1, &translation_matrix);

    assert_eq!(ray1.origin, Point::new(4.0, 6.0, 8.0));
    assert_eq!(ray1.direction, Vec4::new(0.0, 1.0, 0.0, 0.0));
}