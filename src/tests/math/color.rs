use crate::rust_tracer::math::color::Color;
use crate::rust_tracer::math::Vec4;

#[test]
fn color_new_test() {
    let color1 = Color::new(Vec4::new(1.0, 2.0, -3.0, 0.0));

    assert_eq!(color1.color.x, 1.0);
    assert_eq!(color1.color.y, 2.0);
    assert_eq!(color1.color.z, -3.0);
}

#[test]
fn color_add_test() {
    let color1 = Color::new(Vec4::new(1.0, 2.0, -3.0, 0.0));
    let color2 = Color::new(Vec4::new(2.0, 3.4, 1.8, 0.0));
    let color3 = color1 + color2;

    assert_eq!(color3.color.x, 3.0);
    assert_eq!(color3.color.y, 5.4);
    assert_eq!(color3.color.z, -1.2);
}

#[test]
fn color_sub_test() {
    let color1 = Color::new(Vec4::new(1.0, 2.0, -3.0, 0.0));
    let color2 = Color::new(Vec4::new(2.0, 5.0, 1.8, 0.0));
    let color3 = color1 - color2;

    assert_eq!(color3.color.x, -1.0);
    assert_eq!(color3.color.y, -3.0);
    assert_eq!(color3.color.z, -4.8);
}

// #[test]
// fn color_mul_test() {
//     let color1 = Color::new(Vec4::new(1.0, 2.0, -1.0, 0.0));
//     let color2 = Color::new(Vec4::new(2.0, 3.4, 1.8, 0.0));
//     let color3 = color1 * color2;
//
//     assert_eq!(color3.color.x, 2.0);
//     assert_eq!(color3.color.y, 6.8);
//     assert_eq!(color3.color.z, -1.8);
// }