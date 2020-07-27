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
