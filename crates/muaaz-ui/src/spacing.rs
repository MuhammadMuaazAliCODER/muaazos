//! The Muaaz spacing scale.
//!
//! Spacing values are multiples of a 4 px grid, which keeps layouts
//! horizontally and vertically consistent across the desktop.

/// The Muaaz spacing scale, in pixels, as a multiple of four.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spacing(pub u32);

impl Spacing {
    pub const XS: u32 = 4;
    pub const SM: u32 = 8;
    pub const MD: u32 = 12;
    pub const LG: u32 = 16;
    pub const XL: u32 = 24;
    pub const XXL: u32 = 32;
    pub const XXXL: u32 = 48;
    pub const HUGE: u32 = 64;
    pub const MASSIVE: u32 = 96;

    /// Returns the value in pixels.
    pub const fn px(self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_is_ordered() {
        assert!(Spacing::XS < Spacing::SM);
        assert!(Spacing::SM < Spacing::MD);
        assert!(Spacing::MD < Spacing::LG);
        assert!(Spacing::LG < Spacing::XL);
        assert!(Spacing::XL < Spacing::XXL);
        assert!(Spacing::XXL < Spacing::XXXL);
        assert!(Spacing::XXXL < Spacing::HUGE);
        assert!(Spacing::HUGE < Spacing::MASSIVE);
    }
}