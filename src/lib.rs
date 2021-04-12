//! `js_linalg` is probably the smallest linear algebra library on the planet. It comes without dependencies,
//! is very fast, uses no number abstraction traits (all `f32`) and tailored for usage with OpenGL/WebGL. 
//! It implements four structs; `Vector3`, `Vector4`, `Matrix3` and `Matrix4`. This is all you need for a happy and fulfilled life.
//! 
//! Please find more info in the README.md.

/// Three dimensional vector of `f32`'s. 
/// The three dimensions are accessible through the fields `x`, `y` and `z`.
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

mod vector3;

/// Four dimensional vector of `f32`'s. The four dimensions are
/// accessible through the fields `x`, `y`, `z` and `w`.
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    /// The first dimension of `Vector4`.
    pub x: f32,

    /// The second dimension of `Vector4`.
    pub y: f32,

    /// The third dimension of `Vector4`.
    pub z: f32,

    /// The fourth dimension of `Vector4`.
    pub w: f32
}

pub mod vector4;

/// A 3x3 matrix of `f32`'s.
#[derive(Debug, Clone)]
pub struct Matrix3 {
    pub m_0_0 : f32,
    pub m_0_1 : f32,
    pub m_0_2 : f32,

    pub m_1_0 : f32,
    pub m_1_1 : f32,
    pub m_1_2 : f32,

    pub m_2_0 : f32,
    pub m_2_1 : f32,
    pub m_2_2 : f32,
}

mod matrix3;


/// A 4x4 matrix of `f32`'s.
#[derive(Debug, Clone)]
pub struct Matrix4 {
    pub m_0_0 : f32,
    pub m_0_1 : f32,
    pub m_0_2 : f32,
    pub m_0_3 : f32,

    pub m_1_0 : f32,
    pub m_1_1 : f32,
    pub m_1_2 : f32,
    pub m_1_3 : f32,

    pub m_2_0 : f32,
    pub m_2_1 : f32,
    pub m_2_2 : f32,
    pub m_2_3 : f32,

    pub m_3_0 : f32,
    pub m_3_1 : f32,
    pub m_3_2 : f32,
    pub m_3_3 : f32
}

mod matrix4;