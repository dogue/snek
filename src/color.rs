#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub const RED: Color = Color(255, 0, 0);
    pub const GREEN: Color = Color(0, 255, 0);
    pub const BLUE: Color = Color(0, 0, 255);
    pub const MAGENTA: Color = Color(255, 0, 255);
    pub const YELLOW: Color = Color(255, 255, 0);
    pub const CYAN: Color = Color(0, 255, 255);
    pub const WHITE: Color = Color(255, 255, 255);
    pub const BLACK: Color = Color(0, 0, 0);

    // Personal preference for a BG Color
    // DARK ASS GRAY
    pub const BLANK: Color = Color(18, 18, 18);
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        let (r, g, b) = (self.0 as u32, self.1 as u32, self.2 as u32);
        (r << 16) | (g << 8) | b
    }
}
