use crate::rust_tracer::math::color::Color;
use std::clone::Clone;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut canvas = Canvas{width, height, pixels: Vec::new()};
        let default_color = Color::new(0.0, 0.0, 0.0);
        canvas.pixels = vec![vec![default_color; height]; width];
        return canvas;
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[x][y] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &(self.pixels[x][y])
    }
}