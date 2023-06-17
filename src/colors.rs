pub type Color = [f32; 4];

/// Creates a color from RGB values.
#[inline]
pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
    [r, g, b, 1.0]
}

/// Creates a color from RGBA values.
#[inline]
pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    [r, g, b, a]
}

/// Adjusts the alpha value of a color.
#[inline]
pub const fn with_alpha(mut color: Color, alpha: f32) -> Color {
    color[3] = alpha;
    color
}

pub const BLACK: Color = rgb(0.0, 0.0, 0.0);

pub const WHITE: Color = rgb(1.0, 1.0, 1.0);

pub const RED: Color = rgb(1.0, 0.0, 0.0);

pub const GREEN: Color = rgb(0.0, 1.0, 0.0);

pub const BLUE: Color = rgb(0.0, 1.0, 0.0);

pub const YELLOW: Color = rgb(1.0, 1.0, 0.0);

pub const CYAN: Color = rgb(0.0, 1.0, 1.0);

pub const MAGENTA: Color = rgb(1.0, 0.0, 1.0);

pub const GREY: Color = rgb(0.5, 0.5, 0.5);

pub const FINE: Color = rgb(0.384, 0.643, 0.855);

pub const MASTERWORK: Color = rgb(0.102, 0.576, 0.024);

pub const RARE: Color = rgb(0.988, 0.816, 0.043);

pub const EXOTIC: Color = rgb(1.0, 0.643, 0.02);

pub const ASCENDED: Color = rgb(0.984, 0.243, 0.553);

pub const LEGENDARY: Color = rgb(0.592, 0.306, 1.0);
