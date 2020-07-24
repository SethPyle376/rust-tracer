use crate::rust_tracer::math::point::Point;

#[test]
fn new_point_test() {
    let point = Point::new(1.2, 2.2, 3.2);
    assert_eq!(point.x, 1.2);
    assert_eq!(point.y, 2.2);
    assert_eq!(point.z, 3.2);
    assert_eq!(point.w, 1.0);
}