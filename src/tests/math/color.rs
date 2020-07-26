use crate::rust_tracer::math::color::Color;

#[test]
fn color_new_test() {
    let color1 = Color::new(1.0, 2.0, -3.0);

    assert_eq!(color1.r, 1.0);
    assert_eq!(color1.g, 2.0);
    assert_eq!(color1.b, -3.0);
}

#[test]
fn color_add_test() {
    let color1 = Color::new(1.0, 2.0, -3.0);
    let color2 = Color::new(2.0, 3.4, 1.8);
    let color3 = color1 + color2;

    assert_eq!(color3.r, 3.0);
    assert_eq!(color3.g, 5.4);
    assert_eq!(color3.b, -1.2);
}

#[test]
fn color_sub_test() {
    let color1 = Color::new(1.0, 2.0, -3.0);
    let color2 = Color::new(2.0, 5.0, 1.8);
    let color3 = color1 - color2;

    assert_eq!(color3.r, -1.0);
    assert_eq!(color3.g, -3.0);
    assert_eq!(color3.b, -4.8);
}

#[test]
fn color_mul_test() {
    let color1 = Color::new(1.0, 2.0, -1.0);
    let color2 = Color::new(2.0, 3.4, 1.8);
    let color3 = color1 * color2;

    assert_eq!(color3.r, 2.0);
    assert_eq!(color3.g, 6.8);
    assert_eq!(color3.b, -1.8);
}