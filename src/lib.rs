pub mod primitives {
    mod point;
    mod vector;
    mod quaternion;

    pub use point::Point;
    pub use vector::Vector;
    pub use quaternion::Quaternion;

    #[derive(Debug)]
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
    use crate::primitives::{Point, Ray};

    #[derive(Debug)]
    pub struct LightRay {
        pub ray: Ray,
        pub intensity: f32,
        pub color: u32
    }

    pub trait Light {
        fn get_ray(&self, point: Point) -> LightRay;
    }

    pub struct DotLight {
        pub pos: Point,
        pub intensity: f32,
        pub color: u32,
    }

    impl Light for DotLight {
        fn get_ray(&self, point: Point) -> LightRay {
            LightRay {
                ray: Ray {
                    orig: self.pos,
                    dir: point - self.pos,
                },
                intensity: self.intensity,
                color: self.color
            }
        }
    }

    impl Default for DotLight {
        fn default() -> Self {
            DotLight {
                pos: Point::zero(),
                intensity: 1.,
                color: 0xffffffff,
            }
        }
    }
}

pub mod shapes {
    use crate::primitives::{Point, Ray, Vector};

    #[derive(Debug)]
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
    mod polynomial2;
    pub use polynomial2::Polynomial2;
}
