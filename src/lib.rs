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

impl Vector3 {

    /// Creates a new Vector3 with all dimensions set to zero.
    pub fn new() -> Vector3 {
        Vector3 {x: 0.0, y: 0.0, z: 0.0}
    }

    /// Creates a new Vector3 from a given Vector3 by copying 
    /// the dimension values.
    pub fn from_vec3(vec: &Vector3) -> Vector3 {
        Vector3 {x:vec.x, y:vec.y, z:vec.z}
    }

    /// Creates a new Vector3 from three `f32`'s. This is a slightly
    /// more convenient way to create Vector3`s than via `Vector3{x: ..., y:...}`.
    pub fn from(x:f32, y:f32, z:f32) -> Vector3 {
        Vector3 {x:x, y:y, z:z}
    }

    /// Creates a new Vector3 from three `i32`'s. 
    pub fn from_i32(x:i32, y:i32, z:i32) -> Vector3 {
        Vector3 {x:x as f32, y:y as f32, z:z as f32}
    }

    /// In-place normalization of this `Vector3`. It divides all dimensions by the length of
    /// this Vector3 such that this vector ends up with a length of 1. 
    /// 
    /// # Examples
    /// ```
    /// use js_linalg::Vector3;
    /// let mut a = Vector3::from_i32(1, 2, 3);
    /// let _b = a.normalize_mut();
    /// // _b is a &mut reference for further use
    /// ```
    pub fn normalize_mut(&mut self) -> &mut Vector3{
        let d = ( self.x*self.x + self.y*self.y + self.z*self.z ) .sqrt();
        self.x /= d;
        self.y /= d;
        self.z /= d;
        self
    }

    pub fn normalize(&self) -> Vector3{
        let d = ( self.x*self.x + self.y*self.y + self.z*self.z ) .sqrt();
        Vector3 {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }

    /// Set the individual dimensions of this Vector3. Note that the `_mut` suffix is dropped
    /// here since the name `set_mut` would sound redundant.
    pub fn set(&mut self, xp:f32, yp:f32, zp:f32) -> &mut Vector3{
        self.x = xp;
        self.y = yp;
        self.z = zp;

        self
    }

    /// Set the individual dimensions of this Vector3 copied
    /// from the provided Vector3. 
    pub fn set_vector3(&mut self, vec:&Vector3) -> &mut Vector3{
        self.x = vec.x;
        self.y = vec.y;
        self.z = vec.z;

        self
    }

    /// In-place subtraction of provided values from each dimension of this `Vector3`.
    pub fn sub_mut(&mut self, xp:f32, yp:f32, zp:f32) -> &mut Vector3 {
        self.x -= xp;
        self.y -= yp;
        self.z -= zp;

        self
    }

    /// Subtracts given parameters from this Vector3 and returns result as new Vector3.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Vector3;
    /// let a = Vector3::from_i32(1, 2, 3);
    /// let c = a.sub(1., 4., 12.); // creates new Vector3
    /// assert_eq!(c.x, 0.);
    /// ```
    pub fn sub(&self, xp:f32, yp:f32, zp:f32) -> Vector3 {
        Vector3 {
            x: self.x-xp,
            y: self.y-yp,
            z: self.z-zp,
        }
    }

    /// In-place subtraction of a provided `Vector3 from this `Vector3`. It writes
    /// the result back to this Vector3 (aka in-place).
    pub fn sub_mut_vector3(&mut self, vec:&Vector3) -> &mut Vector3 {
        self.x -= vec.x;
        self.y -= vec.y;
        self.z -= vec.z;

        self
    }

    /// In-place addition on all three dimensions via three provided values.
    pub fn add_mut(&mut self, xp:f32, yp:f32, zp:f32) -> &mut Vector3 {
        self.x += xp;
        self.y += yp;
        self.z += zp;

        self
    }

    /// In-place cross-product computation.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Vector3;
    /// let mut a = Vector3::from_i32(0, 0, 1);
    /// let     b = Vector3::from_i32(1, 0, 0);
    /// a.cross_mut(&b);
    /// ```
    pub fn cross_mut(&mut self, a:&Vector3) -> &mut Vector3 {
        let temp_x = self.y * a.z - self.z * a.y;
        let temp_y = self.z * a.x - self.x * a.z;
        let temp_z = self.x * a.y - self.y * a.x;

        self.x = temp_x;
        self.y = temp_y;
        self.z = temp_z;

        self
    }

    /// Computes the cross-product between two `Vector3`'s and returns the result as 
    /// new `Vector3`.
    pub fn cross_vector3(s:&Vector3, a:&Vector3) -> Vector3 {
        Vector3 {
            x: s.y * a.z - s.z * a.y,
            y: s.z * a.x - s.x * a.z,
            z: s.x * a.y - s.y * a.x
        }
    }

    /// Computes the cross-product between this vector and a provided one and returns the
    /// result as a new `Vector3`.
    pub fn cross(&self, a:&Vector3) -> Vector3 {
        Vector3 {
            x: self.y * a.z - self.z * a.y,
            y: self.z * a.x - self.x * a.z,
            z: self.x * a.y - self.y * a.x
        }
    }

}

impl std::convert::AsMut<Vector3> for Vector3 {

    fn as_mut(&mut self) -> &mut Vector3 {
        return self;
    }

}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    /// Overrides '+' operator to add one vector to another.
    /// 
    /// # Examples
    /// ```
    /// use js_linalg::Vector3;
    /// let a = Vector3::from_i32(1, 2, 3);
    /// let b = Vector3::from_i32(4, 4, 4);
    /// let c = a + b;
    /// ```
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl std::ops::Add<&Vector3> for Vector3 {
    type Output = Vector3;

    /// Overrides '+' operator to add one vector to another.
    /// 
    /// # Examples
    /// ```
    /// use js_linalg::Vector3;
    /// let a = Vector3::from_i32(1, 2, 3);
    /// let b = &Vector3::from_i32(4, 4, 4);
    /// let c = a + b;
    /// ```
    fn add(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl std::ops::Add<&Vector3> for &Vector3 {
    type Output = Vector3;

    /// Overrides '+' operator to add one vector to another.
    /// 
    /// # Examples
    /// ```
    /// use js_linalg::Vector3;
    /// let a = &Vector3::from_i32(1, 2, 3);
    /// let b = &Vector3::from_i32(4, 4, 4);
    /// let c = a + b;
    /// ```
    fn add(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl std::ops::Add<Vector3> for &Vector3 {
    type Output = Vector3;

    /// Overrides '+' operator to add one vector to another.
    /// 
    /// # Examples
    /// ```
    /// use js_linalg::Vector3;
    /// let a = &Vector3::from_i32(1, 2, 3);
    /// let b = Vector3::from_i32(4, 4, 4);
    /// let c = a + b;
    /// assert_eq!(c.x, 1. + 4.);
    /// assert_eq!(c.y, 2. + 4.);
    /// assert_eq!(c.z, 3. + 4.);
    /// ```
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

#[test]
fn check_vector3_add_completeness() {
    let a = Vector3::from(0., 2., 4.);
    let b = Vector3::from(1., 3., 2.);

    let _c  = a + b;
    let _c  = &a + b;
    let _c  = a + &b;
    let _c  = &a + &b;
}

impl std::ops::AddAssign<Vector3> for &mut Vector3 {

    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::AddAssign<&Vector3> for &mut Vector3 {
    /// Implements '+=' operator for Vector3
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Vector3;
    /// let mut a = Vector3::from_i32(1, 2, 3);
    /// let mut a = a.as_mut();
    /// let     b = Vector3::from_i32(1, 2, 3);
    /// a += b;
    /// assert_eq!(a.x, 2.);
    /// assert_eq!(a.y, 4.);
    /// assert_eq!(a.z, 6.);
    /// ```    
    
    fn add_assign(&mut self, rhs: &Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::AddAssign<&Vector3> for Vector3 {
    /// Implements '+=' operator for Vector3
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Vector3;
    /// let mut a =  Vector3::from_i32(1, 2, 3);
    /// let     b = &Vector3::from_i32(1, 2, 3);
    /// a += b;
    /// assert_eq!(a.x, 2.);
    /// assert_eq!(a.y, 4.);
    /// assert_eq!(a.z, 6.);
    /// ```      
    /// Or as `&mut Vector3`:
    /// ```
    /// use js_linalg::Vector3;
    /// let mut a = Vector3::from_i32(1, 2, 3);
    /// let     b = Vector3::from_i32(1, 2, 3);
    /// let mut a = a.as_mut();
    /// a += b;
    /// a += &b;
    /// ```
    
    fn add_assign(&mut self, rhs: &Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::AddAssign<Vector3> for Vector3 {
    /// Implements '+=' operator for Vector3
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Vector3;
    /// let mut a = Vector3::from_i32(1, 2, 3);
    /// let     b = Vector3::from_i32(1, 2, 3);
    /// a += b;
    /// assert_eq!(a.x, 2.);
    /// assert_eq!(a.y, 4.);
    /// assert_eq!(a.z, 6.);
    /// ```    
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl std::ops::Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl std::ops::Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl std::ops::Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

#[test]
fn check_vector3_sub_completeness() {
    let a = Vector3::from(0., 2., 4.);
    let b = Vector3::from(1., 3., 2.);

    let _c  = a - b;
    let _c  = &a - b;
    let _c  = a - &b;
    let _c  = &a - &b;
}

impl std::ops::SubAssign<&Vector3> for Vector3 {
    /// Implements '-=' operator for Vector3
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Vector3;
    /// let mut a = Vector3::from_i32(1, 2, 3);
    /// let     b = Vector3::from_i32(1, 1, 1);
    /// a -= &b;
    /// assert_eq!(a.x, 0.);
    /// assert_eq!(a.y, 1.);
    /// assert_eq!(a.z, 2.);
    /// ```    
    
    fn sub_assign(&mut self, rhs: &Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::SubAssign<Vector3> for Vector3 {
    /// Implements '-=' operator for Vector3
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Vector3;
    /// let mut a = Vector3::from_i32(1, 2, 3);
    /// let     b = Vector3::from_i32(1, 1, 1);
    /// a -= b;
    /// assert_eq!(a.x, 0.);
    /// assert_eq!(a.y, 1.);
    /// assert_eq!(a.z, 2.);
    /// ```    
    
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.x * rhs,
            z: self.x * rhs
        }
    }
}

impl std::ops::Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.x * rhs,
            z: self.x * rhs
        }
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}


impl std::ops::Mul<&Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl std::ops::MulAssign<f32> for Vector3 {

    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.x *= rhs;
        self.x *= rhs;       
    }
}

#[test]
fn check_vector3_mul_completeness() {
    let a = Vector3::from(0., 2., 4.);
    let b = 1.4;

    let _c  = a * b;
    let _c  = &a * b;

    let _c  = b * a;
    let _c  = b * &a;

}

pub const ZERO : &'static Vector3 = &Vector3 {x : 0.0, y: 0.0, z: 0.0};
pub const Z_UP : &'static Vector3 = &Vector3 {x : 0.0, y: 0.0, z: 1.0};


#[test]
fn zero_defined_correctly() {
    assert_eq!(ZERO.x, 0.0);
    assert_eq!(ZERO.y, 0.0);
    assert_eq!(ZERO.z, 0.0);
}

#[test]
fn z_up_defined_correctly() {
    assert_eq!(Z_UP.x, 0.0);
    assert_eq!(Z_UP.y, 0.0);
    assert_eq!(Z_UP.z, 1.0);
}

#[test]
fn test_new_vector3() {
    let m = Vector3::new();
    assert_eq!(m.x, 0.0);
    assert_eq!(m.y, 0.0);
    assert_eq!(m.z, 0.0);
}

#[test]
fn test_new_vector3_assignment() {
    let m = Vector3{x:1.0, y:2.0, z:3.0};
    let k = Vector3::from_vec3(&m);
    assert_eq!(k.x, 1.0);
    assert_eq!(k.y, 2.0);
    assert_eq!(k.z, 3.0);
}

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

/// A four dimensional vector of `f32`'s.
/// 
/// 
/// The four dimensions are accessible via the fields `x`, `y`, `z` and `w`.
impl Vector4 {

    /// Create a new `Vector4` struct with all dimensions set to zero.
    /// 
    /// # Examples
    /// ```
    /// use js_linalg::Vector4;
    /// let vec = Vector4::new();
    /// assert_eq!(vec.x, 0.0);
    /// assert_eq!(vec.y, 0.0);
    /// assert_eq!(vec.z, 0.0);
    /// assert_eq!(vec.w, 0.0);
    /// ```
    pub fn new() -> Vector4 {
        Vector4 {x: 0.0, y: 0.0, z: 0.0, w:0.0}
    }

    /// Create a new `Vector4` struct where the dimension vales are copied from 
    /// the provided `vec` function parameter.
    /// 
    /// # Arguments
    /// * `vec` - A `Vector4` reference that is used to set the dimensions of the to-be-created `Vector4`.
    /// 
    /// # Examples
    /// ```
    /// use js_linalg::Vector4;
    /// // A wild Vector4 appears. It contains some non-standard values.
    /// let some_vec = Vector4{ 
    ///     x: 0.1234,
    ///     y: 321.23,
    ///     z: 1113.001,
    ///     w: 0.12
    /// };
    /// // The values from some_vec are copied-over into the newly created Vector4.
    /// let vec = Vector4::from_vector4(&some_vec);
    /// assert_eq!(vec.x, some_vec.x);
    /// assert_eq!(vec.y, some_vec.y);
    /// assert_eq!(vec.z, some_vec.z);
    /// assert_eq!(vec.w, some_vec.w);
    /// ```
    pub fn from_vector4(vec: &Vector4) -> Vector4 {
        Vector4 {x: vec.x, y: vec.y, z: vec.z, w:vec.w}
    }

    /// Scales the length of this vector to 1 without changing its direction.
    pub fn normalize_mut(&mut self) {

        let d = ( self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w) .sqrt();

        self.x /= d;
        self.y /= d;
        self.z /= d;
        self.w /= d;
    }

    pub fn set(&mut self, xp:f32, yp:f32, zp:f32, wp:f32) {
        self.x = xp;
        self.y = yp;
        self.z = zp;
        self.w = wp;
    }

    pub fn scale_mut_scalar(&mut self, s:f32) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
        self.w *= s;
    }

    pub fn sub_mut(&mut self, xp:f32, yp:f32, zp:f32, wp:f32) {
        self.x -= xp;
        self.y -= yp;
        self.z -= zp;
        self.w -= wp;
    }

    pub fn sub_mut_vector4(&mut self, vec:&Vector4) {
        self.x -= vec.x;
        self.y -= vec.y;
        self.z -= vec.z;
        self.w -= vec.w;
    }

    pub fn add_mut(&mut self, xp:f32, yp:f32, zp:f32, wp:f32) {
        self.x += xp;
        self.y += yp;
        self.z += zp;
        self.w += wp;
    }

}

#[test]
fn test_new_vector4() {
    let m = Vector4::new();
    assert_eq!(m.x, 0.0);
    assert_eq!(m.y, 0.0);
    assert_eq!(m.z, 0.0);
    assert_eq!(m.w, 0.0);
}

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

impl Matrix3 {

    pub fn new() -> Matrix3 {
        Matrix3 {
            m_0_0: 1.0,
            m_1_0: 0.0,
            m_2_0: 0.0,

            m_0_1: 0.0,
            m_1_1: 1.0,
            m_2_1: 0.0,

            m_0_2: 0.0,
            m_1_2: 0.0,
            m_2_2: 1.0,
        }
    }

    pub fn calc_mut_normal_matrix(&mut self, view: &Matrix4) {
        self.m_0_0 = view.m_0_0;
        self.m_1_0 = view.m_1_0;
        self.m_2_0 = view.m_2_0;

        self.m_0_1 = view.m_0_1;
        self.m_1_1 = view.m_1_1;
        self.m_2_1 = view.m_2_1;

        self.m_0_2 = view.m_0_2;
        self.m_1_2 = view.m_1_2;
        self.m_2_2 = view.m_2_2;

        self.inverse_mut();
        self.transpose_mut();
    }

    pub fn set_with_matrix3(&mut self, m:&Matrix3) {
        self.m_0_0 = m.m_0_0;
        self.m_0_1 = m.m_0_1;
        self.m_0_2 = m.m_0_2;
        self.m_1_0 = m.m_1_0;
        self.m_1_1 = m.m_1_1;
        self.m_1_2 = m.m_1_2;
        self.m_2_0 = m.m_2_0;
        self.m_2_1 = m.m_2_1;
        self.m_2_2 = m.m_2_2;
    }

    pub fn inverse_mut(&mut self) {
        let a = self.m_0_0;
        let b = self.m_1_0;
        let c = self.m_2_0;
        let d = self.m_0_1;
        let e = self.m_1_1;
        let f = self.m_2_1;
        let g = self.m_0_2;
        let h = self.m_1_2;
        let i = self.m_2_2;

        let det = a * (e * i - f * h) - b * (i * d - f * g) + c * (d * h - e * g);

        self.m_0_0 = (e * i - f * h) / det;    // A
        self.m_1_0 = -(b * i - c * h) / det;    // D
        self.m_2_0 = (b * f - c * e) / det;    // G
        self.m_0_1 = -(d * i - f * g) / det;    // B
        self.m_1_1 = (a * i - c * g) / det;    // E
        self.m_2_1 = -(a * f - c * d) / det;    // H
        self.m_0_2 = (d * h - e * g) / det;    // C
        self.m_1_2 = -(a * h - b * g) / det;    // F
        self.m_2_2 = (a * e - b * d) / det;    // I
    }

    pub fn transpose_mut(&mut self) {
        let mut tmp;

        tmp        = self.m_0_1;
        self.m_0_1 = self.m_1_0;
        self.m_1_0 = tmp;

        tmp        = self.m_0_2;
        self.m_0_2 = self.m_2_0;
        self.m_2_0 = tmp;

        tmp        = self.m_1_2;
        self.m_1_2 = self.m_2_1;
        self.m_2_1 = tmp;
    }

    pub fn as_ptr(&self) -> *const f32 {
        &self.m_0_0
    }
}

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

#[allow(dead_code)]
impl Matrix4 {

    pub fn as_ptr(&self) -> *const f32 {
        &self.m_0_0
    }

    pub fn rotation(&mut self, alpha: f32, u : &Vector3) {

        self.identity(); // TODO necessary? I think all values get overriden anyways later

        let c = f32::cos(alpha);
        let s = f32::sin(alpha);
        let t = 1.0 - c;

        self.m_0_0 = t * u.x * u.x + c;
        self.m_1_0 = t * u.x * u.y - u.z * s;
        self.m_2_0 = u.x * u.z * t + u.y * s;
        self.m_3_0 = 0.0;
        self.m_0_1 = t * u.y * u.x + u.z * s;
        self.m_1_1 = t * u.y * u.y + c;
        self.m_2_1 = u.y * u.z * t - u.x * s;
        self.m_3_1 = 0.0;
        self.m_0_2 = t * u.z * u.x - u.y * s;
        self.m_1_2 = t * u.z * u.y + u.x * s;
        self.m_2_2 = u.z * u.z * t + c;
        self.m_3_2 = 0.0;
        self.m_0_3 = 0.0;
    }

    pub fn new() -> Matrix4 {
        Matrix4 {
            m_0_0: 1.0,
            m_1_0: 0.0,
            m_2_0: 0.0,
            m_3_0: 0.0,

            m_0_1: 0.0,
            m_1_1: 1.0,
            m_2_1: 0.0,
            m_3_1: 0.0,

            m_0_2: 0.0,
            m_1_2: 0.0,
            m_2_2: 1.0,
            m_3_2: 0.0,

            m_0_3: 0.0,
            m_1_3: 0.0,
            m_2_3: 0.0,
            m_3_3: 1.0,
        }
    }
/*
    #[test]
    fn test_new_matrix4() {
        let m = Matrix4::new();
        assert!(m.m_0_0 == 1.0);
        assert!(m.m_1_1 == 1.0);
        assert!(m.m_2_2 == 1.0);
    } */

    pub fn identity(&mut self) {
        self.m_0_0 = 1.0;
        self.m_1_0 = 0.0;
        self.m_2_0 = 0.0;
        self.m_3_0 = 0.0;

        self.m_0_1 = 0.0;
        self.m_1_1 = 1.0;
        self.m_2_1 = 0.0;
        self.m_3_1 = 0.0;

        self.m_0_2 = 0.0;
        self.m_1_2 = 0.0;
        self.m_2_2 = 1.0;
        self.m_3_2 = 0.0;

        self.m_0_3 = 0.0;
        self.m_1_3 = 0.0;
        self.m_2_3 = 0.0;
        self.m_3_3 = 1.0;
    }

    pub fn frustum(&mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
        // http://www.songho.ca/opengl/gl_projectionmatrix.html

        self.identity();

        self.m_0_0 = (2.0 * near) / (right - left);
        self.m_2_0 = (right + left) / (right - left);

        self.m_1_1 = (2.0 * near) / (top - bottom);
        self.m_2_1 = (top + bottom) / (top - bottom);

        self.m_2_2 = -(far + near) / (far - near);
        self.m_3_2 = -2.0 * (far * near) / (far - near);

        self.m_2_3 = -1.0;
        self.m_3_3 = 0.0;
    }

    pub fn copy(&mut self, c:&Matrix4) {
        self.m_0_0 = c.m_0_0;
        self.m_1_0 = c.m_1_0;
        self.m_2_0 = c.m_2_0;
        self.m_3_0 = c.m_3_0;

        self.m_0_1 = c.m_0_1;
        self.m_1_1 = c.m_1_1;
        self.m_2_1 = c.m_2_1;
        self.m_3_1 = c.m_3_1;

        self.m_0_2 = c.m_0_2;
        self.m_1_2 = c.m_1_2;
        self.m_2_2 = c.m_2_2;
        self.m_3_2 = c.m_3_2;

        self.m_0_3 = c.m_0_3;
        self.m_1_3 = c.m_1_3;
        self.m_2_3 = c.m_2_3;
        self.m_3_3 = c.m_3_3;
    }

    pub fn row(&mut self, row: u8, x: f32, y: f32, z: f32, a: f32) {
        match row {
            0 => {
                self.m_0_0 = x;
                self.m_1_0 = y;
                self.m_2_0 = z;
                self.m_3_0 = a;
            },
            1 => {
                self.m_0_1 = x;
                self.m_1_1 = y;
                self.m_2_1 = z;
                self.m_3_1 = a;
            },
            2 => {
                self.m_0_2 = x;
                self.m_1_2 = y;
                self.m_2_2 = z;
                self.m_3_2 = a;
            },
            3 => {
                self.m_0_3 = x;
                self.m_1_3 = y;
                self.m_2_3 = z;
                self.m_3_3 = a;
            },
            _ => panic!("Matrix4x4 has rows 0 to 3, not {}", row),
        };
    }

    pub fn projection(&mut self, view_angle: f32, width: f32, height: f32, near_clipping_plane: f32, far_clipping_plane: f32) {
        // http://www.geeks3d.com/20090729/howto-perspective-projection-matrix-in-opengl/
        use std::f32::consts::PI;
        let radians: f32 = view_angle * PI / 180.0;
        let half_height = f32::tan(radians / 2.0) * near_clipping_plane;
        let half_scaled_aspect_ratio = half_height * (width / height);
        self.frustum(-half_scaled_aspect_ratio, half_scaled_aspect_ratio, -half_height, half_height, near_clipping_plane, far_clipping_plane);
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.m_0_0 = x;
        self.m_1_0 = 0.0;
        self.m_2_0 = 0.0;
        self.m_3_0 = 0.0;

        self.m_0_1 = 0.0;
        self.m_1_1 = y;
        self.m_2_1 = 0.0;
        self.m_3_1 = 0.0;

        self.m_0_2 = 0.0;
        self.m_1_2 = 0.0;
        self.m_2_2 = z;
        self.m_3_2 = 0.0;

        self.m_0_3 = 0.0;
        self.m_1_3 = 0.0;
        self.m_2_3 = 0.0;
        self.m_3_3 = 1.0;
    }
/*
    pub fn clone(&self) -> Matrix4 {
        Matrix4{
            m_0_0 : self.m_0_0,
            m_0_1 : self.m_0_1,
            m_0_2 : self.m_0_2,
            m_0_3 : self.m_0_3,
            m_1_0 : self.m_1_0,
            m_1_1 : self.m_1_1,
            m_1_2 : self.m_1_2,
            m_1_3 : self.m_1_3,
            m_2_0 : self.m_2_0,
            m_2_1 : self.m_2_1,
            m_2_2 : self.m_2_2,
            m_2_3 : self.m_2_3,
            m_3_0 : self.m_3_0,
            m_3_1 : self.m_3_1,
            m_3_2 : self.m_3_2,
            m_3_3 : self.m_3_3}
    }        
*/
    pub fn set_with_matrix4(&mut self, m:&Matrix4) {
        self.m_0_0 = m.m_0_0;
        self.m_0_1 = m.m_0_1;
        self.m_0_2 = m.m_0_2;
        self.m_0_3 = m.m_0_3;
        self.m_1_0 = m.m_1_0;
        self.m_1_1 = m.m_1_1;
        self.m_1_2 = m.m_1_2;
        self.m_1_3 = m.m_1_3;
        self.m_2_0 = m.m_2_0;
        self.m_2_1 = m.m_2_1;
        self.m_2_2 = m.m_2_2;
        self.m_2_3 = m.m_2_3;
        self.m_3_0 = m.m_3_0;
        self.m_3_1 = m.m_3_1;
        self.m_3_2 = m.m_3_2;
        self.m_3_3 = m.m_3_3;
    }

    pub fn translation(&mut self, x: f32, y: f32, z: f32) {
        self.identity();

        // fourth column
        self.m_3_0 = x;
        self.m_3_1 = y;
        self.m_3_2 = z;
        self.m_3_3 = 1.0;
    }

    /// Computes a classical view matrix by providing a vantage point (`eye`), point where to look at (`center`)
    /// and an up vector (`up`).
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector3};
    /// let mut view_matrix = Matrix4::new();
    /// let     eye         = Vector3::from_i32(4, 3, 5);
    /// let     center      = Vector3::from_i32(0, 0, 0);
    /// let     up          = Vector3::from_i32(0, 0, 1);
    /// view_matrix.look_at(&eye, &center, &up);
    /// ```
    pub fn look_at(&mut self, eye: &Vector3, center: &Vector3, up: &Vector3) -> &mut Matrix4{
        let mut u = Vector3::new();
        let mut v = Vector3::new();
        let mut w = Vector3::from_vec3(eye);

        // the w vector is computed by w = eye - center which means
        // it is the inverse of the viewing direction.
        w.set_vector3(eye).sub_mut_vector3(center).normalize_mut();

        // compute cross product
        u.set_vector3(&up).cross_mut(&w).normalize_mut();
        // side = (0,0,1) x w

        // up = side x look
        v.set_vector3(&w).cross_mut(&u).normalize_mut();
        ////v.set(w).cross(u).normalize();

        self.identity();

        self.row(0, u.x, u.y, u.z, 0.0);
        self.row(1, v.x, v.y, v.z, 0.0);
        self.row(2, w.x, w.y, w.z, 0.0);

        let mut trans = Matrix4::new();
        trans.m_3_0 = -eye.x;
        trans.m_3_1 = -eye.y;
        trans.m_3_2 = -eye.z;

        self.mult(&trans);

        self
    }

    pub fn mult(&mut self, that: &Matrix4) {
        let m00 = self.m_0_0 * that.m_0_0 + self.m_1_0 * that.m_0_1 + self.m_2_0 * that.m_0_2 + self.m_3_0 * that.m_0_3;
        let m01 = self.m_0_1 * that.m_0_0 + self.m_1_1 * that.m_0_1 + self.m_2_1 * that.m_0_2 + self.m_3_1 * that.m_0_3;
        let m02 = self.m_0_2 * that.m_0_0 + self.m_1_2 * that.m_0_1 + self.m_2_2 * that.m_0_2 + self.m_3_2 * that.m_0_3;
        let m03 = self.m_0_3 * that.m_0_0 + self.m_1_3 * that.m_0_1 + self.m_2_3 * that.m_0_2 + self.m_3_3 * that.m_0_3;

        let m10 = self.m_0_0 * that.m_1_0 + self.m_1_0 * that.m_1_1 + self.m_2_0 * that.m_1_2 + self.m_3_0 * that.m_1_3;
        let m11 = self.m_0_1 * that.m_1_0 + self.m_1_1 * that.m_1_1 + self.m_2_1 * that.m_1_2 + self.m_3_1 * that.m_1_3;
        let m12 = self.m_0_2 * that.m_1_0 + self.m_1_2 * that.m_1_1 + self.m_2_2 * that.m_1_2 + self.m_3_2 * that.m_1_3;
        let m13 = self.m_0_3 * that.m_1_0 + self.m_1_3 * that.m_1_1 + self.m_2_3 * that.m_1_2 + self.m_3_3 * that.m_1_3;

        let m20 = self.m_0_0 * that.m_2_0 + self.m_1_0 * that.m_2_1 + self.m_2_0 * that.m_2_2 + self.m_3_0 * that.m_2_3;
        let m21 = self.m_0_1 * that.m_2_0 + self.m_1_1 * that.m_2_1 + self.m_2_1 * that.m_2_2 + self.m_3_1 * that.m_2_3;
        let m22 = self.m_0_2 * that.m_2_0 + self.m_1_2 * that.m_2_1 + self.m_2_2 * that.m_2_2 + self.m_3_2 * that.m_2_3;
        let m23 = self.m_0_3 * that.m_2_0 + self.m_1_3 * that.m_2_1 + self.m_2_3 * that.m_2_2 + self.m_3_3 * that.m_2_3;

        let m30 = self.m_0_0 * that.m_3_0 + self.m_1_0 * that.m_3_1 + self.m_2_0 * that.m_3_2 + self.m_3_0 * that.m_3_3;
        let m31 = self.m_0_1 * that.m_3_0 + self.m_1_1 * that.m_3_1 + self.m_2_1 * that.m_3_2 + self.m_3_1 * that.m_3_3;
        let m32 = self.m_0_2 * that.m_3_0 + self.m_1_2 * that.m_3_1 + self.m_2_2 * that.m_3_2 + self.m_3_2 * that.m_3_3;
        let m33 = self.m_0_3 * that.m_3_0 + self.m_1_3 * that.m_3_1 + self.m_2_3 * that.m_3_2 + self.m_3_3 * that.m_3_3;

        self.m_0_0 = m00;
        self.m_0_1 = m01;
        self.m_0_2 = m02;
        self.m_0_3 = m03;

        self.m_1_0 = m10;
        self.m_1_1 = m11;
        self.m_1_2 = m12;
        self.m_1_3 = m13;

        self.m_2_0 = m20;
        self.m_2_1 = m21;
        self.m_2_2 = m22;
        self.m_2_3 = m23;

        self.m_3_0 = m30;
        self.m_3_1 = m31;
        self.m_3_2 = m32;
        self.m_3_3 = m33;
    }

    pub fn mult_to_vec4(&self, vec: &mut Vector4) {
        let nx = vec.x * self.m_0_0 + vec.y * self.m_1_0 + vec.z * self.m_2_0 + vec.w * self.m_3_0;
        let ny = vec.x * self.m_0_1 + vec.y * self.m_1_1 + vec.z * self.m_2_1 + vec.w * self.m_3_1;
        let nz = vec.x * self.m_0_2 + vec.y * self.m_1_2 + vec.z * self.m_2_2 + vec.w * self.m_3_2;
        let nw = vec.x * self.m_0_3 + vec.y * self.m_1_3 + vec.z * self.m_2_3 + vec.w * self.m_3_3;

        vec.x = nx;
        vec.y = ny;
        vec.z = nz;
        vec.w = nw;
    }

    pub fn mult_to_vec3(&self, vec: &mut Vector3, w:f32) {
        let nx = vec.x * self.m_0_0 + vec.y * self.m_1_0 + vec.z * self.m_2_0 + w * self.m_3_0;
        let ny = vec.x * self.m_0_1 + vec.y * self.m_1_1 + vec.z * self.m_2_1 + w * self.m_3_1;
        let nz = vec.x * self.m_0_2 + vec.y * self.m_1_2 + vec.z * self.m_2_2 + w * self.m_3_2;

        vec.x = nx;
        vec.y = ny;
        vec.z = nz;
    }
}
