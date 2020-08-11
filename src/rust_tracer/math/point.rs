use std::ops::{Add, Sub, Mul, Div};

use crate::rust_tracer::math::{EPSILON, Vec4};

#[derive(Debug,Copy, Clone)]
pub struct Point {
    pub point: Vec4
}

impl Point {
    pub fn new(point: Vec4) -> Point {
        Point{ point }
    }

    pub fn equal(&self, other: &Point) -> bool {
        return (self.point.x - other.point.x).abs() < EPSILON && (self.point.y - other.point.y).abs() < EPSILON &&
            (self.point.z - other.point.z).abs() < EPSILON && (self.point.w - other.point.w).abs() < EPSILON;
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { point: self.point + other.point }
    }
}

impl Add<Vec4> for Point {
    type Output = Point;

    fn add(self, other: Vec4) -> Point {
        Point { point: self.point + other }
    }
}

impl Add<Vec4> for &Point {
    type Output = Point;

    fn add(self, other: Vec4) -> Point {
        Point { point: &self.point + other }
    }
}

impl Sub<&Point> for &Point {
    type Output = Vec4;

    fn sub(self, other: &Point) -> Vec4 {
        return &self.point - &other.point;
    }
}

impl Sub<Vec4> for Point {
    type Output = Point;

    fn sub(self, other: Vec4) -> Point {
        Point {point: self.point - other}
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, other: f32) -> Point {
        Point {point: self.point * other}
    }
}

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, other: f32) -> Point {
        let reciprocal = 1.0 / other;
        Point { point: self.point * reciprocal }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.point == other.point;
    }
}