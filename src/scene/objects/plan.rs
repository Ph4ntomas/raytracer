//!
//! 3D infinite plan.
//!

use crate::{
    maths::{Point, Vector},
    render::{Intersection, Ray},
};

use super::Object;

///
/// Infinite plan.
///
#[derive(Clone, Copy, Debug)]
pub struct Plan {
    pos: Point,
    norm: Vector,
}

impl Plan {
    pub fn new(pos: Point, norm: Vector) -> Self {
        Self {
            pos,
            norm: norm.normalize(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        match ray.dir.dot(-self.norm) {
            denom if denom.abs() > 1e-7 => {
                let sign = denom.signum();
                let p = self.pos - ray.orig;
                let d = -(p.dot(sign * self.norm)) / denom.abs();

                if d > 1e-7 {
                    Some(Intersection::new(
                        d,
                        ray.orig + d * ray.dir,
                        sign * self.norm,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Object for Plan {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        self.intersect(ray)
    }

    fn cloned(&self) -> Box<dyn Object> {
        Box::new(*self) as Box<dyn Object>
    }
}
