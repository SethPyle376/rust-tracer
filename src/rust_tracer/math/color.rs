use std::ops::{Add,Sub,Mul};

#[derive(Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color { Color{r, g, b} }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {r: self.r + other.r, g: self.g + other.g, b: self.b + other.b}
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {r: self.r - other.r, g: self.g - other.g, b: self.b - other.b}
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {r: self.r * other.r, g: self.g * other.g, b: self.b * other.b}
    }
}