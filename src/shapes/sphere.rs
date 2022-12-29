use crate::{
    math::Polynome2,
    primitives::{ Point, Ray },
    shapes::{ Intersection, Shape },
};

pub struct Sphere {
    pub position: Point,
    pub radius: f32,
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut poly : Polynome2<f32> = Default::default();
        let adj = ray.orig - self.position;

        poly.a = ray.dir.x.powi(2) + ray.dir.y.powi(2) + ray.dir.z.powi(2);
        poly.b = 2. * (adj.x * ray.dir.x + adj.y * ray.dir.y + adj.z * ray.dir.z);
        poly.c = adj.x.powi(2) + adj.y.powi(2) + adj.z.powi(2) - self.radius.powi(2);

        poly.root()
            .and_then(|x| x.into_iter().filter(|arr| arr.is_sign_positive()).nth(0))
            .map(|dist| Intersection {
                distance: dist,
                position: ray.orig + ray.dir * dist,
                normal: (ray.orig + ray.dir * dist) - self.position
            })
    }
}
