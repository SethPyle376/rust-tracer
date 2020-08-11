use std::ops::{Add,Sub,Mul};
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

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color { color: Vec4::new(self.color.x * other.color.x, self.color.y * other.color.y, self.color.z * other.color.z, 0.0)}
    }
}

impl Mul<&Color> for &Color {
    type Output = Color;
    fn mul(self, other: &Color) -> Color {
        Color { color: Vec4::new(self.color.x * other.color.x, self.color.y * other.color.y, self.color.z * other.color.z, 0.0)}
    }
}

impl Mul<f32> for &Color {
    type Output = Color;
    fn mul(self, other: f32) -> Color {
        Color { color: Vec4::new(self.color.x * other, self.color.y * other, self.color.z * other, 0.0)}
    }
}