use crate::rust_tracer::math::mat4::Mat4;
use crate::rust_tracer::math::vector::Vec4;

#[test]
fn mat4_row_test() {
  let mut mat4 = Mat4::new();
  let vector = Vec4::new(0.0, 1.0, 2.0, 0.0);
  mat4.set_row(&vector, 0);

  assert_eq!(mat4.get_value(0, 0), 0.0);
  assert_eq!(mat4.get_value(0, 1), 1.0);
  assert_eq!(mat4.get_value(0, 2), 2.0);
  assert_eq!(mat4.get_value(0, 3), 0.0);
}

#[test]
fn mat4_column_test() {
  let mut mat4 = Mat4::new();
  let vector = Vec4::new(0.0, 1.0, 2.0, 0.0);
  mat4.set_column(&vector, 0);

  assert_eq!(mat4.get_value(0, 0), 0.0);
  assert_eq!(mat4.get_value(1, 0), 1.0);
  assert_eq!(mat4.get_value(2, 0), 2.0);
  assert_eq!(mat4.get_value(3, 0), 0.0);
}

#[test]
fn mat4_equality_test() {
  let mut matrix1 = Mat4::new();
  let mut matrix2 = Mat4::new();
  let mut matrix3 = Mat4::new();

  let vector1 = Vec4::new(12.0, 2.0, 3.0, 0.0);

  matrix1.set_row(&vector1, 2);
  matrix2.set_row(&vector1, 2);
  matrix3.set_column(&vector1, 3);

  assert!(matrix1 == matrix2);
  assert!(matrix2 == matrix1);
  assert!(!(matrix1 == matrix3));
  assert!(!(matrix3 == matrix2));
}

#[test]
fn mat4_mul_test() {
  let mut matrix1 = Mat4::new();
  let mut matrix2 = Mat4::new();
  let mut matrix3 = Mat4::new();

  matrix1.set_row(&(Vec4::new(1.0, 2.0, 3.0, 4.0)), 0);
  matrix1.set_row(&(Vec4::new(5.0, 6.0, 7.0, 8.0)), 1);
  matrix1.set_row(&(Vec4::new(9.0, 8.0, 7.0, 6.0)), 2);
  matrix1.set_row(&(Vec4::new(5.0, 4.0, 3.0, 2.0)), 3);

  matrix2.set_row(&(Vec4::new(-2.0, 1.0, 2.0, 3.0)), 0);
  matrix2.set_row(&(Vec4::new(3.0, 2.0, 1.0, -1.0)), 1);
  matrix2.set_row(&(Vec4::new(4.0, 3.0, 6.0, 5.0)), 2);
  matrix2.set_row(&(Vec4::new(1.0, 2.0, 7.0, 8.0)), 3);

  matrix3.set_row(&(Vec4::new(20.0, 22.0, 50.0, 48.0)), 0);
  matrix3.set_row(&(Vec4::new(44.0, 54.0, 114.0, 108.0)), 1);
  matrix3.set_row(&(Vec4::new(40.0, 58.0, 110.0, 102.0)), 2);
  matrix3.set_row(&(Vec4::new(16.0, 26.0, 46.0, 42.0)), 3);

  let matrix4 = matrix1 * matrix2;

  assert!(matrix3 == matrix4);
}

#[test]
fn mat4_mul_identity_test() {
  let mut matrix1 = Mat4::new();
  let mut matrix2 = Mat4::identity();

  matrix1.set_row(&(Vec4::new(1.0, 2.0, 3.0, 0.0)), 0);
  matrix1.set_row(&(Vec4::new(1.0, 2.0, 3.0, 0.0)), 1);
  matrix1.set_row(&(Vec4::new(1.0, 2.0, 3.0, 0.0)), 2);
  matrix1.set_row(&(Vec4::new(1.0, 2.0, 3.0, 0.0)), 3);

  let matrix3 = &matrix1 * &matrix2;

  assert!(matrix1 == matrix3);
}

#[test]
fn mat4_transpose_test() {
  let mut matrix1 = Mat4::new();
  let mut matrix2 = Mat4::new();

  matrix1.set_row(&(Vec4::new(0.0, 9.0, 3.0, 0.0)), 0);
  matrix1.set_row(&(Vec4::new(9.0, 8.0, 0.0, 8.0)), 1);
  matrix1.set_row(&(Vec4::new(1.0, 8.0, 5.0, 3.0)), 2);
  matrix1.set_row(&(Vec4::new(0.0, 0.0, 5.0, 8.0)), 3);

  matrix2.set_row(&(Vec4::new(0.0, 9.0, 1.0, 0.0)), 0);
  matrix2.set_row(&(Vec4::new(9.0, 8.0, 8.0, 0.0)), 1);
  matrix2.set_row(&(Vec4::new(3.0, 0.0, 5.0, 5.0)), 2);
  matrix2.set_row(&(Vec4::new(0.0, 8.0, 3.0, 8.0)), 3);

  matrix1.transpose();

  assert!(matrix1 == matrix2);
}

#[test]
fn mat4_submatrix_test() {
  let mut matrix1 = Mat4::new();

  matrix1.set_row(&(Vec4::new(0.0, 9.0, 3.0, 0.0)), 0);
  matrix1.set_row(&(Vec4::new(9.0, 8.0, 0.0, 8.0)), 1);
  matrix1.set_row(&(Vec4::new(1.0, 8.0, 5.0, 3.0)), 2);
  matrix1.set_row(&(Vec4::new(0.0, 0.0, 5.0, 8.0)), 3);

  let submatrix = matrix1.get_submatrix(1, 2);

  assert_eq!(submatrix[0], 0.0);
  assert_eq!(submatrix[1], 9.0);
  assert_eq!(submatrix[2], 0.0);
  assert_eq!(submatrix[3], 1.0);
  assert_eq!(submatrix[4], 8.0);
  assert_eq!(submatrix[5], 3.0);
  assert_eq!(submatrix[6], 0.0);
  assert_eq!(submatrix[7], 0.0);
  assert_eq!(submatrix[8], 8.0);
}

#[test]
fn mat4_subsubmatrix_test() {
  let mut matrix1 = Mat4::new();

  matrix1.set_row(&(Vec4::new(0.0, 9.0, 3.0, 0.0)), 0);
  matrix1.set_row(&(Vec4::new(9.0, 8.0, 0.0, 8.0)), 1);
  matrix1.set_row(&(Vec4::new(1.0, 8.0, 5.0, 3.0)), 2);
  matrix1.set_row(&(Vec4::new(0.0, 0.0, 5.0, 8.0)), 3);
  let submatrix = matrix1.get_submatrix(1, 2);
  let subsubmatrix = Mat4::get_sub_sub_matrix(&submatrix, 0, 0);

  assert_eq!(subsubmatrix[0], 8.0);
  assert_eq!(subsubmatrix[1], 3.0);
  assert_eq!(subsubmatrix[2], 0.0);
  assert_eq!(subsubmatrix[3], 8.0);
}

#[test]
fn mat4_minor_test() {
  let mut matrix1 = Mat4::new();

  matrix1.set_row(&(Vec4::new(0.0, 9.0, 3.0, 0.0)), 0);
  matrix1.set_row(&(Vec4::new(9.0, 8.0, 0.0, 8.0)), 1);
  matrix1.set_row(&(Vec4::new(1.0, 8.0, 5.0, 3.0)), 2);
  matrix1.set_row(&(Vec4::new(0.0, 0.0, 5.0, 8.0)), 3);

  let submatrix = matrix1.get_submatrix(1, 2);

  let minor = Mat4::get_minor(&submatrix, 0, 0);

  assert_eq!(minor, 64.0);
}

#[test]
fn mat4_cofactor_test() {
  let mut matrix1 = Mat4::new();

  matrix1.set_row(&(Vec4::new(0.0, 9.0, 3.0, 0.0)), 0);
  matrix1.set_row(&(Vec4::new(9.0, 8.0, 0.0, 8.0)), 1);
  matrix1.set_row(&(Vec4::new(1.0, 8.0, 5.0, 3.0)), 2);
  matrix1.set_row(&(Vec4::new(0.0, 0.0, 5.0, 8.0)), 3);

  let submatrix = matrix1.get_submatrix(1, 2);

  let cofactor = Mat4::get_cofactor(&submatrix, 1, 0);

  assert_eq!(cofactor, -72.0);
}

#[test]
fn mat4_sub_determinant_test() {
  let mut matrix1 = Mat4::new();

  matrix1.set_row(&(Vec4::new(0.0, 9.0, 3.0, 0.0)), 0);
  matrix1.set_row(&(Vec4::new(9.0, 8.0, 0.0, 8.0)), 1);
  matrix1.set_row(&(Vec4::new(1.0, 8.0, 5.0, 3.0)), 2);
  matrix1.set_row(&(Vec4::new(0.0, 0.0, 5.0, 8.0)), 3);

  let submatrix = matrix1.get_submatrix(1, 2);

  let sub_determinant = Mat4::get_sub_determinant(&submatrix);

  assert_eq!(sub_determinant, -72.0);
}

#[test]
fn mat4_determinate_test() {
  let mut matrix1 = Mat4::new();

  matrix1.set_row(&(Vec4::new(-2.0, -8.0, 3.0, 5.0)), 0);
  matrix1.set_row(&(Vec4::new(-3.0, 1.0, 7.0, 3.0)), 1);
  matrix1.set_row(&(Vec4::new(1.0, 2.0, -9.0, 6.0)), 2);
  matrix1.set_row(&(Vec4::new(-6.0, 7.0, 7.0, -9.0)), 3);

  let determinant = matrix1.determinant();

  assert_eq!(determinant, -4071.0);
}

pub fn round_to_decimal(value: f32, places: f32) -> f32 {
  let round_factor = 10.0f32.powf(places);
  let return_value = (value * round_factor).round() / round_factor;
  return return_value;
}

#[test]
fn mat4_inverted_test() {
  let mut matrix1 = Mat4::new();

  matrix1.set_row(&(Vec4::new(-5.0, 2.0, 6.0, -8.0)), 0);
  matrix1.set_row(&(Vec4::new(1.0, -5.0, 1.0, 8.0)), 1);
  matrix1.set_row(&(Vec4::new(7.0, 7.0, -6.0, -7.0)), 2);
  matrix1.set_row(&(Vec4::new(1.0, -3.0, 7.0, 4.0)), 3);

  let matrix2 = Mat4::inverse(&matrix1);

  assert_eq!(round_to_decimal(matrix2.values[0][0], 5.0), 0.21805);
  assert_eq!(round_to_decimal(matrix2.values[0][1], 5.0), 0.45113);
  assert_eq!(round_to_decimal(matrix2.values[0][2], 5.0), 0.24060);
  assert_eq!(round_to_decimal(matrix2.values[0][3], 5.0), -0.04511);
  assert_eq!(round_to_decimal(matrix2.values[1][0], 5.0), -0.80827);
  assert_eq!(round_to_decimal(matrix2.values[1][1], 5.0), -1.45677);
  assert_eq!(round_to_decimal(matrix2.values[1][2], 5.0), -0.44361);
  assert_eq!(round_to_decimal(matrix2.values[1][3], 5.0), 0.52068);
  assert_eq!(round_to_decimal(matrix2.values[2][0], 5.0), -0.07895);
  assert_eq!(round_to_decimal(matrix2.values[2][1], 5.0), -0.22368);
  assert_eq!(round_to_decimal(matrix2.values[2][2], 5.0), -0.05263);
  assert_eq!(round_to_decimal(matrix2.values[2][3], 5.0), 0.19737);
  assert_eq!(round_to_decimal(matrix2.values[3][0], 5.0), -0.52256);
  assert_eq!(round_to_decimal(matrix2.values[3][1], 5.0), -0.81391);
  assert_eq!(round_to_decimal(matrix2.values[3][2], 5.0), -0.30075);
  assert_eq!(round_to_decimal(matrix2.values[3][3], 5.0), 0.30639);
}

#[test]
fn mat4_product_inverted_test() {
  let mut matrix1 = Mat4::new();

  matrix1.set_row(&(Vec4::new(3.0, -9.0, 7.0, 3.0)), 0);
  matrix1.set_row(&(Vec4::new(3.0, -8.0, 2.0, -9.0)), 1);
  matrix1.set_row(&(Vec4::new(-4.0, 4.0, 4.0, 1.0)), 2);
  matrix1.set_row(&(Vec4::new(-6.0, 5.0, -1.0, 1.0)), 3);

  let mut matrix2 = Mat4::new();

  matrix2.set_row(&(Vec4::new(8.0, 2.0, 2.0, 2.0)), 0);
  matrix2.set_row(&(Vec4::new(3.0, -1.0, 7.0, 0.0)), 1);
  matrix2.set_row(&(Vec4::new(7.0, 0.0, 5.0, 4.0)), 2);
  matrix2.set_row(&(Vec4::new(6.0, -2.0, 0.0, 5.0)), 3);

  let matrix3 = &matrix1 * &matrix2;

  let matrix4 = Mat4::inverse(&matrix2);

  let mut matrix5 = &matrix3 * &matrix4;

  for y in 0..4 {
    for x in 0..4 {
      matrix5.values[x][y] = round_to_decimal(matrix5.values[x][y], 1.0);
    }
  }

  assert_eq!(matrix1.values, matrix5.values);
}