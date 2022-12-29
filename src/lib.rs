pub mod primitives {
    mod point;
    mod vector;
    mod quaternion;

    pub use point::Point;
    pub use vector::Vector;
    pub use quaternion::Quaternion;

    pub struct Ray {
        pub orig: Point,
        pub dir: Vector
    }
}

mod camera;
mod image;
mod renderer;

pub use camera::Camera;
pub use image::Image;
pub use renderer::Renderer;

#[derive(Debug, Copy, Clone)]
pub struct Dimension {
    pub x: u32,
    pub y: u32
}

pub mod lights {
    pub trait Light {}

    pub struct Point {
        pub position: super::primitives::Point
    }
}

pub mod shapes {
    use crate::primitives::{Point, Ray, Vector};

    pub struct Intersection {
        pub distance: f32,
        pub position: Point,
        pub normal: Vector
    }

    pub trait Shape {
        /// Compute closest intersection between a Ray and a shape.
        ///
        /// Returns an Intersection containing the distance, the actual Point and a Vector
        /// orthogonal to the hit surface.
        fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    }

    mod sphere;
    pub use sphere::Sphere;


}

pub mod math {
    mod polynome2;
    pub use polynome2::Polynome2;
}
