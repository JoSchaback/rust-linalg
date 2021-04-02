use crate::Vector3;

impl Vector3 {

    pub fn new() -> Vector3 {
        Vector3 {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn from_vec3(vec: &Vector3) -> Vector3 {
        Vector3 {x:vec.x, y:vec.y, z:vec.z}
    }

    pub fn from(x:f32, y:f32, z:f32) -> Vector3 {
        Vector3 {x:x, y:y, z:z}
    }


    pub fn from_i32(x:i32, y:i32, z:i32) -> Vector3 {
        Vector3 {x:x as f32, y:y as f32, z:z as f32}
    }

    pub fn normalize(&mut self) -> &mut Vector3 {

        let d = ( self.x*self.x + self.y*self.y + self.z*self.z ) .sqrt();

        self.x /= d;
        self.y /= d;
        self.z /= d;

        self
    }

    pub fn set(&mut self, xp:f32, yp:f32, zp:f32) {
        self.x = xp;
        self.y = yp;
        self.z = zp;
    }

    pub fn copy(&mut self, vec:&Vector3) {
        self.x = vec.x;
        self.y = vec.y;
        self.z = vec.z;
    }

    pub fn sub(&mut self, xp:f32, yp:f32, zp:f32) {
        self.x -= xp;
        self.y -= yp;
        self.z -= zp;
    }

    pub fn sub_vec(&mut self, vec:&Vector3) {
        self.x -= vec.x;
        self.y -= vec.y;
        self.z -= vec.z;
    }

    pub fn add(&mut self, xp:f32, yp:f32, zp:f32) {
        self.x += xp;
        self.y += yp;
        self.z += zp;
    }

    pub fn cross(&mut self, a:&Vector3) {
        let temp_x = self.y * a.z - self.z * a.y;
        let temp_y = self.z * a.x - self.x * a.z;
        let temp_z = self.x * a.y - self.y * a.x;

        self.x = temp_x;
        self.y = temp_y;
        self.z = temp_z;
    }
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