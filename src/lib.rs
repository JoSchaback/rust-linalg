//! `js_linalg` is probably the smallest linear algebra library on the planet. It comes without dependencies,
//! is very fast, uses no number abstraction traits (all `f32`) and tailored for usage with OpenGL/WebGL. 
//! It implements four simple
//! structs, this is all you need for a happy and fulfilled life:
//! * `Vector3` - 
//! 
//! # Design Principles
//! The mere purpose of `js_linalg` is to make the life of a computer graphics programmer easier.
//! But not necessarily the life of a mathematician. That is why we keep things simple towards the the use-cases of
//! setting up "view", "model-view", "projection" or "normal" matrices.
//! The library intentionally drops the concepts of transpositions or generalizations such as for example 
//! regarding 3-dimensional vectors as 1x3 matrices. Instead, the library has the concept of explicit 
//! 3D and 4D vectors ("columns matrices" for the mathematically inclined reader) and
//! hard-wired 3x3 and 4x4 matrices. This means you may apply 3x3 matrices onto any 3D vector as well as 4x4 matrices onto any
//! 4D vector.
//!  
//! As a user of this library you need to keep track of transpositions yourselve. 
//! There are other libraries that cover transformations and 
//! 
//! `js_linalg` contains vector and matrix implementations of different
//! dimensions, but all with 
//! - explicit fields (no arrays) which makes it easier for look-ahead optimizations to take place,
//! - matrix component order that is aligned with OpenGL for simple as fast uploading, and
//! - all fields are `f32`'s as this is sufficient precision while small enough for most *GL applications.
//! 
//! # Main structs
//! `js_linalg` consists of the following central structs that are instrumental.
//! - `Vector3`: A three dimensional vector. Commonly used. Bascially stuff that you remember from school.
//! - `Vector4`: A four dimensional vector, usually used rarley except for exotic multiplications with 4x4 matrices.
//! - `Matrix3`

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