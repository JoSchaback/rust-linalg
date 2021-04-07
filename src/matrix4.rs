// pub mod matrix4 {

    use crate::Vector3;
    use crate::Vector4;
    use crate::Matrix4;

    #[allow(dead_code)]


    //trait Matrix4 : Sized {}

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

        pub fn look_at(&mut self, eye: &Vector3, center: &Vector3, up: &Vector3) {
            let mut u = Vector3::new();
            let mut v = Vector3::new();
            let mut w = Vector3::from_vec3(eye);

            // the w vector is computed by w = eye - center which means
            // it is the inverse of the viewing direction.
            w.copy(&eye);
            w.sub_vec(&center);
            w.normalize();

            // compute cross product
            u.copy(&up);
            u.cross(&w);
            u.normalize();
            // side = (0,0,1) x w

            // up = side x look
            v.copy(&w);
            v.cross(&u);
            v.normalize();
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

//}