//! Euclidean 3D Vector.
//!
//! The Vector type represents both a direction and a movement in 3-dimensional space.
//!
//! The "length" of a vector is called it's magnitude.
//! A unit vector, is any vector of magnitude 1.
//!
//! The `normalize` operation returns a unit vector, parallel to the one being normalized.
//!
//! # Properties
//! **Equality**:
//! Two vector are equal if they have the same magnitude and direction.
//!
//! **Addition and substraction**:
//! The sum of two vector is a third vector (called the resultant vector) that represent the
//! sequential application of each vectors to a given point in space. That is, for a point `P`, two
//! vectors `a` and `b`, and their resultant vector `r`, `P + a + b = P + v`.
//! As `-b` is the opposite vector (inverse direction, but same magnitude), and the addition is
//! commutative, the operation `a - b` equals the addition of the opposite of `b` to `a`.
//!
//! **Multiplication**:
//! There are two kind of vector multiplication:
//! - The dot (or scalar) product returns a scalar equals to `||a|| * ||b|| * cos(θ)`, with `θ`
//! being the angle between `a` and `b`.
//! - The cross product is a third vector, orthogonal to both.
//!
//! **Scalar Multiplication and Division**:
//! Multiplying or dividing a vector by a scalar scales the vector by a given amount. The direction
//! is left unchanged, but each of the vector's component is multiplied or divided by the given
//! scalar.
//!

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::Quaternion;

///
/// Euclidean 3D Vector.
///
/// See [module documentation](self) for more informations.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    ///
    /// Create a new Vector.
    ///
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    ///
    /// Zero vector.
    ///
    /// Adding or substracting this vector to another yield the id operation
    /// `a + zero = a`
    /// `b + zero = b`
    ///
    pub const ZERO: Self = Self::new(0., 0., 0.);

    ///
    /// Unit vector, parallel to the x axis.
    ///
    pub const X: Self = Self::new(1., 0., 0.);

    ///
    /// Unit vector, parallel to the y axis.
    ///
    pub const Y: Self = Self::new(0., 1., 0.);

    ///
    /// Unit vector, parallel to the z axis.
    ///
    pub const Z: Self = Self::new(0., 0., 1.);

    ///
    /// Return a new unit vector, orthogonal to `self`
    ///
    /// See [pixar's paper](https://graphics.pixar.com/library/OrthonormalB/paper.pdf)
    ///
    /// # Panics
    /// Panic in debug if `self` is not a unit vector
    ///
    pub fn any_orthonormal(self) -> Vector {
        let (ret, _) = self.any_orthonormal_pair();
        ret
    }

    ///
    /// Return a new unit vector pair, both orthogonal to `self` and normalized.
    ///
    /// See [pixar's paper](https://graphics.pixar.com/library/OrthonormalB/paper.pdf)
    ///
    /// # Panics
    /// Panic in debug if `self` is not a unit vector
    ///
    pub fn any_orthonormal_pair(self) -> (Vector, Vector) {
        debug_assert!(
            self.is_normalized(),
            "Vector::any_orthonormal: `self` is not a unit vector."
        );

        let sign = f32::copysign(1.0, self.z);
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;

        (
            Self::new(b, sign + self.y.powi(2) * a, -self.y),
            Self::new(1.0 + sign * self.x.powi(2) * a, sign * b, -sign * self.x),
        )
    }

    ///
    /// Compute the dot (or scalar) product of two vectors.
    ///
    /// The dot product can be computed as `a · b = ||a|| * ||b|| * cos(θ)`, with `θ` denoting the
    /// angle between `a` and `b`.
    ///
    pub fn dot(self, rhs: Vector) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    ///
    /// Compute the cross product of two vectors.
    ///
    /// The resulting vector is a vector that's orthogonal to both inputs vectors. That is, it's
    /// the normal of the plan defined by the two vectors.
    /// It is noted as `a × b`.
    ///
    pub fn cross(self, rhs: Vector) -> Vector {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    ///
    /// Return the vector's magnitude squared.
    ///
    /// The vector's magnitude is the square-root of this value. However, the square of the
    /// magnitude can be useful when comparing vectors, and is quicker to compute as it does not
    /// need to compute a square-root.
    ///
    pub fn magn2(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    ///
    /// Compute the vector magnitude.
    ///
    /// The magnitude of the vector is it's "length".
    ///
    pub fn magn(self) -> f32 {
        self.magn2().sqrt()
    }

    ///
    /// Return a unit vector with the same direction.
    ///
    /// A unit vector is defined as any vector with a magnitude of 1
    ///
    pub fn normalize(self) -> Self {
        self / self.magn()
    }

    ///
    /// Check if the vector is normalized.
    ///
    pub fn is_normalized(self) -> bool {
        f32::abs(self.magn() - 1.0) <= 1e-6
    }
}

impl From<Quaternion> for Vector {
    ///
    /// Convert a Vector Quaternion to a vector.
    ///
    fn from(value: Quaternion) -> Self {
        Self::new(value.i, value.j, value.k)
    }
}

impl Neg for Vector {
    type Output = Vector;

    ///
    /// Compute the opposite vector
    ///
    fn neg(self) -> Self::Output {
        self * -1.
    }
}

impl Add for Vector {
    type Output = Vector;

    ///
    /// Compute the resultant vector of `self` and `rhs`.
    ///
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector {
    ///
    /// Compute `self` + `rhs` and assign the result to `self`.
    ///
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector {
    type Output = Vector;

    ///
    ///
    /// Compute the resultant vector of `self` and `-rhs`.
    ///
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vector {
    ///
    /// Compute `self` - `rhs` and assign the result to `self`.
    ///
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    ///
    /// Multiply each of the vector dimension by a given factor.
    ///
    /// This operation conserves the direction of the vector, but scale it by a factor.
    ///
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f32> for Vector {
    ///
    /// Multiply each of the vector dimension by a given factor.
    ///
    /// This operation conserves the direction of the vector, but scale it by a factor.
    ///
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    ///
    /// Multiply each of the vector dimension by a given factor.
    ///
    /// This operation conserves the direction of the vector, but scale it by a factor.
    ///
    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    ///
    /// Divide each of the vector dimension by a given factor.
    ///
    /// This operation conserves the direction of the vector, but scale it by a factor.
    ///
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f32> for Vector {
    ///
    /// Divide each of the vector dimension by a given factor.
    ///
    /// This operation conserves the direction of the vector, but scale it by a factor.
    ///
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Default for Vector {
    ///
    /// Return the Zero vector.
    ///
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magn2_test() {
        let v = Vector::new(2., 3., 4.);
        let res = f32::powi(2., 2) + f32::powi(3., 2) + f32::powi(4., 2);

        assert_eq!(v.magn2(), res);
    }

    #[test]
    fn magn_test() {
        let v = Vector::new(2., 3., 4.);
        let res = f32::sqrt(f32::powi(2., 2) + f32::powi(3., 2) + f32::powi(4., 2));

        assert_eq!(v.magn(), res);
    }

    #[test]
    fn zero_is_id_test() {
        let v = Vector::new(2., 3., 4.);
        let z = Vector::ZERO;

        assert_eq!(v + z, v);
        assert_eq!(v - z, v);
    }

    #[test]
    fn normalize_leave_unit_untouched_test() {
        let x = Vector::X;
        let y = Vector::Y;
        let z = Vector::Z;

        assert_eq!(x.normalize(), Vector::X);
        assert_eq!(y.normalize(), Vector::Y);
        assert_eq!(z.normalize(), Vector::Z);
    }

    #[test]
    fn normalize_test() {
        let v = Vector::new(2., 3., 4.);
        let magn = v.magn();
        let res = Vector::new(2. / magn, 3. / magn, 4. / magn);

        assert_eq!(v.normalize(), res);
    }

    #[test]
    fn cross_test() {
        let x = Vector::X;
        let y = Vector::Y;
        let z = Vector::Z;

        assert_eq!(x.cross(y), z);
        assert_eq!(y.cross(x), -z);

        assert_eq!(x.cross(z), -y);
        assert_eq!(z.cross(x), y);

        assert_eq!(y.cross(z), x);
        assert_eq!(z.cross(y), -x);
    }

    #[test]
    fn add_test() {
        let x = Vector::new(1., 2., 3.);
        let y = Vector::new(4., 5., 6.);
        let res = Vector::new(5., 7., 9.);

        assert_eq!(x + y, res);
        assert_eq!(y + x, res);
    }

    #[test]
    fn add_assign_test() {
        let x = Vector::new(1., 2., 3.);
        let mut x2 = x;
        let y = Vector::new(4., 5., 6.);
        let mut y2 = y;
        let res = Vector::new(5., 7., 9.);

        x2 += y;
        y2 += x;
        assert_eq!(x2, res);
        assert_eq!(y2, res);
    }

    #[test]
    fn sub_test() {
        let x = Vector::new(1., 2., 3.);
        let y = Vector::new(4., 5., 6.);

        assert_eq!(x - y, x + (-y));
        assert_eq!(y - x, y + (-x));
    }

    #[test]
    fn sub_assign_test() {
        let x = Vector::new(1., 2., 3.);
        let mut x2 = x;
        let y = Vector::new(4., 5., 6.);
        let mut y2 = y;

        x2 -= y;
        y2 -= x;

        assert_eq!(x2, x + (-y));
        assert_eq!(y2, y + (-x));
    }

    #[test]
    fn mul_test() {
        let x = Vector::new(2., 4., 8.);
        let res = Vector::new(4., 8., 16.);

        assert_eq!(2. * x, res);
        assert_eq!(x * 2., res);
    }

    #[test]
    fn mul_assign_test() {
        let mut x = Vector::new(2., 4., 8.);
        let res = Vector::new(4., 8., 16.);

        x *= 2.;
        assert_eq!(x, res);
    }

    #[test]
    fn div_test() {
        let x = Vector::new(2., 4., 8.);
        let res = Vector::new(1., 2., 4.);

        assert_eq!(x / 2., res);
    }

    #[test]
    fn div_assign_test() {
        let mut x = Vector::new(2., 4., 8.);
        let res = Vector::new(1., 2., 4.);

        x /= 2.;
        assert_eq!(x, res);
    }

    #[test]
    fn dot_test() {
        let x = Vector::new(2., 3., 4.);
        let y = Vector::new(3., 4., 5.);
        let dot = 2. * 3. + 3. * 4. + 4. * 5.;

        assert_eq!(x.dot(y), dot);
        assert_eq!(y.dot(x), dot);
    }

    #[test]
    fn any_orthonormal_pair_test() {
        let v = Vector::new(1., 2., 3.).normalize();
        let (v2, v3) = v.any_orthonormal_pair();

        let threshold = 1e-6;

        let angle_v_v2 = v.dot(v2).acos();
        let angle_v_v3 = v.dot(v3).acos();
        let angle_v2_v3 = v2.dot(v3).acos();

        let deviation_v_v2 = f32::abs(angle_v_v2 - f32::to_radians(90.));
        let deviation_v_v3 = f32::abs(angle_v_v3 - f32::to_radians(90.));
        let deviation_v2_v3 = f32::abs(angle_v2_v3 - f32::to_radians(90.));

        assert!(
            deviation_v_v2 <= threshold,
            "angle v - v2: Expected {}, got {}",
            f32::to_radians(90.),
            angle_v_v2
        );
        assert!(
            deviation_v_v3 <= threshold,
            "angle v - v3: Expected {}, got {}",
            f32::to_radians(90.),
            angle_v_v3
        );
        assert!(
            deviation_v2_v3 <= threshold,
            "angle v2 - v3: Expected {}, got {}",
            f32::to_radians(90.),
            angle_v2_v3
        );

        assert!(v2.is_normalized(), "v2 is not a unit vector");
        assert!(v3.is_normalized(), "v3 is not a unit vector");
    }

    #[test]
    fn any_orthonormal_test() {
        let v = Vector::new(1., 2., 3.).normalize();
        let v2 = v.any_orthonormal();

        let angle = v.dot(v2).acos();
        let threshold = 1e-6;
        let deviation = f32::abs(angle - f32::to_radians(90.));

        assert!(
            deviation <= threshold,
            "Expected {}, got {}",
            f32::to_radians(90.),
            angle
        );
        assert!(v2.is_normalized(), "v2 is not a unit vector");
    }
}
