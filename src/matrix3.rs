//pub mod matrix3 {

    use crate::Matrix4;
    use crate::Matrix3;

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

        pub fn calc_normal_matrix(&mut self, view: &Matrix4) {
            self.m_0_0 = view.m_0_0;
            self.m_1_0 = view.m_1_0;
            self.m_2_0 = view.m_2_0;

            self.m_0_1 = view.m_0_1;
            self.m_1_1 = view.m_1_1;
            self.m_2_1 = view.m_2_1;

            self.m_0_2 = view.m_0_2;
            self.m_1_2 = view.m_1_2;
            self.m_2_2 = view.m_2_2;

            self.inverse();
            self.transpose();
        }

        pub fn inverse(&mut self) {
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

        pub fn transpose(&mut self) {
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
        /*
        private Matrix3 calcNormalMatrix(Matrix4 mv) {
            // set upper left
            normal.m_0_0 = mv.m_0_0;
            normal.m_1_0 = mv.m_1_0;
            normal.m_2_0 = mv.m_2_0;

            normal.m_0_1 = mv.m_0_1;
            normal.m_1_1 = mv.m_1_1;
            normal.m_2_1 = mv.m_2_1;

            normal.m_0_2 = mv.m_0_2;
            normal.m_1_2 = mv.m_1_2;
            normal.m_2_2 = mv.m_2_2;

            normal.inverse();
            normal.transpose();

            return normal;
        }
        */

    }
//}