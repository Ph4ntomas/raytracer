//!
//! Virtual Camera.
//!
//! A [Camera] shoots [Rays](crate::render::rays::Rays) at objects of a
//! [Scene](crate::scene::Scene), computing intersection with the `Scene`
//! [objects](crate::scene::objects), and how [lights](crate::scene::lights) would affect said
//! objects.
//!

use std::fmt::{self, Display};

use crate::maths::{Point, Quaternion, Vector};
use crate::render::Ray;

pub mod fov;
pub mod sensor;

pub use fov::Fov;
pub use sensor::Sensor;

///
/// Raytracer `Camera`s.
///
/// See [module documentation](self) for more informations.
///
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    /// Spatial position of the camera.
    pos: Point,
    /// Looking direction of the camera.
    dir: Vector,
    /// Render screen.
    sensor: Sensor,
    /// Camera field of view.
    fov: Fov,
    /// Rotation around the `dir` axis.
    tilt: f32,

    /// Distance from the camera to the screen.
    dist: f32,
    /// Rotation to apply to each rays leaving the camera.
    base_rot: Quaternion,
}

impl Camera {
    ///
    /// Build a new camera.
    ///
    /// `pos` is the position of the camera in the scene.
    /// `dir` is the direction the camera is looking in.
    /// `sensor` Camera's [Sensor].
    /// `fov` the distance between the camera and the screen is computed from the camera Field of
    /// view.
    /// `tilt` an arbirary rotation, using the `dir` vector as an axis.
    ///
    pub fn new(pos: Point, dir: Vector, sensor: Sensor, fov: Fov, tilt: f32) -> Self {
        let dir = dir.normalize();
        let tilt = tilt.to_radians();

        let dist = sensor.distance(fov);

        let mut base_rot = Quaternion::from_arc(Vector::Z, dir);
        base_rot *= Quaternion::from_z_rot(tilt.to_radians());

        Camera {
            pos,
            dir,
            sensor,
            fov,
            tilt,
            dist,
            base_rot,
        }
    }

    ///
    /// Returns the [Ray](crate::render::rays::Ray) that would hit a given pixel of the `Camera`'s
    /// `Sensor`.
    ///
    pub fn get_ray(&self, x: i32, y: i32) -> Option<Ray> {
        self.sensor.pixel_pos_to_render_pos(x, y).map(|(x, y)| {
            let dir = Vector::new(x, y, 0.) + self.dist * Vector::Z;
            Ray::new(self.pos, (self.base_rot * dir).normalize())
        })
    }
}

impl Default for Camera {
    ///
    /// Default `Camera`.
    ///
    /// This `Camera` has the default [Sensor], which has a resolution of 512x512 pixels.
    /// It looks toward the Z-axis, and doesn't have any tilt.
    ///
    /// Its [Fov] is the default one, which means the Camera has an horizontal fov of 70 degrees.
    ///
    fn default() -> Self {
        Camera::new(
            Point::ORIGIN,
            Vector::Z,
            Default::default(),
            Default::default(),
            0.,
        )
    }
}

impl Display for Camera {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "Camera {{
            pos: {},
            dir: {},
            sensor: {},
            fov: {},
            tilt: {}
        }}",
            self.pos, self.dir, self.sensor, self.fov, self.tilt
        ))
    }
}
