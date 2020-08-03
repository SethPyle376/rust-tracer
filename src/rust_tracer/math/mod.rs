use nalgebra::{Vector4, Matrix, ArrayStorage, U4};

const EPSILON: f32 = 0.00001;

pub(crate) type Vec4 = Vector4<f32>;
pub(crate) type Mat4 = Matrix<f32, U4, U4, ArrayStorage<f32, U4, U4>>;

pub mod point;
pub mod color;
pub mod ray;
pub mod sphere;