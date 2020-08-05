use crate::rust_tracer::math::Vec4;

pub fn round_to_decimal(value: f32, places: f32) -> f32 {
    let round_factor = 10.0f32.powf(places);
    let return_value = (value * round_factor).round() / round_factor;
    return return_value;
}

pub fn reflection(incoming_vector: &Vec4, surface_normal: &Vec4) -> Vec4 {
	return incoming_vector - surface_normal * (2.0 * incoming_vector.dot(surface_normal));
}