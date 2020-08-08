use crate::rust_tracer::math::{color::Color, point::Point};

pub struct PointLight {
	pub intensity: Color,
	pub position: Point
}

impl PointLight {
	pub fn new(intensity: Color, position: Point) -> PointLight { PointLight { intensity, position }}
}