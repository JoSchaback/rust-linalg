use super::Vector4;

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

    pub fn from_i32(x:i32, y:i32, z:i32, w:i32) -> Vector4 {
        Vector4 {x: x as f32, y: y as f32, z: z as f32, w:w as f32}
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

    pub fn scale_mut(&mut self, s:f32) {
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
