pub struct Mat4 {
  pub values: Vec<Vec<f32>>
}

impl Mat4 {
  pub fn new() -> Mat4 {
    Mat4 {values: vec!{vec!{0.0; 4}; 4}}
  }

  pub fn set_row(&mut self, other: Vec<f32>, row_index: usize) {
    self.values[row_index] = other;
  }

  pub fn set_column(&mut self, other: Vec<f32>, column_index: usize) {
    self.values[0][column_index] = other[0];
    self.values[1][column_index] = other[1];
    self.values[2][column_index] = other[2];
    self.values[3][column_index] = other[3];
  }

  pub fn get_value(&self, row: usize, column: usize) -> f32 {
    return self.values[row][column];
  }
}