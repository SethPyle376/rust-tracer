use crate::rust_tracer::canvas::Canvas;
use crate::rust_tracer::math::color::Color;

#[test]
fn canvas_pixel_test() {
    let mut canvas = Canvas::new(1280, 720);
    let color1 = Color::new(1.0, 1.0, 1.0);

    canvas.write_pixel(1279, 719, color1);
    let color2 = canvas.pixel_at(1279, 719);

    assert_eq!(color2.r, 1.0);
    assert_eq!(color2.g, 1.0);
    assert_eq!(color2.b, 1.0);
}