//!
//! Color module.
//!

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba([u8; 4]);

impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 0xFF)
    }

    pub const BLACK: Rgba = Rgba::from_rgb(0x0, 0x0, 0x0);
    pub const WHITE: Rgba = Rgba::from_rgb(0xFF, 0xFF, 0xFF);
    pub const RED: Rgba = Rgba::from_rgb(0xFF, 0x0, 0x0);
    pub const GREEN: Rgba = Rgba::from_rgb(0x0, 0xFF, 0x0);
    pub const BLUE: Rgba = Rgba::from_rgb(0x0, 0x0, 0xFF);

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn to_array(self) -> [u8; 4] {
        self.0
    }
}
