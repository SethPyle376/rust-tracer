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

  pub fn transpose(&mut self) {
    let mut temp_matrix = Mat4::new();

    for x in 0..4 {
      temp_matrix.set_column(&(self.get_row(x)), x);
    }

    self.values = temp_matrix.values;
  }

  pub fn set_row(&mut self, other: &Vec4, row_index: usize) {
    self.values[row_index][0] = other.x;
    self.values[row_index][1] = other.y;
    self.values[row_index][2] = other.z;
    self.values[row_index][3] = other.w;
  }

  pub fn get_row(&self, row_index: usize) -> Vec4 {
    Vec4 {
      x: self.values[row_index][0],
      y: self.values[row_index][1],
      z: self.values[row_index][2],
      w: self.values[row_index][3]
    }
  }

  pub fn set_column(&mut self, other: &Vec4, column_index: usize) {
    self.values[0][column_index] = other.x;
    self.values[1][column_index] = other.y;
    self.values[2][column_index] = other.z;
    self.values[3][column_index] = other.w;
  }

  pub fn get_column(&self, column_index: usize) -> Vec4 {
    Vec4 {
      x: self.values[0][column_index],
      y: self.values[1][column_index],
      z: self.values[2][column_index],
      w: self.values[3][column_index]
    }
  }

  pub fn get_value(&self, row: usize, column: usize) -> f32 {
    return self.values[row][column];
  }

  pub fn set_value(&mut self, row: usize, column: usize, value: f32) {
    self.values[row][column] = value;
  }

  pub fn get_submatrix(&self, row: usize, column: usize) -> Vec<f32> {
    let mut submatrix = vec!{};
    for y in 0..4 {
      for x in 0..4 {
        if y != row && x != column {
          submatrix.push(self.values[y][x]);
        }
      }
    }
    return submatrix;
  }

  pub fn get_sub_sub_matrix(submatrix: &Vec<f32>, row: usize, column: usize) -> Vec<f32> {
    let mut sub_sub_matrix = vec!{};
    for y in 0..3 {
      for x in 0..3 {
        if y != row && x != column {
          let index = y * 3 + x;
          sub_sub_matrix.push(submatrix[index]);
        }
      }
    }
    return sub_sub_matrix;
  }

  pub fn get_minor(submatrix: &Vec<f32>, row: usize, column: usize) -> f32 {
    let subsubmatrix = Mat4::get_sub_sub_matrix(submatrix, row, column);

    return subsubmatrix[0] * subsubmatrix[3] - subsubmatrix[1] * subsubmatrix[2];
  }

  pub fn get_cofactor(submatrix: &Vec<f32>, row: usize, column: usize) -> f32 {
    let minor = Mat4::get_minor(submatrix, row, column);

    let row_column = row + column;

    if row_column % 2 != 0 {
      return -minor;
    } else {
      return minor;
    }
  }

  pub fn get_sub_determinant(submatrix: &Vec<f32>) -> f32 {
    let mut accum = 0.0;

    for x in 0..3 {
      let value = submatrix[x] * Mat4::get_cofactor(submatrix, 0, x);
      accum += value;
    }

    return accum;
  }


  pub fn determinant(&self) -> f32 {
    let mut accum = 0.0;

    for x in 0..4 {
      let submatrix = self.get_submatrix(0, x);

      let mut value = Mat4::get_sub_determinant(&submatrix);

      if x % 2 != 0 {
        value = -value;
      }

      accum += self.values[0][x] * value;
    }

    return accum;
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