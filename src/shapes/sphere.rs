use crate::{
    math::Polynomial2,
    primitives::{ Point, Ray },
    shapes::{ Intersection, Shape },
};

pub struct Sphere {
    pub position: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new(position: Point, radius: f32) -> Self {
        Self { position, radius }
    }
}

impl Shape for Sphere {
    /// Compute the closes intersection between a Ray and a Sphere.
    ///
    /// To compute said intersection, we first compute a 2nd degree polynomial.
    /// Said polynomial can be found by injecting the parametric equation of a line into
    /// the equation of a 3d sphere :
    /// X = lx + k * lvx
    /// Y = ly + k * lvy
    /// Z = lz + k * lvz
    ///
    /// R^2 = (X - sx)^2 + (Y - sy)^2 + (Z - sz)^2
    ///
    /// We then find the smallest positive root of said polynomial, which is the closest
    /// intersection from the origin of the ray.
    ///
    /// From this, we can compute the exact intersection, as well as the surface normal at this
    /// point.
    ///
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut poly : Polynomial2<f32> = Default::default();
        let adj = ray.orig - self.position;

        poly.a = ray.dir.x.powi(2) + ray.dir.y.powi(2) + ray.dir.z.powi(2);
        poly.b = 2. * (adj.x * ray.dir.x + adj.y * ray.dir.y + adj.z * ray.dir.z);
        poly.c = adj.x.powi(2) + adj.y.powi(2) + adj.z.powi(2) - self.radius.powi(2);

        poly.root()
            .and_then(|x| x.into_iter().find(|arr| arr.is_sign_positive()))
            .map(|dist| Intersection {
                distance: dist,
                position: ray.orig + ray.dir * dist,
                normal: (ray.orig + ray.dir * dist) - self.position
            })
    }
}
