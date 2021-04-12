use super::{Matrix4, Vector3, Vector4};

#[allow(dead_code)]
impl Matrix4 {

    /// Createa a new `Matrix4`struct, initialized as identity matrix.
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

        /// Createa a new `Matrix4`struct, initialized as identity matrix.
        pub fn from_matrix4(m:&Matrix4) -> Matrix4 {
            Matrix4 {
                m_0_0: m.m_0_0,
                m_1_0: m.m_1_0,
                m_2_0: m.m_2_0,
                m_3_0: m.m_3_0,
    
                m_0_1: m.m_0_1,
                m_1_1: m.m_1_1,
                m_2_1: m.m_2_1,
                m_3_1: m.m_3_1,
    
                m_0_2: m.m_0_2,
                m_1_2: m.m_1_2,
                m_2_2: m.m_2_2,
                m_3_2: m.m_3_2,
    
                m_0_3: m.m_0_3,
                m_1_3: m.m_1_3,
                m_2_3: m.m_2_3,
                m_3_3: m.m_3_3,
            }
        }

    pub fn rotation_mut(&mut self, alpha: f32, u : &Vector3) {

        self.set_identity(); // TODO necessary? I think all values get overriden anyways later

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

    pub fn set_identity(&mut self) -> &mut Matrix4 {
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

        self
    }

    pub fn frustum_mut(&mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
        // http://www.songho.ca/opengl/gl_projectionmatrix.html

        self.set_identity();

        self.m_0_0 = (2.0 * near) / (right - left);
        self.m_2_0 = (right + left) / (right - left);

        self.m_1_1 = (2.0 * near) / (top - bottom);
        self.m_2_1 = (top + bottom) / (top - bottom);

        self.m_2_2 = -(far + near) / (far - near);
        self.m_3_2 = -2.0 * (far * near) / (far - near);

        self.m_2_3 = -1.0;
        self.m_3_3 = 0.0;
    }

    /// Sets the components of the specified row.
    /// 
    /// # Arguments
    /// * `row` - the row to set, the first row (on top) is row `0`.
    /// * `x` - the first, left-most component to set
    /// * `y` - the second component of row
    /// * `z` - the third component of row
    /// * `a` - the fourth component of row
    /// 
    /// #Example
    /// ```
    /// use js_linalg::Matrix4;
    /// let mut a = Matrix4::new();
    /// a.set_row(2, 4., 5., 6., 7.);
    /// assert_eq!(a.m_0_2, 4.);
    /// assert_eq!(a.m_1_2, 5.);
    /// assert_eq!(a.m_2_2, 6.);
    /// assert_eq!(a.m_3_2, 7.);
    /// ```
    pub fn set_row(&mut self, row: u32, x: f32, y: f32, z: f32, a: f32) {
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

    pub fn projection_mut(&mut self, view_angle: f32, width: f32, height: f32, near_clipping_plane: f32, far_clipping_plane: f32) {
        // http://www.geeks3d.com/20090729/howto-perspective-projection-matrix-in-opengl/
        use std::f32::consts::PI;
        let radians: f32 = view_angle * PI / 180.0;
        let half_height = f32::tan(radians / 2.0) * near_clipping_plane;
        let half_scaled_aspect_ratio = half_height * (width / height);
        self.frustum_mut(-half_scaled_aspect_ratio, half_scaled_aspect_ratio, -half_height, half_height, near_clipping_plane, far_clipping_plane);
    }

    pub fn scale_mut(&mut self, x: f32, y: f32, z: f32) {
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

    pub fn add_mut_matrix4(&mut self, m:&Matrix4) {
        self.m_0_0 += m.m_0_0;
        self.m_0_1 += m.m_0_1;
        self.m_0_2 += m.m_0_2;
        self.m_0_3 += m.m_0_3;
        self.m_1_0 += m.m_1_0;
        self.m_1_1 += m.m_1_1;
        self.m_1_2 += m.m_1_2;
        self.m_1_3 += m.m_1_3;
        self.m_2_0 += m.m_2_0;
        self.m_2_1 += m.m_2_1;
        self.m_2_2 += m.m_2_2;
        self.m_2_3 += m.m_2_3;
        self.m_3_0 += m.m_3_0;
        self.m_3_1 += m.m_3_1;
        self.m_3_2 += m.m_3_2;
        self.m_3_3 += m.m_3_3;
    }

    pub fn sub_mut_matrix4(&mut self, m:&Matrix4) {
        self.m_0_0 -= m.m_0_0;
        self.m_0_1 -= m.m_0_1;
        self.m_0_2 -= m.m_0_2;
        self.m_0_3 -= m.m_0_3;
        self.m_1_0 -= m.m_1_0;
        self.m_1_1 -= m.m_1_1;
        self.m_1_2 -= m.m_1_2;
        self.m_1_3 -= m.m_1_3;
        self.m_2_0 -= m.m_2_0;
        self.m_2_1 -= m.m_2_1;
        self.m_2_2 -= m.m_2_2;
        self.m_2_3 -= m.m_2_3;
        self.m_3_0 -= m.m_3_0;
        self.m_3_1 -= m.m_3_1;
        self.m_3_2 -= m.m_3_2;
        self.m_3_3 -= m.m_3_3;
    }

    pub fn set_matrix4(&mut self, m:&Matrix4) {
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

    pub fn set_translation(&mut self, x: f32, y: f32, z: f32) -> &mut Matrix4 {
        self.set_identity();

        // fourth column
        self.m_3_0 = x;
        self.m_3_1 = y;
        self.m_3_2 = z;
        self.m_3_3 = 1.0;

        self
    }

    /// Computes a classical view matrix by providing a vantage point (`eye`), point where to look at (`center`)
    /// and an up vector (`up`). Returns a new `Matrix4` struct.
    pub fn look_at(eye: &Vector3, center: &Vector3, up: &Vector3) -> Matrix4 {
        let mut matrix = Matrix4::new();
        matrix.look_at_mut(eye, center, up);
        matrix
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
    /// view_matrix.look_at_mut(&eye, &center, &up);
    /// ```
    pub fn look_at_mut(&mut self, eye: &Vector3, center: &Vector3, up: &Vector3) -> &mut Matrix4{
        let mut u = Vector3::new();
        let mut v = Vector3::new();
        let mut w = Vector3::from_vector3(eye);

        // the w vector is computed by w = eye - center which means
        // it is the inverse of the viewing direction.
        w.set_vector3(eye).sub_mut_vector3(center).normalize_mut();

        // compute cross product
        u.set_vector3(&up).cross_mut(&w).normalize_mut();
        // side = (0,0,1) x w

        // up = side x look
        v.set_vector3(&w).cross_mut(&u).normalize_mut();
        ////v.set(w).cross(u).normalize();

        self.set_identity();

        self.set_row(0, u.x, u.y, u.z, 0.0);
        self.set_row(1, v.x, v.y, v.z, 0.0);
        self.set_row(2, w.x, w.y, w.z, 0.0);

        let mut trans = Matrix4::new();
        trans.m_3_0 = -eye.x;
        trans.m_3_1 = -eye.y;
        trans.m_3_2 = -eye.z;

        self.mult_mut(&trans);

        self
    }

    pub fn mult_mut(&mut self, that: &Matrix4) -> &mut Matrix4 {
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

        self
    }

    pub fn mult_to_vector4(&self, vec: &mut Vector4) {
        let nx = vec.x * self.m_0_0 + vec.y * self.m_1_0 + vec.z * self.m_2_0 + vec.w * self.m_3_0;
        let ny = vec.x * self.m_0_1 + vec.y * self.m_1_1 + vec.z * self.m_2_1 + vec.w * self.m_3_1;
        let nz = vec.x * self.m_0_2 + vec.y * self.m_1_2 + vec.z * self.m_2_2 + vec.w * self.m_3_2;
        let nw = vec.x * self.m_0_3 + vec.y * self.m_1_3 + vec.z * self.m_2_3 + vec.w * self.m_3_3;

        vec.x = nx;
        vec.y = ny;
        vec.z = nz;
        vec.w = nw;
    }

    pub fn mult_to_vector3(&self, vec: &mut Vector3, w:f32) {
        let nx = vec.x * self.m_0_0 + vec.y * self.m_1_0 + vec.z * self.m_2_0 + w * self.m_3_0;
        let ny = vec.x * self.m_0_1 + vec.y * self.m_1_1 + vec.z * self.m_2_1 + w * self.m_3_1;
        let nz = vec.x * self.m_0_2 + vec.y * self.m_1_2 + vec.z * self.m_2_2 + w * self.m_3_2;

        vec.x = nx;
        vec.y = ny;
        vec.z = nz;
    }
}

impl std::ops::Add<Matrix4> for Matrix4 {
    type Output = Matrix4;

    /// Overrides '+' operator to add one `Matrix4` to another component by component.
    /// 
    /// # Examples
    /// ```
    /// use js_linalg::Matrix4;
    /// let a = Matrix4::new();
    /// let b = Matrix4::new();
    /// let c = a + b;
    /// assert_eq!(c.m_0_0, 2.);
    /// ```
    fn add(self, rhs: Matrix4) -> Matrix4 {
        Matrix4 {
            m_0_0: self.m_0_0 + rhs.m_0_0,
            m_0_1: self.m_0_1 + rhs.m_0_1,
            m_0_2: self.m_0_2 + rhs.m_0_1,
            m_0_3: self.m_0_3 + rhs.m_0_3,
            m_1_0: self.m_1_0 + rhs.m_1_0,
            m_1_1: self.m_1_1 + rhs.m_1_1,
            m_1_2: self.m_1_2 + rhs.m_1_2,
            m_1_3: self.m_1_3 + rhs.m_1_3,
            m_2_0: self.m_2_0 + rhs.m_2_0,
            m_2_1: self.m_2_1 + rhs.m_2_1,
            m_2_2: self.m_2_2 + rhs.m_2_2,
            m_2_3: self.m_2_3 + rhs.m_2_3,
            m_3_0: self.m_3_0 + rhs.m_3_0,
            m_3_1: self.m_3_1 + rhs.m_3_1,
            m_3_2: self.m_3_2 + rhs.m_3_2,
            m_3_3: self.m_3_3 + rhs.m_3_3,
        }
    }
}

impl std::ops::Add<Matrix4> for &Matrix4 {
    type Output = Matrix4;

    /// Overrides '+' operator to add one `Matrix4` to another component by component.
    /// 
    /// # Examples
    /// ```
    /// use js_linalg::Matrix4;
    /// let a = Matrix4::new();
    /// let b = Matrix4::new();
    /// let c = &a + b;
    /// assert_eq!(c.m_0_0, 2.);
    /// ```
    fn add(self, rhs: Matrix4) -> Matrix4 {
        Matrix4 {
            m_0_0: self.m_0_0 + rhs.m_0_0,
            m_0_1: self.m_0_1 + rhs.m_0_1,
            m_0_2: self.m_0_2 + rhs.m_0_1,
            m_0_3: self.m_0_3 + rhs.m_0_3,
            m_1_0: self.m_1_0 + rhs.m_1_0,
            m_1_1: self.m_1_1 + rhs.m_1_1,
            m_1_2: self.m_1_2 + rhs.m_1_2,
            m_1_3: self.m_1_3 + rhs.m_1_3,
            m_2_0: self.m_2_0 + rhs.m_2_0,
            m_2_1: self.m_2_1 + rhs.m_2_1,
            m_2_2: self.m_2_2 + rhs.m_2_2,
            m_2_3: self.m_2_3 + rhs.m_2_3,
            m_3_0: self.m_3_0 + rhs.m_3_0,
            m_3_1: self.m_3_1 + rhs.m_3_1,
            m_3_2: self.m_3_2 + rhs.m_3_2,
            m_3_3: self.m_3_3 + rhs.m_3_3,
        }
    }
}

impl std::ops::Add<&Matrix4> for &Matrix4 {
    type Output = Matrix4;

    /// Overrides '+' operator to add one `Matrix4` to another component by component.
    /// 
    /// # Examples
    /// ```
    /// use js_linalg::Matrix4;
    /// let a = Matrix4::new();
    /// let b = Matrix4::new();
    /// let c = &a + &b;
    /// assert_eq!(c.m_0_0, 2.);
    /// ```
    fn add(self, rhs: &Matrix4) -> Matrix4 {
        Matrix4 {
            m_0_0: self.m_0_0 + rhs.m_0_0,
            m_0_1: self.m_0_1 + rhs.m_0_1,
            m_0_2: self.m_0_2 + rhs.m_0_1,
            m_0_3: self.m_0_3 + rhs.m_0_3,
            m_1_0: self.m_1_0 + rhs.m_1_0,
            m_1_1: self.m_1_1 + rhs.m_1_1,
            m_1_2: self.m_1_2 + rhs.m_1_2,
            m_1_3: self.m_1_3 + rhs.m_1_3,
            m_2_0: self.m_2_0 + rhs.m_2_0,
            m_2_1: self.m_2_1 + rhs.m_2_1,
            m_2_2: self.m_2_2 + rhs.m_2_2,
            m_2_3: self.m_2_3 + rhs.m_2_3,
            m_3_0: self.m_3_0 + rhs.m_3_0,
            m_3_1: self.m_3_1 + rhs.m_3_1,
            m_3_2: self.m_3_2 + rhs.m_3_2,
            m_3_3: self.m_3_3 + rhs.m_3_3,
        }
    }
}

impl std::ops::Add<&Matrix4> for Matrix4 {
    type Output = Matrix4;

    /// Overrides '+' operator to add one `Matrix4` to another component by component.
    /// 
    /// # Examples
    /// ```
    /// use js_linalg::Matrix4;
    /// let a = Matrix4::new();
    /// let b = Matrix4::new();
    /// let c = &a + &b;
    /// assert_eq!(c.m_0_0, 2.);
    /// ```
    fn add(self, rhs: &Matrix4) -> Matrix4 {
        Matrix4 {
            m_0_0: self.m_0_0 + rhs.m_0_0,
            m_0_1: self.m_0_1 + rhs.m_0_1,
            m_0_2: self.m_0_2 + rhs.m_0_1,
            m_0_3: self.m_0_3 + rhs.m_0_3,
            m_1_0: self.m_1_0 + rhs.m_1_0,
            m_1_1: self.m_1_1 + rhs.m_1_1,
            m_1_2: self.m_1_2 + rhs.m_1_2,
            m_1_3: self.m_1_3 + rhs.m_1_3,
            m_2_0: self.m_2_0 + rhs.m_2_0,
            m_2_1: self.m_2_1 + rhs.m_2_1,
            m_2_2: self.m_2_2 + rhs.m_2_2,
            m_2_3: self.m_2_3 + rhs.m_2_3,
            m_3_0: self.m_3_0 + rhs.m_3_0,
            m_3_1: self.m_3_1 + rhs.m_3_1,
            m_3_2: self.m_3_2 + rhs.m_3_2,
            m_3_3: self.m_3_3 + rhs.m_3_3,
        }
    }
}

#[test]
fn test_add_matrix4_completeness() {
    let a = Matrix4::new();
    let b = Matrix4::new();
    let _c = &a + &b;
    let _c = a.clone() + b.clone();
    let _c = a.clone() + &b;
    let _c = &a + b;
}

#[test]
fn test_new_matrix4() {
    let m = Matrix4::new();
    assert!(m.m_0_0 == 1.0);
    assert!(m.m_1_1 == 1.0);
    assert!(m.m_2_2 == 1.0);
}

impl std::ops::Sub<Matrix4> for Matrix4 {
    type Output = Matrix4;

    /// Overrides '-' operator to subtract one `Matrix4` from another, component by component.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Matrix4;
    /// let     a = Matrix4::new();
    /// let mut b = Matrix4::new();
    /// b.set_row(0, 10., 10., 10., 10.0);
    /// let c = a - b;
    /// assert_eq!(c.m_0_0, -9.);
    /// ```
    fn sub(self, rhs: Matrix4) -> Matrix4 {
        Matrix4 {
            m_0_0: self.m_0_0 - rhs.m_0_0,
            m_0_1: self.m_0_1 - rhs.m_0_1,
            m_0_2: self.m_0_2 - rhs.m_0_1,
            m_0_3: self.m_0_3 - rhs.m_0_3,
            m_1_0: self.m_1_0 - rhs.m_1_0,
            m_1_1: self.m_1_1 - rhs.m_1_1,
            m_1_2: self.m_1_2 - rhs.m_1_2,
            m_1_3: self.m_1_3 - rhs.m_1_3,
            m_2_0: self.m_2_0 - rhs.m_2_0,
            m_2_1: self.m_2_1 - rhs.m_2_1,
            m_2_2: self.m_2_2 - rhs.m_2_2,
            m_2_3: self.m_2_3 - rhs.m_2_3,
            m_3_0: self.m_3_0 - rhs.m_3_0,
            m_3_1: self.m_3_1 - rhs.m_3_1,
            m_3_2: self.m_3_2 - rhs.m_3_2,
            m_3_3: self.m_3_3 - rhs.m_3_3,
        }
    }
}

impl std::ops::Sub<Matrix4> for &Matrix4 {
    type Output = Matrix4;

    /// Overrides '-' operator to subtract one `Matrix4` from another, component by component.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Matrix4;
    /// let     a = Matrix4::new();
    /// let mut b = Matrix4::new();
    /// b.set_row(0, 10., 10., 10., 10.0);
    /// let c = &a - b;
    /// assert_eq!(c.m_0_0, -9.);
    /// ```
    fn sub(self, rhs: Matrix4) -> Matrix4 {
        Matrix4 {
            m_0_0: self.m_0_0 - rhs.m_0_0,
            m_0_1: self.m_0_1 - rhs.m_0_1,
            m_0_2: self.m_0_2 - rhs.m_0_1,
            m_0_3: self.m_0_3 - rhs.m_0_3,
            m_1_0: self.m_1_0 - rhs.m_1_0,
            m_1_1: self.m_1_1 - rhs.m_1_1,
            m_1_2: self.m_1_2 - rhs.m_1_2,
            m_1_3: self.m_1_3 - rhs.m_1_3,
            m_2_0: self.m_2_0 - rhs.m_2_0,
            m_2_1: self.m_2_1 - rhs.m_2_1,
            m_2_2: self.m_2_2 - rhs.m_2_2,
            m_2_3: self.m_2_3 - rhs.m_2_3,
            m_3_0: self.m_3_0 - rhs.m_3_0,
            m_3_1: self.m_3_1 - rhs.m_3_1,
            m_3_2: self.m_3_2 - rhs.m_3_2,
            m_3_3: self.m_3_3 - rhs.m_3_3,
        }
    }
}

impl std::ops::Sub<&Matrix4> for &Matrix4 {
    type Output = Matrix4;

    /// Overrides '-' operator to subtract one `Matrix4` from another, component by component.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Matrix4;
    /// let     a = Matrix4::new();
    /// let mut b = Matrix4::new();
    /// b.set_row(0, 10., 10., 10., 10.0);
    /// let c = &a - &b;
    /// assert_eq!(c.m_0_0, -9.);
    /// ```
    #[inline]
    fn sub(self, rhs: &Matrix4) -> Matrix4 {
        Matrix4 {
            m_0_0: self.m_0_0 - rhs.m_0_0,
            m_0_1: self.m_0_1 - rhs.m_0_1,
            m_0_2: self.m_0_2 - rhs.m_0_1,
            m_0_3: self.m_0_3 - rhs.m_0_3,
            m_1_0: self.m_1_0 - rhs.m_1_0,
            m_1_1: self.m_1_1 - rhs.m_1_1,
            m_1_2: self.m_1_2 - rhs.m_1_2,
            m_1_3: self.m_1_3 - rhs.m_1_3,
            m_2_0: self.m_2_0 - rhs.m_2_0,
            m_2_1: self.m_2_1 - rhs.m_2_1,
            m_2_2: self.m_2_2 - rhs.m_2_2,
            m_2_3: self.m_2_3 - rhs.m_2_3,
            m_3_0: self.m_3_0 - rhs.m_3_0,
            m_3_1: self.m_3_1 - rhs.m_3_1,
            m_3_2: self.m_3_2 - rhs.m_3_2,
            m_3_3: self.m_3_3 - rhs.m_3_3,
        }
    }
}

impl std::ops::AddAssign<Matrix4> for Matrix4 {
    /// Overrides '+=' operator to add a `Matrix4` to another in-place, component by component.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Matrix4;
    /// let     a = Matrix4::new();
    /// let mut b = Matrix4::new();
    /// b.set_row(0, 10., 10., 10., 10.0);
    /// b += a;
    /// assert_eq!(b.m_0_0, 11.);
    /// ```
    #[inline]
    fn add_assign(&mut self, m: Matrix4) {
        self.add_mut_matrix4(&m);
    }
}

impl std::ops::AddAssign<&Matrix4> for Matrix4 {
    /// Overrides '+=' operator to add a `Matrix4` to another in-place, component by component.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Matrix4;
    /// let     a = Matrix4::new();
    /// let mut b = Matrix4::new();
    /// b.set_row(0, 10., 10., 10., 10.0);
    /// b += &a;
    /// assert_eq!(b.m_0_0, 11.);
    /// ```
    fn add_assign(&mut self, m: &Matrix4) {
        self.add_mut_matrix4(m);
    }
}

impl std::ops::SubAssign<Matrix4> for Matrix4 {
    /// Overrides '-=' operator to subtract a `Matrix4` from another in-place, component by component.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Matrix4;
    /// let     a = Matrix4::new();
    /// let mut b = Matrix4::new();
    /// b.set_row(0, 10., 10., 10., 10.0);
    /// b -= a;
    /// assert_eq!(b.m_0_0, 9.);
    /// ```
    fn sub_assign(&mut self, m: Matrix4) {
        self.sub_mut_matrix4(&m);
    }
}

impl std::ops::SubAssign<&Matrix4> for Matrix4 {
    /// Overrides '-=' operator to subtract a `Matrix4` from another in-place, component by component.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Matrix4;
    /// let     a = Matrix4::new();
    /// let mut b = Matrix4::new();
    /// b.set_row(0, 10., 10., 10., 10.0);
    /// b -= &a;
    /// assert_eq!(b.m_0_0, 9.);
    /// ```
    fn sub_assign(&mut self, m: &Matrix4) {
        self.sub_mut_matrix4(m);
    }
}

impl std::ops::Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    /// Overrides '*' operator to multiply a `Matrix4` with a `vector3`, where the fourth dimension of the vector is hard-coded to 1.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector3};
    /// let mat = Matrix4::new();
    /// let vec = Vector3::from_i32(3, 4, 5);
    /// 
    /// let c = mat * vec;
    /// assert_eq!(c.x, 3.);
    /// assert_eq!(c.y, 4.);
    /// assert_eq!(c.z, 5.);
    /// ```
    fn mul(self, rhs: Vector3) -> Vector3 {
        let mut b = Vector3::from_vector3(&rhs);
        self.mult_to_vector3(&mut b, 1.0);
        b
    }
}

impl std::ops::Mul<&Vector3> for Matrix4 {
    type Output = Vector3;

    /// Overrides '*' operator to multiply a `Matrix4` with a `vector3`, where the fourth dimension of the vector is hard-coded to 1.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector3};
    /// let mat = Matrix4::new();
    /// let vec = Vector3::from_i32(3, 4, 5);
    /// 
    /// let c = mat * &vec;
    /// assert_eq!(c.x, 3.);
    /// assert_eq!(c.y, 4.);
    /// assert_eq!(c.z, 5.);
    /// ```
    fn mul(self, rhs: &Vector3) -> Vector3 {
        let mut b = Vector3::from_vector3(rhs);
        self.mult_to_vector3(&mut b, 1.0);
        b
    }
}

impl std::ops::Mul<&Vector3> for &Matrix4 {
    type Output = Vector3;

    /// Overrides '*' operator to multiply a `Matrix4` with a `vector3`, where the fourth dimension of the vector is hard-coded to 1.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector3};
    /// let mat = Matrix4::new();
    /// let vec = Vector3::from_i32(3, 4, 5);
    /// 
    /// let c = &mat * &vec;
    /// assert_eq!(c.x, 3.);
    /// assert_eq!(c.y, 4.);
    /// assert_eq!(c.z, 5.);
    /// ```
    fn mul(self, rhs: &Vector3) -> Vector3 {
        let mut b = Vector3::from_vector3(rhs);
        self.mult_to_vector3(&mut b, 1.0);
        b
    }
}

impl std::ops::Mul<Vector3> for &Matrix4 {
    type Output = Vector3;

    /// Overrides '*' operator to multiply a `Matrix4` with a `vector3`, where the fourth dimension of the vector is hard-coded to 1.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector3};
    /// let mat = Matrix4::new();
    /// let vec = Vector3::from_i32(3, 4, 5);
    /// 
    /// let c = &mat * vec;
    /// assert_eq!(c.x, 3.);
    /// assert_eq!(c.y, 4.);
    /// assert_eq!(c.z, 5.);
    /// ```
    fn mul(self, rhs: Vector3) -> Vector3 {
        let mut b = Vector3::from_vector3(&rhs);
        self.mult_to_vector3(&mut b, 1.0);
        b
    }
}

impl std::ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    /// Overrides '*' operator to multiply a `Matrix4` with another `Matrix4`.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector3};
    /// let m = Matrix4::new();
    /// let u = Matrix4::new();
    /// 
    /// let c = m * u;
    /// ```
    fn mul(self, rhs: Matrix4) -> Matrix4 {
        let mut a = Matrix4::from_matrix4(&self);
        a.mult_mut(&rhs);
        a
    }
}


impl std::ops::Mul<&Matrix4> for Matrix4 {
    type Output = Matrix4;

    /// Overrides '*' operator to multiply a `Matrix4` with another `Matrix4`.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector3};
    /// let m = Matrix4::new();
    /// let u = Matrix4::new();
    /// 
    /// let c = m * &u;
    /// ```
    fn mul(self, rhs: &Matrix4) -> Matrix4 {
        let mut a = Matrix4::from_matrix4(&self);
        a.mult_mut(rhs);
        a
    }
}


impl std::ops::Mul<&Matrix4> for &Matrix4 {
    type Output = Matrix4;

    /// Overrides '*' operator to multiply a `Matrix4` with another `Matrix4`.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector3};
    /// let m = Matrix4::new();
    /// let u = Matrix4::new();
    /// 
    /// let c = &m * &u;
    /// ```
    fn mul(self, rhs: &Matrix4) -> Matrix4 {
        let mut a = Matrix4::from_matrix4(&self);
        a.mult_mut(rhs);
        a
    }
}

impl std::ops::Mul<Matrix4> for &Matrix4 {
    type Output = Matrix4;

    /// Overrides '*' operator to multiply a `Matrix4` with another `Matrix4`.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector3};
    /// let m = Matrix4::new();
    /// let u = Matrix4::new();
    /// 
    /// let c = &m * &u;
    /// ```
    fn mul(self, rhs: Matrix4) -> Matrix4 {
        let mut a = Matrix4::from_matrix4(&self);
        a.mult_mut(&rhs);
        a
    }
}

impl std::ops::MulAssign<&Matrix4> for Matrix4 {
    /// Overrides '*=' operator to multiply a `Matrix4` with another in-place.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Matrix4;
    /// let     a = Matrix4::new();
    /// let mut b = Matrix4::new();
    /// b.set_row(0, 10., 10., 10., 10.0);
    /// b *= &a;
    /// assert_eq!(b.m_0_0, 10.);
    /// ```
    fn mul_assign(&mut self, m: &Matrix4) {
        self.mult_mut(m);
    }
}

impl std::ops::MulAssign<Matrix4> for Matrix4 {
    /// Overrides '*=' operator to multiply a `Matrix4` with another in-place.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::Matrix4;
    /// let     a = Matrix4::new();
    /// let mut b = Matrix4::new();
    /// b.set_row(0, 10., 10., 10., 10.0);
    /// b *= a;
    /// assert_eq!(b.m_0_0, 10.);
    /// ```
    fn mul_assign(&mut self, m: Matrix4) {
        self.mult_mut(&m);
    }
}

impl std::ops::Mul<&Vector4> for Matrix4 {
    type Output = Vector4;

    /// Overrides '*' operator to multiply a `Matrix4` with a `vector3`, where the fourth dimension of the vector is hard-coded to 1.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector4};
    /// let mat = Matrix4::new();
    /// let vec = Vector4::from_i32(3, 4, 5, 6);
    /// 
    /// let c = mat * &vec;
    /// assert_eq!(c.x, 3.);
    /// assert_eq!(c.y, 4.);
    /// assert_eq!(c.z, 5.);
    /// assert_eq!(c.w, 6.);
    /// ```
    fn mul(self, rhs: &Vector4) -> Vector4 {
        let mut b = Vector4::from_vector4(rhs);
        self.mult_to_vector4(&mut b);
        b
    }
}

impl std::ops::Mul<&Vector4> for &Matrix4 {
    type Output = Vector4;

    /// Overrides '*' operator to multiply a `Matrix4` with a `vector3`, where the fourth dimension of the vector is hard-coded to 1.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector4};
    /// let mat = Matrix4::new();
    /// let vec = Vector4::from_i32(3, 4, 5, 6);
    /// 
    /// let c = &mat * &vec;
    /// assert_eq!(c.x, 3.);
    /// assert_eq!(c.y, 4.);
    /// assert_eq!(c.z, 5.);
    /// assert_eq!(c.w, 6.);
    /// ```
    fn mul(self, rhs: &Vector4) -> Vector4 {
        let mut b = Vector4::from_vector4(rhs);
        self.mult_to_vector4(&mut b);
        b
    }
}

impl std::ops::Mul<Vector4> for &Matrix4 {
    type Output = Vector4;

    /// Overrides '*' operator to multiply a `Matrix4` with a `vector3`, where the fourth dimension of the vector is hard-coded to 1.
    /// 
    /// # Example
    /// ```
    /// use js_linalg::{Matrix4, Vector4};
    /// let mat = Matrix4::new();
    /// let vec = Vector4::from_i32(3, 4, 5, 6);
    /// 
    /// let c = &mat * &vec;
    /// assert_eq!(c.x, 3.);
    /// assert_eq!(c.y, 4.);
    /// assert_eq!(c.z, 5.);
    /// assert_eq!(c.w, 6.);
    /// ```
    fn mul(self, rhs: Vector4) -> Vector4 {
        let mut b = Vector4::from_vector4(&rhs);
        self.mult_to_vector4(&mut b);
        b
    }
}

