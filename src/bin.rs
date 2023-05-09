use nannou::wgpu::Texture;
use raytracer::{
    render::{Camera, Renderer},
    scene::Scene,
};

pub struct Model {
    pub renderer: Renderer,
    pub camera: Camera,
    pub scene: Scene,
    pub texture: Option<Texture>,
}

impl Model {
    pub fn new(renderer: Renderer, camera: Camera, scene: Scene) -> Self {
        Self {
            renderer,
            camera,
            scene,

            texture: None,
        }
    }

    pub fn empty() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::empty()
    }
}
