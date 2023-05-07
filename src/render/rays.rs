//!
//! Rays module
//!
//! [Rays](Ray) are the rendering primitive of raytracing. They are shot by a camera in a
//! [Scene](crate::scene::Scene), then intersect with object. Rays of light are also represented by
//! `Ray`.
//!
//! At their core, `Ray`s are defined as a [Point](crate::maths::Point) in space, and a unit
//! [Vector](crate::maths::Vector).
//!

use crate::maths::{Point, Vector};

///
/// Ray representation.
///
/// See [module documentation](self) for more informations.
///
pub struct Ray {
    pub orig: Point,
    pub dir: Vector,
}

impl Ray {
    ///
    /// Create a new `Ray` from a `Point` in space and a unit `Vector`
    ///
    /// # Panic
    /// Panics in debug if `dir` is not a unit vector.
    ///
    pub fn new(orig: Point, dir: Vector) -> Self {
        debug_assert!(dir.is_normalized(), "Ray::new: dir is not a unit vector.");
        Self { orig, dir }
    }
}
