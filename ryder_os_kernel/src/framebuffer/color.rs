#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGBColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RGBColor {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn blend(&self, other: Self, intensity: u8) -> Self {
        let alpha = u16::from(intensity);
        let inverse_alpha = 255 - alpha;

        let blended_red = (u16::from(self.red) * alpha + u16::from(other.red) * inverse_alpha) / 255;
        let blended_green = (u16::from(self.green) * alpha + u16::from(other.green) * inverse_alpha) / 255;
        let blended_blue = (u16::from(self.blue) * alpha + u16::from(other.blue) * inverse_alpha) / 255;

        Self::new(
            blended_red as u8,
            blended_green as u8,
            blended_blue as u8,
        )
    }

    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const WHITE: Self = Self::new(255, 255, 255);
    pub const RED: Self = Self::new(255, 0, 0);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);
}

impl Default for RGBColor {
    fn default() -> Self {
        Self::BLACK
    }
}