use std::clone::Clone;
use std::fs::File;
use std::string::ToString;
use crate::rust_tracer::math::color::Color;
use std::io::Write;

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

    pub fn save(&self, filename: String) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write(b"P3\n")?;
        file.write(format!("{} {}\n", self.width, self.height).as_bytes())?;
        file.write(b"255\n");

        for i in self.pixels.iter().flatten() {
            let r = i.r / 1.0 * 255.0;
            let r = r as i64;

            let g = i.g / 1.0 * 255.0;
            let g = g as i64;

            let b = i.b / 1.0 * 255.0;
            let b = b as i64;

            file.write(format!("{} {} {}\n", r, g, b).as_bytes());
        }

        Ok(())
    }
}