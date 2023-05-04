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

///
/// Quaternion data-structure.
///
/// See [module documentation](self) for more informations.
///
pub struct Quaternion {
    /// Real part
    pub r: f32,
    /// First order imaginary
    pub i: f32,
    /// Second order imaginary
    pub j: f32,
    /// Third order imaginary
    pub k: f32
}
