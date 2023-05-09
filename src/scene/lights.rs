//!
//! Lights module.
//!
//! Lights provide illumination (and thus shadows) to the scene.
//!

pub mod directional;
pub use directional::Directional;

///
/// Primary trait for lights.
///
pub trait Light {
    fn cloned(&self) -> Box<dyn Light>;
}

impl Clone for Box<dyn Light> {
    fn clone(&self) -> Self {
        self.cloned()
    }
}
