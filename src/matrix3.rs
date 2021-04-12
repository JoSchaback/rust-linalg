use super::{Matrix3, Matrix4};

impl Matrix3 {

    /// Creates a new `Matrix3` struct as identity matrix.
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

    pub fn calc_mut_normal_matrix(&mut self, view: &Matrix4) -> &mut Matrix3 {
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

        self
    }

    pub fn normal_matrix(view:&Matrix4) -> Matrix3 {
        let mut m = Matrix3::new();
        m.calc_mut_normal_matrix(view);
        m        
    }

    pub fn set_matrix3(&mut self, m:&Matrix3) {
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
}