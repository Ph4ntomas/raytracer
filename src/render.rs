//!
//! Raytracer rendering module.
//!
//! Raytracing rendering works by first finding an [Intersection] between a [Ray] coming from a
//! [Camera](camera::Camera) and [Objects](crate::scene::Object). Once an intersection is found,
//! `Ray`s are sent from [Light](crate::scene::Light) sources to create shadow, or bounced back
//! from the [Point](crate::maths::Point) of the `intersection` to create other effects such as
//! reflection or transparency, in a recursive fashion.
//!

pub mod camera;
pub mod intersection;
pub mod rays;

pub use camera::*;
pub use intersection::Intersection;
pub use rays::Ray;

use crate::{colors::Rgba, scene::Scene};

///
/// Renderer.
///
pub struct Renderer {
    background: Rgba,
    shaders: Vec<Box<dyn Shader>>,
}

impl Renderer {
    pub fn new(background: Rgba) -> Self {
        let fast = Box::new(shader::Fast) as Box<dyn Shader>;
        let gamma_enc = Box::<shader::GammaEncoder>::default() as Box<dyn Shader>;

        Self {
            background,
            shaders: vec![fast, gamma_enc],
        }
    }

    pub fn render_pixel(&self, x: u32, y: u32, camera: &Camera, scene: &Scene) -> Rgba {
        let Some(ray) = camera.get_ray(x, y) else {
            eprintln!("WARN: trying to render outside of the camera sensor at ({}, {})", x, y);
            return self.background
        };

        let inters = scene
            .objects
            .iter()
            .filter_map(|o| o.intersect(ray).zip(Some(o)));

        let closest = inters.min_by(|(i, _), (i2, _)| i.dist.total_cmp(&i2.dist));

        closest
            .map(|(inter, obj)| {
                self.shaders.iter().fold(Rgba::WHITE, |col, s| {
                    s.compute_color(ray, obj.as_ref(), inter, col, scene)
                })
            })
            .unwrap_or(self.background)
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new(Rgba::BLACK)
    }
}

pub mod shader {
    use super::*;
    use crate::scene::Object;

    pub trait Shader {
        fn compute_color(
            &self,
            ray: Ray,
            object: &dyn Object,
            intersection: Intersection,
            color: Rgba,
            scene: &Scene,
        ) -> Rgba;
    }

    pub struct Fast;

    impl Shader for Fast {
        fn compute_color(
            &self,
            ray: Ray,
            _object: &dyn Object,
            intersection: Intersection,
            color: Rgba,
            _scene: &Scene,
        ) -> Rgba {
            let intensity = ray.dir.dot(-intersection.normal);
            let [r, g, b, _] = color.to_array();

            let r = ((r as f32) * intensity) as u8;
            let g = ((g as f32) * intensity) as u8;
            let b = ((b as f32) * intensity) as u8;

            Rgba::from_rgb(r, g, b)
        }
    }

    pub struct GammaEncoder {
        factor: f32,
    }

    impl GammaEncoder {
        pub fn new(factor: f32) -> Self {
            Self { factor }
        }

        pub fn encode(&self, color: Rgba) -> Rgba {
            let [r, g, b, a] = color.to_array();

            let mut f_r = (r as f32) / 255.;
            let mut f_g = (g as f32) / 255.;
            let mut f_b = (b as f32) / 255.;

            f_r = f_r.powf(self.factor.recip());
            f_g = f_g.powf(self.factor.recip());
            f_b = f_b.powf(self.factor.recip());

            let r = (f_r * 255.) as u8;
            let g = (f_g * 255.) as u8;
            let b = (f_b * 255.) as u8;

            Rgba::new(r, g, b, a)
        }
    }

    impl Shader for GammaEncoder {
        fn compute_color(
            &self,
            _ray: Ray,
            _object: &dyn Object,
            _intersection: Intersection,
            color: Rgba,
            _scene: &Scene,
        ) -> Rgba {
            self.encode(color)
        }
    }

    impl Default for GammaEncoder {
        fn default() -> Self {
            Self::new(2.2)
        }
    }
}

pub use shader::Shader;
