//!
//! Scene module
//!
//! The [Scene] is what gets rendered by the raytracer. It represents 3D space, and contains
//! [objects] and [lights].
//!

pub mod lights;
pub mod objects;

pub use lights::Light;
pub use objects::Object;

#[allow(dead_code)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
    pub ambiant: f32,
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Object>>, lights: Vec<Box<dyn Light>>, ambiant: f32) -> Self {
        Self {
            objects,
            lights,
            ambiant,
        }
    }

    pub fn empty() -> Self {
        Self::new(vec![], vec![], 0.)
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::empty()
    }
}

pub struct SceneBuilder {
    objects: Vec<Box<dyn Object>>,
    lights: Vec<Box<dyn Light>>,
    ambiant: Option<f32>,
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        Self {
            objects: vec![],
            lights: vec![],
            ambiant: None,
        }
    }

    pub fn with_ambiant(&mut self, ambiant: f32) -> &mut Self {
        self.ambiant = Some(ambiant.clamp(0., 1.));
        self
    }

    pub fn with_object(&mut self, obj: impl Object + 'static) -> &mut Self {
        let boxed = Box::new(obj) as Box<dyn Object>;

        self.objects.push(boxed);
        self
    }

    pub fn with_light(&mut self, obj: impl Light + 'static) -> &mut Self {
        let boxed = Box::new(obj) as Box<dyn Light>;

        self.lights.push(boxed);
        self
    }

    pub fn build(&self) -> Scene {
        Scene::new(
            self.objects.clone(),
            self.lights.clone(),
            self.ambiant.unwrap_or(0.3),
        )
    }
}

pub fn scene() -> SceneBuilder {
    SceneBuilder::new()
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self::new()
    }
}
