//!
//! 3D infinite cylinder.
//!

use crate::{
    maths::{Point, Polynom2, Vector},
    render::{Intersection, Ray},
};

use super::Object;

///
/// Infinite Cylinder
///
#[derive(Debug, Clone, Copy)]
pub struct Cylinder {
    pos: Point,
    dir: Vector,
    radius: f32,
}

impl Cylinder {
    pub fn new(pos: Point, dir: Vector, radius: f32) -> Self {
        Self {
            pos,
            dir: dir.normalize(),
            radius,
        }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let adj = ray.orig - self.pos;

        let a = 1.0 - self.dir.dot(ray.dir).powi(2);
        let b = 2.0 * adj.dot(ray.dir) - ray.dir.dot(self.dir) * adj.dot(self.dir);
        let c = adj.magn2() - adj.dot(self.dir).powi(2) - self.radius.powi(2);

        let inter = Polynom2::new(a, b, c)
            .roots()
            .and_then(|x| x.into_iter().find(|r| r.is_sign_positive()));

        if let Some(dist) = inter {
            let point = ray.orig + ray.dir * dist;
            let projection = self.pos + self.dir * (point - self.pos).dot(self.dir);

            Some(Intersection::new(
                dist,
                point,
                (point - projection).normalize(),
            ))
        } else {
            None
        }
    }
}

impl Object for Cylinder {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        self.intersect(ray)
    }

    fn cloned(&self) -> Box<dyn Object> {
        Box::new(*self) as Box<dyn Object>
    }
}
