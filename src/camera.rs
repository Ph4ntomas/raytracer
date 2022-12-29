use crate::{
    Dimension,
    primitives::{Point, Vector, Quaternion, Ray}
};

#[derive(Debug)]
pub struct Camera {
    pub position: Point,
    pub direction: Vector,
    pub fov: f32,
    pub distance: f32,
        rotation: Quaternion
}

impl Camera {
    pub fn new(position: Point, direction: Vector, fov: f32, distance: f32) -> Self {
        let ref_vector = Vector::new(0., 0., 1.);
        let angle = direction.angle(&ref_vector);
        let axis = direction.cross(&ref_vector);

        Self {
            position, direction, fov, distance,
            rotation: Quaternion::new_rotation(angle, axis)
        }
    }

    fn compute_angle(fov: f32, max_dist: u32, pos: u32) -> f32 {
        (fov.to_radians() / max_dist as f32) * (pos as f32 - max_dist as f32 / 2.)
    }

    pub fn get_ray(&self, dimension: Dimension, x: u32, y: u32) -> Ray {
        let xangle = Self::compute_angle(self.fov, dimension.x, x);
        let yangle = Self::compute_angle(self.fov, dimension.y, y);

        let dirvec = Vector {
            x: self.distance * f32::tan(xangle),
            y: self.distance * f32::tan(yangle),
            z: self.distance
        };

        Ray {
            orig: self.position,
            dir: self.rotation.rotate_vector(dirvec)
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Point::zero(),
            direction: Vector::new(0., 0., 1.),
            fov: 80.,
            distance: 100.,
            rotation: Quaternion { a: 1., b: 0., c: 0., d: 0. }
        }
    }
}
