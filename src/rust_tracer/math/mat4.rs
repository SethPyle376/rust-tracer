use std::cmp::{PartialEq};
use std::ops::{Add,Sub,Mul};
use crate::rust_tracer::math::vector::Vec4;

pub struct Mat4 {
  pub values: Vec<Vec<f32>>
}

impl Mat4 {
  pub fn new() -> Mat4 {
    Mat4 {values: vec!{vec!{0.0; 4}; 4}}
  }

  pub fn identity() -> Mat4 {
    let mut matrix = Mat4::new();
    for x in 0..4 {
      matrix.values[x][x] = 1.0;
    }
    return matrix;
  }

  pub fn set_row(&mut self, other: &Vec4, row_index: usize) {
    self.values[row_index][0] = other.x;
    self.values[row_index][1] = other.y;
    self.values[row_index][2] = other.z;
    self.values[row_index][3] = other.w;
  }

  pub fn set_column(&mut self, other: &Vec4, column_index: usize) {
    self.values[0][column_index] = other.x;
    self.values[1][column_index] = other.y;
    self.values[2][column_index] = other.z;
    self.values[3][column_index] = other.w;
  }

  pub fn get_value(&self, row: usize, column: usize) -> f32 {
    return self.values[row][column];
  }

  pub fn set_value(&mut self, row: usize, column: usize, value: f32) {
    self.values[row][column] = value;
  }
}

impl PartialEq for Mat4 {
  fn eq(&self, other: &Mat4) -> bool {
    return self.values == other.values;
  }
}

impl Mul<Mat4> for Mat4 {
  type Output = Mat4;

  fn mul(self, other: Mat4) -> Mat4 {
    let mut matrix = Mat4::new();
    for y in 0..4 {
      for x in 0..4 {
        let value = self.values[y][0] * other.values[0][x] + self.values[y][1] * other.values[1][x] +
            self.values[y][2] * other.values[2][x] + self.values[y][3] * other.values[3][x];
        matrix.values[y][x] = value;
      }
    }
    return matrix;
  }
}

impl Mul<&Mat4> for &Mat4 {
  type Output = Mat4;

  fn mul(self, other: &Mat4) -> Mat4 {
    let mut matrix = Mat4::new();
    for y in 0..4 {
      for x in 0..4 {
        let value = self.values[y][0] * other.values[0][x] + self.values[y][1] * other.values[1][x] +
            self.values[y][2] * other.values[2][x] + self.values[y][3] * other.values[3][x];
        matrix.values[y][x] = value;
      }
    }
    return matrix;
  }
}