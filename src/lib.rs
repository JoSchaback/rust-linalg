/// Three dimensional vector of `f32`'s. The three dimensions are
/// accessible through the fields `x`, `y` and `z`.
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/// Four dimensional vector of `f32`'s. The four dimensions are
/// accessible through the fields `x`, `y`, `z` and `w`.
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

/// A 3x3 matrix.
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

pub mod vector3;
pub mod vector4;
pub mod matrix3;
pub mod matrix4;