//!
//! 3D objects
//!
//! [Objects](Object) are rendering primitives.
//!

use crate::render::{Intersection, Ray};

pub mod cone;
pub mod cylinder;
pub mod plan;
pub mod sphere;

pub use cone::Cone;
pub use cylinder::Cylinder;
pub use plan::Plan;
pub use sphere::Sphere;

///
/// Trait representing 3D objects.
///
pub trait Object {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
    fn cloned(&self) -> Box<dyn Object>;
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.cloned()
    }
}
