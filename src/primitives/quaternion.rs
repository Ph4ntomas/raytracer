use std::ops;
use crate::primitives::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Quaternion {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32
}

impl Quaternion {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { a, b, c, d }
    }

    pub fn new_from_vector(vector: &Vector) -> Self {
        Self {
            a: 0.,
            b: vector.x,
            c: vector.y,
            d: vector.z
        }
    }

    pub fn new_rotation(angle: f32, axis: Vector) -> Self {
        let c = f32::cos(angle/2.);
        let s = f32::sin(angle/2.);

        Self {
            a: c,
            b: axis.x * s,
            c: axis.y * s,
            d: axis.z * s
        }
    }

    pub fn conj(&self) -> Self {
        Self {
            a: self.a,
            b: -self.b,
            c: -self.c,
            d: -self.d
        }
    }

    pub fn norm_sqr(&self) -> f32 {
        self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d
    }

    pub fn norm(&self) -> f32 {
        f32::sqrt(self.norm_sqr())
    }

    pub fn normalize(&self) -> Self {
        self / self.norm()
    }

    pub fn inv(&self) -> Self {
        self / self.norm_sqr()
    }

    pub fn rotate_vector(&self, vector: Vector) -> Vector {
        let p : Quaternion = vector.into();
        (self * p * self.inv()).try_into().unwrap()
    }
}

impl From<Vector> for Quaternion {
    fn from(value: Vector) -> Self {
        Self::new_from_vector(&value)
    }
}

impl From<&Vector> for Quaternion {
    fn from(value: &Vector) -> Self {
        Self::new_from_vector(value)
    }
}

impl ops::Mul<Quaternion> for f32 {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        Self::Output {
            a: self * rhs.a,
            b: self * rhs.b,
            c: self * rhs.c,
            d: self * rhs.d
        }
    }
}

impl ops::Mul<&Quaternion> for f32 {
    type Output = Quaternion;

    fn mul(self, rhs: &Quaternion) -> Self::Output {
        Self::Output {
            a: self * rhs.a,
            b: self * rhs.b,
            c: self * rhs.c,
            d: self * rhs.d
        }
    }
}

impl<'a> ops::Mul<Quaternion> for &'a f32 {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        Self::Output {
            a: self * rhs.a,
            b: self * rhs.b,
            c: self * rhs.c,
            d: self * rhs.d
        }
    }
}

impl<'a, 'b> ops::Mul<&'b Quaternion> for &'a f32 {
    type Output = Quaternion;

    fn mul(self, rhs: &'b Quaternion) -> Self::Output {
        Self::Output {
            a: self * rhs.a,
            b: self * rhs.b,
            c: self * rhs.c,
            d: self * rhs.d
        }
    }
}

impl ops::Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
            d: self.d * rhs
        }
    }
}

impl ops::Mul<&f32> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: &f32) -> Self::Output {
        Self::Output {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
            d: self.d * rhs
        }
    }
}

impl<'a> ops::Mul<f32> for &'a Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
            d: self.d * rhs
        }
    }
}

impl<'a, 'b> ops::Mul<&'b f32> for &'a Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &'b f32) -> Self::Output {
        Self::Output {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
            d: self.d * rhs
        }
    }
}

impl ops::Div<f32> for Quaternion {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            a: self.a / rhs,
            b: self.b / rhs,
            c: self.c / rhs,
            d: self.d / rhs
        }
    }
}

impl<'a> ops::Div<f32> for &'a Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            a: self.a / rhs,
            b: self.b / rhs,
            c: self.c / rhs,
            d: self.d / rhs
        }
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        Self::Output {
            a: self.a * rhs.a - self.b * rhs.b - self.c * rhs.c - self.d * rhs.d,
            b: self.a * rhs.b + self.b * rhs.a + self.c * rhs.d - self.d * rhs.c,
            c: self.a * rhs.c - self.b * rhs.d + self.c * rhs.a + self.d * rhs.b,
            d: self.a * rhs.d + self.b * rhs.c - self.c * rhs.b + self.d * rhs.a
        }
    }
}

impl ops::Mul<&Quaternion> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: &Quaternion) -> Self::Output {
        Self::Output {
            a: self.a * rhs.a - self.b * rhs.b - self.c * rhs.c - self.d * rhs.d,
            b: self.a * rhs.b + self.b * rhs.a + self.c * rhs.d - self.d * rhs.c,
            c: self.a * rhs.c - self.b * rhs.d + self.c * rhs.a + self.d * rhs.b,
            d: self.a * rhs.d + self.b * rhs.c - self.c * rhs.b + self.d * rhs.a
        }
    }
}

impl<'a> ops::Mul<Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        Self::Output {
            a: self.a * rhs.a - self.b * rhs.b - self.c * rhs.c - self.d * rhs.d,
            b: self.a * rhs.b + self.b * rhs.a + self.c * rhs.d - self.d * rhs.c,
            c: self.a * rhs.c - self.b * rhs.d + self.c * rhs.a + self.d * rhs.b,
            d: self.a * rhs.d + self.b * rhs.c - self.c * rhs.b + self.d * rhs.a
        }
    }
}

impl<'a, 'b> ops::Mul<&'b Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &'b Quaternion) -> Self::Output {
        Self::Output {
            a: self.a * rhs.a - self.b * rhs.b - self.c * rhs.c - self.d * rhs.d,
            b: self.a * rhs.b + self.b * rhs.a + self.c * rhs.d - self.d * rhs.c,
            c: self.a * rhs.c - self.b * rhs.d + self.c * rhs.a + self.d * rhs.b,
            d: self.a * rhs.d + self.b * rhs.c - self.c * rhs.b + self.d * rhs.a
        }
    }
}
