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
                orig: point,
                dir: self.pos - point,
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
