use crate::Vector3;

impl Vector3 {

    pub fn zero() -> Vector3 {
        Vector3 {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {x: x, y: y, z: z}
    }

    pub fn from_vec3(vec: &Vector3) -> Vector3 {
        Vector3::new(vec.x, vec.y, vec.z)
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


//}