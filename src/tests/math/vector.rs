use crate::rust_tracer::math::vector::Vec4;

#[test]
fn new_vector_test() {
    let vector = Vec4::new(1.2, 3.2, 4.2);
    assert_eq!(vector.x, 1.2);
    assert_eq!(vector.y, 3.2);
    assert_eq!(vector.z, 4.2);
    assert_eq!(vector.w, 0.0);
}

#[test]
fn vector_equal_test() {
    let vector1 = Vec4::new(1.2, 3.2, 4.2);
    let vector2 = Vec4::new(1.2, 3.2, 4.2);
    assert!(vector1.equal(&vector2));
}

#[test]
fn vector_near_equal_test() {
    let vector1 = Vec4::new(1.200001, 3.200001, 4.200001);
    let vector2 = Vec4::new(1.2, 3.2, 4.2);
    assert!(vector1.equal(&vector2));
}

#[test]
fn vector_not_equal_test() {
    let vector1 = Vec4::new(1.3, 3.2, 4.2);
    let vector2 = Vec4::new(1.2, 3.2, 4.2);
    assert!(!vector1.equal(&vector2));
}