pub struct Intersection {
    pub t: f32,
    pub object_intersected: i32
}

impl Intersection {
    pub fn new(t: f32, object_intersected: i32) -> Intersection {
        Intersection {t, object_intersected}
    }
}