//!
//! Basic 3D sphere.
//!

use crate::{
    maths::{Point, Polynom2},
    render::{Intersection, Ray},
};

use super::Object;

///
/// Simple Sphere.
///
#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pos: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(pos: Point, radius: f32) -> Self {
        Self { pos, radius }
    }

    /// Compute the closes intersection between a Ray and a Sphere.
    ///
    /// To compute said intersection, we first compute a 2nd degree polynomial.
    /// Said polynomial can be found by injecting the parametric equation of a line into
    /// the equation of a 3d sphere :
    /// ```text
    /// X = rpos.x + k * rdir.x
    /// Y = rpos.y + k * rdir.y
    /// Z = rpos.z + k * rdir.z
    ///
    /// R^2 = (X - spos.x)^2 + (Y - spos.y)^2 + (Z - spos.z)^2
    /// ```
    /// with :
    /// - `rpos` as the ray origin.
    /// - `rdir` as the ray unit vector
    /// - `k` the distance between a point and `rpos`
    /// - `spos` as the sphere position.
    /// - `R` as the sphere radius.
    ///
    /// After factorization, we find this 2nd degree polynomial:
    /// ```
    /// adj = rpos - spos
    ///
    /// a = ||rdir||^2;
    /// b = 2. * (adj).dot(rdir);
    /// c = ||adj||^2 - R^2
    /// ```
    ///
    /// We can further simplify `a` because the ray direction should be a unit vector, so it's
    /// magnitude is always 1.
    ///
    /// We then find the smallest positive root of said polynomial, which is the closest
    /// intersection from the origin of the ray.
    ///
    /// From this, we can compute the exact intersection, as well as the surface normal at this
    /// point.
    ///
    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let adj = ray.orig - self.pos;

        let a = 1.;
        let b = 2. * adj.dot(ray.dir);
        let c = adj.magn2() - self.radius.powi(2);

        let roots = Polynom2::new(a, b, c).roots();

        roots
            .and_then(|x| x.into_iter().find(|r| r.is_sign_positive()))
            .map(|d| {
                let pos = ray.orig + ray.dir * d;
                let normal = (pos - self.pos).normalize();

                Intersection::new(d, pos, normal)
            })
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        self.intersect(ray)
    }

    fn cloned(&self) -> Box<dyn Object> {
        Box::new(*self) as Box<dyn Object>
    }
}
