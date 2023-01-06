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

pub mod lights;

pub mod shapes {
    use crate::primitives::{Point, Ray, Vector};

    #[derive(Debug, Clone)]
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

pub mod transforms {
    use crate::{shapes::{Intersection, Shape}, Renderer};

    pub struct Transform {
        pub surface_color: u32,
        pub color_ratio: f32,
        pub inter: Intersection
    }

    impl Transform {
        pub fn init(inter: Intersection) -> Transform {
            Transform {
                surface_color: 0xff,
                color_ratio: 0.2_f32,
                inter
            }
        }
    }

    trait Transformer {
        fn apply_transform(&self, input: Transform, shape: &dyn Shape, renderer: &Renderer) -> Transform;
    }

    struct ColorTransformer {
        pub color: u32
    }

    impl Transformer for ColorTransformer {
        fn apply_transform(&self, input: Transform, _shape: &dyn Shape, _renderer: &Renderer) -> Transform {
            Transform {
                surface_color: self.color,
                ..input
            }
        }
    }

    struct ColorRatioTransformer {
        pub color_ratio: f32
    }

    //impl Transformer for ColorRatioTransformer {
        //fn apply_transform(&self, input: Transform, _shape: &dyn Shape, _renderer: &Renderer) -> Transform {

        //}
    //}
}

pub mod math {
    mod polynomial2;
    pub use polynomial2::Polynomial2;
}
