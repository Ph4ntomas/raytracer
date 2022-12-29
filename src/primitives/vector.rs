use std::ops;
use super::Quaternion;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn normalize(&self) -> Self {
        self / self.magnitude()
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.magnitude_sqr())
    }

    pub fn magnitude_sqr(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Vector) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vector) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub fn angle(&self, rhs: &Vector) -> f32 {
        f32::acos((self.dot(rhs) / self.magnitude()) / rhs.magnitude())
    }
}

impl TryFrom<Quaternion> for Vector {
    type Error = String;

    fn try_from(value: Quaternion) -> Result<Self, Self::Error> {
        if value.a != 0. {
            return Err("Input value is not a valid vector (quaternion contains real part).".to_string());
        }

        Ok(Self {
            x: value.b,
            y: value.c,
            z: value.d
        })
    }
}

impl TryFrom<&Quaternion> for Vector {
    type Error = String;

    fn try_from(value: &Quaternion) -> Result<Self, Self::Error> {
        if value.a != 0. {
            return Err("Input value is not a valid vector (quaternion contains real part).".to_string());
        }

        Ok(Self {
            x: value.b,
            y: value.c,
            z: value.d
        })
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Add<&Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<'a> ops::Add<Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<'a, 'b> ops::Add<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, rhs: &'b Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Sub<&Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<'a> ops::Sub<Vector> for &'a Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn sub(self, rhs: &'b Vector) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl ops::Mul<&Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl<'a> ops::Mul<Vector> for &'a f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl<'a, 'b> ops::Mul<&'b Vector> for &'a f32 {
    type Output = Vector;

    fn mul(self, rhs: &'b Vector) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl ops::Mul<&f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl<'a> ops::Mul<f32> for &'a Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl<'a, 'b> ops::Mul<&'b f32> for &'a Vector {
    type Output = Vector;

    fn mul(self, rhs: &'b f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl ops::Div<&f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl<'a> ops::Div<f32> for &'a Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl<'a, 'b> ops::Div<&'b f32> for &'a Vector {
    type Output = Vector;

    fn div(self, rhs: &'b f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}
