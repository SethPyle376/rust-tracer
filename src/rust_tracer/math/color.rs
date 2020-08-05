use std::ops::{Add,Sub};
use crate::rust_tracer::math::Vec4;

#[derive(Clone)]
pub struct Color {
    pub color: Vec4
}

impl Color {
    pub fn new(color: Vec4) -> Color { Color { color } }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color { color: self.color + other.color }
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color { color: self.color - other.color }
    }
}