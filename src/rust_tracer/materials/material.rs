use crate::rust_tracer::math::{Vec4, color::Color};

pub struct Material {
	pub color: Color,
	pub ambient: f32,
	pub diffuse: f32,
	pub specular: f32,
	pub shininess: f32
}

impl Material {
	pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Material {
		Material { color, ambient, diffuse, specular, shininess }
	}

	pub fn default_material() -> Material {
		Material { color: Color::new(Vec4::new(0.5, 0.5, 0.5, 0.0)), ambient: 0.2, diffuse: 0.2, specular: 0.3, shininess: 200.0}
	}
}