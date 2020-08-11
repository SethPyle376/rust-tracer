use crate::rust_tracer::math::Vec4;
use crate::rust_tracer::*;

#[test]
fn vector_reflect_test() {
	let incoming_vector = Vec4::new(1.0, -1.0, 0.0, 0.0);
	let normal_vector = Vec4::new(0.0, 1.0, 0.0, 0.0);

	let reflected_vector = utils::math::reflection(&incoming_vector, &normal_vector);

	assert_eq!(reflected_vector, Vec4::new(1.0, 1.0, 0.0, 0.0));
}

#[test]
fn vector_reflect_slanted_test() {
	let angle_value = 2.0_f32.sqrt() * 0.5;
	let incoming_vector = Vec4::new(0.0, -1.0, 0.0, 0.0);
	let normal_vector = Vec4::new(angle_value, angle_value, 0.0, 0.0);

	let mut reflected_vector = utils::math::reflection(&incoming_vector, & normal_vector);

	reflected_vector.x = utils::math::round_to_decimal(reflected_vector.x, 0.0);
	reflected_vector.y = utils::math::round_to_decimal(reflected_vector.y, 0.0);
	reflected_vector.z = utils::math::round_to_decimal(reflected_vector.z, 0.0);

	assert_eq!(reflected_vector, Vec4::new(1.0, 0.0, 0.0, 0.0));
}