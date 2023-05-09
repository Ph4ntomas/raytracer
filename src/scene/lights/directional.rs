//!
//! Directional [Light](super::Light).
//!
//! A directional light models an infinitely far away object. Any [Ray](crate::render::Ray) emitted
//! by this light will be parallels.
//!

use super::Light;

#[derive(Clone, Copy, Debug)]
pub struct Directional {}

impl Directional {}

impl Light for Directional {
    fn cloned(&self) -> Box<dyn Light> {
        Box::new(*self)
    }
}
