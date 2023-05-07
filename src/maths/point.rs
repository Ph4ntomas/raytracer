//!
//! 3D Point
//!
//! The Point type represents a position in a 3-dimensional space. Because of this, few operations
//! exists on the `Point`.
//!
//! ```rust
//! use raytracer::maths::Point;
//!
//! let p1 = Point::new(4., 5., 6.);
//! let p2 = Point { x: 1., y: 2., z: 3. };
//!
//! let v = p1 - p2; // v is a vector encoding the distance between two points.
//! // p1 + p2 [ERROR] This makes no sense mathematically.
//!
//! let p3 = p2 + v; // Apply v on p2 to compute a new point.
//! let p3 = p2 - v; // Apply the inverse vector of v to compute a new point.
//! ```
//!

use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::vector::Vector;

///
/// Position in 3d space.
///
/// See [module documentation](self) for more informations.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    ///
    /// Create a new point.
    ///
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    ///
    /// Space origin.
    ///
    pub const ORIGIN: Point = Point::new(0., 0., 0.);
}

impl Sub for Point {
    type Output = Vector;

    ///
    /// Compute the distance between two point.
    ///
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    ///
    /// Compute a new point by applying a vector to it.
    ///
    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<Vector> for Point {
    ///
    /// Apply the vector, in place.
    ///
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    ///
    /// Compute a new point by applying the inverse of a vector to it.
    ///
    fn sub(self, rhs: Vector) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign<Vector> for Point {
    ///
    /// Apply the inverse vector, in place.
    ///
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0., 0., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_between_points_test() {
        let p1 = Point::new(4., 5., 6.);
        let p2 = Point::new(1., 2., 3.);
        let p3 = Vector::new(3., 3., 3.);

        assert_eq!(p1 - p2, p3);
    }

    #[test]
    fn add_vector_to_point_test() {
        let p = Point::new(1., 2., 3.);
        let v = Vector::new(1., 2., 3.);

        let res = Point::new(2., 4., 6.);

        assert_eq!(p + v, res);
    }

    #[test]
    fn add_assign_vector_to_point_test() {
        let p = Point::new(1., 2., 3.);
        let mut p2 = p;
        let v = Vector::new(1., 2., 3.);

        let res = Point::new(2., 4., 6.);

        p2 += v;
        assert_eq!(p2, res);
    }

    #[test]
    fn sub_vector_to_point_test() {
        let p = Point::new(2., 4., 6.);
        let v = Vector::new(1., 2., 3.);

        let res = Point::new(1., 2., 3.);

        assert_eq!(p - v, res);
    }

    #[test]
    fn sub_assign_vector_to_point_test() {
        let p = Point::new(2., 4., 6.);
        let mut p2 = p;
        let v = Vector::new(1., 2., 3.);

        let res = Point::new(1., 2., 3.);

        p2 -= v;
        assert_eq!(p2, res);
    }
}
