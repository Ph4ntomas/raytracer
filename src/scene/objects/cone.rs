//!
//! 3D Cone
//!

use crate::{
    maths::{Point, Polynom2, Vector},
    render::{Intersection, Ray},
};

use super::Object;

///
/// Infinite Cone
///
#[derive(Debug, Clone, Copy)]
pub struct Cone {
    pos: Point,
    dir: Vector,
    angle: f32,
}

impl Cone {
    pub fn new(pos: Point, dir: Vector, angle: f32) -> Self {
        Self {
            pos,
            dir: dir.normalize(),
            angle: angle.to_radians(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let adj = ray.orig - self.pos;

        let tan2 = self.angle.tan().powi(2);

        let a = 1.0 - tan2 - ray.dir.dot(self.dir).powi(2);
        let b = 2.0 * (adj.dot(ray.dir) * (1.0 - tan2) - self.dir.dot(ray.dir) * self.dir.dot(adj));
        let c = adj.magn2() * (1. - tan2) - self.dir.dot(adj).powi(2);

        let inter = Polynom2::new(a, b, c)
            .roots()
            .filter(|[_, r2]| !r2.is_nan())
            .and_then(|x| x.into_iter().find(|r| r.is_sign_positive()));

        if let Some(dist) = inter {
            let point = ray.orig + ray.dir * dist;
            let v = point - self.pos;

            let projection = self.pos + self.dir * (v.magn2() / self.dir.dot(v));

            Some(Intersection::new(dist, point, point - projection))
        } else {
            None
        }
    }
}

impl Object for Cone {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        self.intersect(ray)
    }

    fn cloned(&self) -> Box<dyn Object> {
        Box::new(*self) as Box<dyn Object>
    }
}
