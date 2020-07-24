use crate::rust_tracer::math::point::Point;

#[test]
fn new_point_test() {
    let point = Point::new();
    assert_eq!(point.x, 0.0);
    assert_eq!(point.y, 0.0);
    assert_eq!(point.z, 0.0);
    assert_eq!(point.w, 1.0);
}