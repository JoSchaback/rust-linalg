use crate::Vector4;

impl Vector4 {

    pub fn new() -> Vector4 {
        Vector4 {x: 0.0, y: 0.0, z: 0.0, w:0.0}
    }

    pub fn from_vec4(vec: &Vector4) -> Vector4 {
        Vector4 {x: vec.x, y: vec.y, z: vec.z, w:vec.w}
    }

    pub fn normalize(&mut self) -> &mut Vector4 {

        let d = ( self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w) .sqrt();

        self.x /= d;
        self.y /= d;
        self.z /= d;
        self.w /= d;

        self
    }

    pub fn set(&mut self, xp:f32, yp:f32, zp:f32, wp:f32) {
        self.x = xp;
        self.y = yp;
        self.z = zp;
        self.w = wp;
    }

    // TODO this should be rather clone_from()?
    pub fn copy(&mut self, vec:&Vector4) {
        self.x = vec.x;
        self.y = vec.y;
        self.z = vec.z;
        self.w = vec.w;
    }

    pub fn scale_scalar(&mut self, s:f32) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
        self.w *= s;
    }

    pub fn sub(&mut self, xp:f32, yp:f32, zp:f32, wp:f32) {
        self.x -= xp;
        self.y -= yp;
        self.z -= zp;
        self.w -= wp;
    }

    pub fn sub_vec(&mut self, vec:&Vector4) {
        self.x -= vec.x;
        self.y -= vec.y;
        self.z -= vec.z;
        self.w -= vec.w;
    }

    pub fn add(&mut self, xp:f32, yp:f32, zp:f32, wp:f32) {
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