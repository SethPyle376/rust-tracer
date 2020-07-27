use crate::rust_tracer::math::mat4::Mat4;

#[test]
fn mat4_row_test() {
  let mut mat4 = Mat4::new();
  let vector = vec![0.0, 1.0, 2.0, 3.0];
  mat4.set_row(vector, 0);

  assert_eq!(mat4.get_value(0, 0), 0.0);
  assert_eq!(mat4.get_value(0, 1), 1.0);
  assert_eq!(mat4.get_value(0, 2), 2.0);
  assert_eq!(mat4.get_value(0, 3), 3.0);
}

#[test]
fn mat4_column_test() {
  let mut mat4 = Mat4::new();
  let vector = vec![0.0, 1.0, 2.0, 3.0];
  mat4.set_column(vector, 0);

  assert_eq!(mat4.get_value(0, 0), 0.0);
  assert_eq!(mat4.get_value(1, 0), 1.0);
  assert_eq!(mat4.get_value(2, 0), 2.0);
  assert_eq!(mat4.get_value(3, 0), 3.0);
}
