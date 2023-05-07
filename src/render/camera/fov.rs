//!
//! Field of view related type.
//!
//! The field of view determine what part of a scene can be seen through a camera. Imagine your
//! screen as a window. The closer you get, the more stuff you can see. It works the same in a
//! raytracer. The [Fov] type is used to compute the distance between the virtual
//! [Camera](super::Camera) and its associated
//! [Sensor](super::sensor::Sensor).
//!

use std::fmt::Display;

///
/// Disambiguation type to represent a Fov.
///
/// This is use when computing the distance between a [Camera](crate::render::camera::Camera) and
/// the associated [Surface](crate::render::surface::Surface).
///
#[derive(Clone, Copy, Debug)]
pub enum Fov {
    Horizontal(f32),
    Vertical(f32),
}

impl Fov {
    ///
    /// Create a new Fov by specifying an horizontal angle.
    ///
    /// `degree` represent an angle, expressed in angular degree.
    ///
    pub const fn horiz(degree: f32) -> Self {
        Self::Horizontal(degree)
    }

    ///
    /// Create a new Fov by specifying a vertical angle.
    ///
    /// `degree` represent an angle, expressed in angular degree.
    ///
    pub const fn vert(degree: f32) -> Self {
        Self::Vertical(degree)
    }
}

impl Default for Fov {
    ///
    /// Default Fov has an horizontal viewing angle of 70 degree.
    ///
    fn default() -> Self {
        Self::Horizontal(70.)
    }
}

impl Display for Fov {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Horizontal(angle) => f.write_fmt(format_args!("{} H", angle)),
            Self::Vertical(angle) => f.write_fmt(format_args!("{} V", angle)),
        }
    }
}
