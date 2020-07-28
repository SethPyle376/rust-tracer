pub fn round_to_decimal(value: f32, places: f32) -> f32 {
    let round_factor = 10.0f32.powf(places);
    let return_value = (value * round_factor).round() / round_factor;
    return return_value;
}