//!
//! Quaternions, for gimbal-lock free rotations.
//!
//! A Quaternions is a 4-dimensional complex number, holding a scalar part and 3 imaginary parts,
//! *i*, *j*, and *k*.
//!
//! In mathematics, the *i* (imaginary unit) is defined as the square root of -1. Another way to
//! think about complex number is to view the complex plane as orthogonal to the real one. As such,
//! it becomes easy to represent a complex number on a 2D plane, with one axis being the real
//! numbers, and the other the imaginary one. When working in this 2D space, any multiplication by
//! *i* becomes a 90 degree rotation.
//!
//! Quaternions is an extension of this property, with each imaginary number representing a 90
//! degree rotations. As such, the following properties holds:
//! - `i^2 = -1`
//! - `j^2 = -1`
//! - `k^2 = -1`
//! - `i * j * k = -1`
//! - `i * j = -j * i = k`
//! - `j * k = -k * j = i`
//! - `i * k = -k * i = j`
//!

use std::ops::{Div, Mul, MulAssign};

use super::Vector;

///
/// Quaternion data-structure.
///
/// See [module documentation](self) for more informations.
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quaternion {
    /// Real part
    pub r: f32,
    /// First order imaginary
    pub i: f32,
    /// Second order imaginary
    pub j: f32,
    /// Third order imaginary
    pub k: f32,
}

impl Quaternion {
    ///
    /// Create a new quaternion from parts.
    ///
    pub const fn new(r: f32, i: f32, j: f32, k: f32) -> Self {
        Self { r, i, j, k }
    }

    ///
    /// Zero quaternion.
    ///
    pub const ZERO: Self = Self::new(0., 0., 0., 0.);

    ///
    /// Identity quaternion.
    ///
    pub const IDENTITY: Self = Self::new(1., 0., 0., 0.);

    ///
    /// Create a new vector quaternion from a vector.
    ///
    pub fn from_vector(v: Vector) -> Self {
        Self::new(0., v.x, v.y, v.z)
    }

    ///
    /// Create a new scalar quaternion.
    ///
    pub fn from_scalar(s: f32) -> Self {
        Self { r: s, ..Self::ZERO }
    }

    ///
    /// Create a quaternion enabling a rotation around an axis.
    ///
    /// `angle` is assumed to be in radian.
    ///
    pub fn from_rotation(angle: f32, axis: Vector) -> Self {
        let angle = angle / 2.;
        let (sin, cos) = angle.sin_cos();

        Self {
            r: cos,
            ..Self::from_vector(sin * axis)
        }
        .normalize()
    }

    ///
    /// Create a quaternion for a rotation around the x-axis.
    ///
    pub fn from_x_rot(angle: f32) -> Self {
        Self::from_rotation(angle, Vector::X)
    }

    ///
    /// Create a quaternion for a rotation around the y-axis.
    ///
    pub fn from_y_rot(angle: f32) -> Self {
        Self::from_rotation(angle, Vector::Y)
    }

    ///
    /// Create a quaternion for a rotation around the z-axis.
    ///
    pub fn from_z_rot(angle: f32) -> Self {
        Self::from_rotation(angle, Vector::Z)
    }

    ///
    /// Create a quaternion for transforming `from` to `to`.
    ///
    /// The inputs vector must be normalized.
    ///
    /// The way this function works is by first computing the dot and cross product of the inputs
    /// vector, and building a quaternion from them.
    ///
    /// This gives us with a quaternion that would rotate a vector by twice the angle. Once we have
    /// this quaternion, we can then compute the "halfway" quaternion by adding the identity quaternion
    /// to it and normalizing the result.
    ///
    /// # Panics
    /// Panics in debug if either `from` or `to` are not normalized.
    ///
    pub fn from_arc(from: Vector, to: Vector) -> Self {
        debug_assert!(
            from.is_normalized(),
            "Quaternion::from_arc: from is not a unit vector."
        );
        debug_assert!(
            from.is_normalized(),
            "Quaternion::from_arc: to is not a unit vector."
        );

        const DOT_EPSILON: f32 = 1.0 - 2.0 * f32::EPSILON;

        let dot = from.dot(to);

        if dot > DOT_EPSILON {
            Self::IDENTITY
        } else if dot < -DOT_EPSILON {
            Self::from_rotation(std::f32::consts::PI, from.any_orthonormal())
        } else {
            let cross = from.cross(to);
            Self::new(1. + dot, cross.x, cross.y, cross.z).normalize()
        }
    }

    ///
    /// Return the conjugate of the quaternion.
    ///
    pub fn conjugate(self) -> Quaternion {
        Self::new(self.r, -self.i, -self.j, -self.k)
    }

    ///
    /// Square of the quaternion norm.
    ///
    pub fn norm2(self) -> f32 {
        self.r.powi(2) + self.i.powi(2) + self.j.powi(2) + self.k.powi(2)
    }

    ///
    /// Return the quaternion norm, that is, its length.
    ///
    pub fn norm(self) -> f32 {
        self.norm2().sqrt()
    }

    ///
    /// Transform the quaternion into a unit quaternion
    ///
    pub fn normalize(self) -> Self {
        debug_assert!(
            self.norm2() != 0.,
            "Quaternion::normalize: Cannot normalize a quaternion of norm 0."
        );
        self / self.norm()
    }

    ///
    /// Takes the reciprocal of a unit quaternion.
    ///
    /// The quaternion should be normalized beforehand
    ///
    pub fn recip(self) -> Self {
        debug_assert!(
            self.is_normalized(),
            "Quaternion::recip: self is not a unit vector"
        );
        self.conjugate()
    }

    ///
    /// Check if the quaternion is a unit quaternion.
    ///
    pub fn is_normalized(self) -> bool {
        f32::abs(self.norm() - 1.0) <= 1e-6
    }

    ///
    /// Apply the rotation denoted by the unit quaternion to a given vector.
    ///
    /// `self` is assumed to be normalized.
    ///
    /// # Panics
    /// Will panic in debug if `self` is not a unit quaternion.
    ///
    pub fn rotate_vector(self, v: Vector) -> Vector {
        debug_assert!(
            self.is_normalized(),
            "Quaternion::rotate_vector: self is not a unit vector"
        );
        (self * Self::from_vector(v) * self.recip()).into()
    }
}

impl From<Vector> for Quaternion {
    ///
    /// Convert a `Vector` to a vector quaternion.
    ///
    fn from(value: Vector) -> Self {
        Self::from_vector(value)
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    ///
    /// Multiply two quaternions by applying the Hamilton product.
    ///
    /// If they each represent a rotation, the result will represent the combined rotation.
    ///
    /// # Panics
    /// Will panic if `self` or `rhs` are not unit quaternions
    ///
    fn mul(self, rhs: Self) -> Self::Output {
        let r = self.r * rhs.r - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k;
        let i = self.r * rhs.i + self.i * rhs.r + self.j * rhs.k - self.k * rhs.j;
        let j = self.r * rhs.j - self.i * rhs.k + self.j * rhs.r + self.k * rhs.i;
        let k = self.r * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.r;

        Self::new(r, i, j, k)
    }
}

impl Mul<Vector> for Quaternion {
    type Output = Vector;

    ///
    /// Multiply a vector by a quaternion, returning the rotated vector
    ///
    /// # Panics
    /// In debug, will panic if self is not a unit quaternion.
    ///
    fn mul(self, rhs: Vector) -> Self::Output {
        self.rotate_vector(rhs)
    }
}

impl Mul<Quaternion> for Vector {
    type Output = Vector;

    ///
    /// Multiply a vector by a quaternion, returning the rotated vector
    ///
    /// # Panics
    /// In debug, will panic if self is not a unit quaternion.
    ///
    fn mul(self, rhs: Quaternion) -> Self::Output {
        rhs.rotate_vector(self)
    }
}

impl MulAssign<Quaternion> for Vector {
    ///
    /// Multiply a vector by a quaternion, returning the rotated vector
    ///
    /// # Panics
    /// In debug, will panic if self is not a unit quaternion.
    ///
    fn mul_assign(&mut self, rhs: Quaternion) {
        *self = rhs.rotate_vector(*self)
    }
}

impl MulAssign for Quaternion {
    ///
    /// Multiply two quaternions by applying the Hamilton product.
    ///
    /// If they each represent a rotation, the result will represent the combined rotation.
    ///
    /// # Panics
    /// Will panic if `self` or `rhs` are not unit quaternions
    ///
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    ///
    /// Scale the quaternion norm by a given factor.
    ///
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.r * rhs, self.i * rhs, self.j * rhs, self.k * rhs)
    }
}

impl Mul<Quaternion> for f32 {
    type Output = Quaternion;

    ///
    /// Scale the quaternion norm by a given factor.
    ///
    fn mul(self, rhs: Quaternion) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Quaternion {
    ///
    /// Scale the quaternion norm by a given factor
    ///
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Quaternion {
    type Output = Quaternion;

    ///
    /// Divide a quaternion by a scalar.
    ///
    /// This is a scale operation.
    ///
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.r / rhs, self.i / rhs, self.j / rhs, self.k / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_vector_empty_scalar_test() {
        let v = Quaternion::from_vector(Vector::new(1., 2., 3.));

        assert_eq!(v.r, 0.);
    }

    #[test]
    fn from_scalar_vector_zero_test() {
        let v = Quaternion::from_scalar(3.5);

        assert_eq!(v.i, 0.);
        assert_eq!(v.j, 0.);
        assert_eq!(v.k, 0.);
    }

    #[test]
    fn from_arc_no_rotation() {
        let q = Quaternion::from_arc(Vector::X, Vector::X);

        assert_eq!(q, Quaternion::IDENTITY);
        assert_eq!(q * Vector::X, Vector::X);
    }

    #[test]
    fn from_arc_half_rotation() {
        let expected = -Vector::X;
        let q = Quaternion::from_arc(Vector::X, expected);

        let res = q * Vector::X;

        let threshold = 1e-7;
        let adj_x = f32::abs(expected.x - res.x);
        let adj_y = f32::abs(expected.y - res.y);
        let adj_z = f32::abs(expected.z - res.z);

        assert!(adj_x <= threshold);
        assert!(adj_y <= threshold);
        assert!(adj_z <= threshold);
    }

    #[test]
    fn from_arc_arbitrary_rotation() {
        let expected = Vector::new(0., 1., 2.).normalize();
        let q = Quaternion::from_arc(Vector::X, expected);
        let res = q * Vector::X;

        let threshold = 1e-7;
        let adj_x = f32::abs(expected.x - res.x);
        let adj_y = f32::abs(expected.y - res.y);
        let adj_z = f32::abs(expected.z - res.z);

        assert!(adj_x <= threshold);
        assert!(adj_y <= threshold);
        assert!(adj_z <= threshold);
    }

    #[test]
    fn simple_rotation_test() {
        let q_rot_x = Quaternion::from_x_rot(f32::to_radians(45.));
        let q_rot_y = Quaternion::from_y_rot(f32::to_radians(45.));
        let q_rot_z = Quaternion::from_z_rot(f32::to_radians(45.));

        let v_x = Vector::X;
        let v_y = Vector::Y;
        let v_z = Vector::Z;

        let rot_v_x = v_x * q_rot_z;
        let rot_v_y = v_y * q_rot_x;
        let rot_v_z = v_z * q_rot_y;

        assert_ne!(v_x, rot_v_x);
        assert_ne!(v_y, rot_v_y);
        assert_ne!(v_z, rot_v_z);

        assert!(f32::abs(f32::acos(v_x.dot(rot_v_x)) - f32::to_radians(45.)) <= 1e-6);
        assert!(f32::abs(f32::acos(v_y.dot(rot_v_y)) - f32::to_radians(45.)) <= 1e-6);
        assert!(f32::abs(f32::acos(v_z.dot(rot_v_z)) - f32::to_radians(45.)) <= 1e-6);
    }

    #[test]
    fn mul_quaternion_combine_rotation() {
        let q_rot_x = Quaternion::from_x_rot(f32::to_radians(45.));
        let q_rot_y = Quaternion::from_y_rot(f32::to_radians(45.));
        let q_xy = q_rot_x * q_rot_y;

        let mut v_z = Vector::Z;

        v_z *= q_rot_y;
        v_z *= q_rot_x;

        assert!(f32::abs(f32::acos((Vector::Z * q_xy).dot(v_z)) - f32::to_radians(0.)) <= 1e-6);
    }
}
