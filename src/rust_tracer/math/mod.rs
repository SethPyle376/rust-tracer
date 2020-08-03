use nalgebra::{Vector4, Matrix, ArrayStorage, U4, Matrix4};

const EPSILON: f32 = 0.00001;

pub(crate) type Vec4 = Vector4<f32>;
pub(crate) type Mat4 = Matrix4<f32>;

pub mod point;
pub mod color;
pub mod ray;
pub mod sphere;