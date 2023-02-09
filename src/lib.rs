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


pub mod materials {
    #[derive(Debug, Copy, Clone)]
    pub struct Material {
        pub color: u32,

        pub refraction_amount: f32,
        pub refraction_indices: f32,

        pub reflection_amount: f32,
    }

    impl Default for Material {
        fn default() -> Self {
            Self {
                color: 0xffffff,

                refraction_amount: 0_f32,
                refraction_indices: 0_f32,

                reflection_amount: 0_f32
            }
        }
    }
}

pub use materials::Material;

pub mod shapes {
    use crate::primitives::{Point, Ray, Vector};
    use crate::Material;

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
        fn get_material(&self, inter: &Intersection) -> Material;
    }

    mod sphere;
    pub use sphere::Sphere;
}

pub mod math {
    mod polynomial2;
    pub use polynomial2::Polynomial2;
}
