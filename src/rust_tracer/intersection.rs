pub struct Intersection {
    pub t: f32,
    pub object_intersected: i32
}

impl Intersection {
    pub fn new(t: f32, object_intersected: i32) -> Intersection {
        Intersection {t, object_intersected}
    }

    pub fn nearest_intersection(intersections: &Vec<Intersection>) -> Intersection {
        let mut current_index = 0;
        let mut lowest_value = std::f32::MAX;

        for (i, intersection) in intersections.iter().enumerate() {
            if intersection.t >= 0.0 {
                if intersection.t < lowest_value {
                    current_index = i;
                    lowest_value = intersection.t;
                }
            }
        }

        Intersection {t: intersections[current_index].t, object_intersected: intersections[current_index].object_intersected}
    }
}