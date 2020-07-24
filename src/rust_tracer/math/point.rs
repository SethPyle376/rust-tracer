pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Point {
    pub fn new() -> Point {
        Point{x : 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }
}