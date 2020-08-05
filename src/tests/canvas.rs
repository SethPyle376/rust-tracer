use crate::rust_tracer::canvas::Canvas;
use crate::rust_tracer::math::color::Color;
use crate::rust_tracer::math::sphere::Sphere;
use crate::rust_tracer::math::point::Point;
use crate::rust_tracer::math::ray::Ray;
use crate::rust_tracer::math::{Vec4, Mat4};

#[test]
fn canvas_pixel_test() {
    let mut canvas = Canvas::new(1280, 720);
    let color1 = Color::new(Vec4::new(1.0, 1.0, 1.0, 0.0));

    canvas.write_pixel(1279, 719, color1);
    let color2 = canvas.pixel_at(1279, 719);

    assert_eq!(color2.color.x, 1.0);
    assert_eq!(color2.color.y, 1.0);
    assert_eq!(color2.color.z, 1.0);
}

#[test]
#[ignore]
fn canvas_save_test() {
    let mut canvas = Canvas::new(1280, 720);
    let color1 = Color::new(Vec4::new(1.0, 1.0, 1.0, 0.0));

    canvas.write_pixel(1279, 719, color1);

    canvas.save("test.ppm".to_string());
}

#[test]
#[ignore]
fn basic_ray_tracing_test() {
    let canvas_dimension = 1000;
    let wall_size = 7.0;
    let mut canvas = Canvas::new(canvas_dimension, canvas_dimension);

    let pixel_size = wall_size / canvas_dimension as f32;
    let half = wall_size / 2.0;

    let sphere = Sphere::new(Mat4::identity() * Mat4::new_scaling(0.5), 0);


    for y in 0..(canvas_dimension - 1) {
        let world_y = half - pixel_size * y as f32;
        for x in 0..(canvas_dimension - 1) {
            let world_x = -half + pixel_size * x as f32;

            let position = Point::new(Vec4::new(world_x, world_y, 10.0, 1.0));

            let direction = &position - &Point::new(Vec4::new(0.0, 0.0, -5.0, 1.0));
            let direction = direction.normalize();

            let ray = Ray::new(Point::new(Vec4::new(0.0, 0.0, -5.0, 1.0)), direction);

            let intersection = sphere.intersect(&ray);

            let mut color = Color::new(Vec4::new(0.0, 0.0, 0.0, 1.0));

            if !intersection.is_empty() {
                color.color.x = 1.0;
            }

            canvas.write_pixel(x, y, color);
        }
    }

    canvas.save("basic_sphere_test.ppm".to_string());
}