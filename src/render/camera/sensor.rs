//!
//! [Camera](super::Camera)'s Sensor.
//!
//! The `Camera`'s `Sensor` is a rectangle that define the amount of pixel the camera will send
//! [Rays](crate::render::rays::Ray) to.
//!

use std::fmt::Display;

use crate::render::camera::Fov;

///
/// [Camera](super::Camera)'s Sensor.
///
/// The `Sensor` is just defined as two positive integer `width` and `height`.
/// The distance to the `Camera` origin is computed from the `Camera` [Fov](super::Fov).
///
/// See [module documentation](self) or [camera's module documentation](super) for more
/// informations.
///
#[derive(Clone, Copy, Debug)]
pub struct Sensor {
    width: u32,
    height: u32,
}

impl Sensor {
    ///
    /// Create a new `Sensor`
    ///
    /// # Panics:
    /// Panics if `width` or `Height` are not strictly positives integers.
    ///
    pub fn new(width: u32, height: u32) -> Self {
        assert!(width > 0, "Sensor::new: width must be a positive integer.");
        assert!(
            height > 0,
            "Sensor::new: height must be a positive integer."
        );

        Self { width, height }
    }

    ///
    /// Compute the distance between a `Camera` and its sensor, given the `Camera`'s `Fov`.
    ///
    pub fn distance(self, fov: Fov) -> f32 {
        match fov {
            Fov::Horizontal(angle) => (self.width / 2) as f32 / angle.to_radians().tan(),
            Fov::Vertical(angle) => (self.height / 2) as f32 / angle.to_radians().tan(),
        }
    }

    ///
    /// Check whether the pixel `(x, y)` is in bounds.
    ///
    pub fn has_pixel(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    ///
    /// Convert pixel to render position.
    ///
    /// This function remove half the screen width from the `x` and `y` coordinates, then flip the
    /// `y` coordinate, because this library use NDC space.
    ///
    pub fn pixel_pos_to_render_pos(&self, x: u32, y: u32) -> Option<(f32, f32)> {
        if !self.has_pixel(x, y) {
            None
        } else {
            let half_w = (self.width / 2) as i32;
            let half_h = (self.height / 2) as i32;

            let x = ((x as i32) - half_w) as f32;
            let y = (-((y as i32) - half_h)) as f32;

            Some((x, y))
        }
    }
}

impl Default for Sensor {
    fn default() -> Self {
        Sensor::new(512, 512)
    }
}

impl Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{ width: {}, height: {} }}",
            self.width, self.height
        ))
    }
}
