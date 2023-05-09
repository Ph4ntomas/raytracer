//!
//! Intersection between a [Ray](super::Ray) and an [Object](crate::scene::Object)
//!

use crate::maths::{Point, Vector};

///
/// Intersection between a [Ray](super::Ray) and an [Object](crate::scene::Object)
///
#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    pub dist: f32,
    pub pos: Point,
    pub normal: Vector,
}

impl Intersection {
    pub fn new(dist: f32, pos: Point, normal: Vector) -> Self {
        Self { dist, pos, normal }
    }
}
