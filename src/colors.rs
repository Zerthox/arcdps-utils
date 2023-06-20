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

/// Linearly interpolates between two [`f32`]s.
#[inline]
pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}

/// Linearly interpolates between two colors.
#[inline]
pub fn lerp(a: Color, b: Color, t: f32) -> Color {
    [
        lerp_f32(a[0], b[0], t),
        lerp_f32(a[1], b[1], t),
        lerp_f32(a[2], b[2], t),
        lerp_f32(a[3], b[3], t),
    ]
}

pub const TRANSPARENT: Color = rgba(0.0, 0.0, 0.0, 0.0);

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
