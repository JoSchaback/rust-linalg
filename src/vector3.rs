use super::Vector3;

impl Vector3 {

    /// Creates a new Vector3 with all dimensions set to zero.
    pub fn new() -> Vector3 {
        Vector3 {x: 0.0, y: 0.0, z: 0.0}
    }

    /// Creates a new Vector3 from a given Vector3 by copying 
    /// the dimension values.
    pub fn from_vector3(vec: &Vector3) -> Vector3 {
        Vector3 {x:vec.x, y:vec.y, z:vec.z}
    }

    /// Creates a new Vector3 with components `(0, 0, 1)`.
    pub fn new_z_up() -> Vector3 {
        Vector3 {x: 0., y: 0., z: 1.}
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


#[test]
fn zero_defined_correctly() {
    let a = Vector3::new();
    assert_eq!(a.x, 0.0);
    assert_eq!(a.y, 0.0);
    assert_eq!(a.z, 0.0);
}

#[test]
fn z_up_defined_correctly() {
    let a = Vector3::new_z_up();
    assert_eq!(a.x, 0.0);
    assert_eq!(a.y, 0.0);
    assert_eq!(a.z, 1.0);
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
    let k = Vector3::from_vector3(&m);
    assert_eq!(k.x, 1.0);
    assert_eq!(k.y, 2.0);
    assert_eq!(k.z, 3.0);
}

use super::Matrix4;

impl std::ops::MulAssign<&Matrix4> for Vector3 {
    /// Overrides '*=' operator to multiply a `Matrix4` with a `Vector3` in-place. The
    /// fourth dimension is set to 1.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector3};
    /// let     a = Matrix4::new();
    /// let mut b = Vector3::new();
    /// b *= &a;
    /// ```
    fn mul_assign(&mut self, m: &Matrix4) {
        m.mult_to_vector3(self, 1.0);
    }
}

impl std::ops::MulAssign<Matrix4> for Vector3 {
    /// Overrides '*=' operator to multiply a `Matrix4` with a `Vector3` in-place. The
    /// fourth dimension is set to 1.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector3};
    /// let     a = Matrix4::new();
    /// let mut b = Vector3::new();
    /// b *= a;
    /// ```
    fn mul_assign(&mut self, m: Matrix4) {
        m.mult_to_vector3(self, 1.0);
    }
}