#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl Color {
    /// The black color
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    /// The white color
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
            a: 255,
        }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let a: f32 = a as f32 / 100.0 * 255.0;
        Self {
            r: r,
            g: g,
            b: b,
            a: a.max(0.0).min(255.0) as u8,
        }
    }

    pub fn hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches("#");
        if let Some(rgb) = convert_hex(&hex) {
            return Some(Self {
                r: rgb.0,
                g: rgb.1,
                b: rgb.2,
                a: 255,
            });
        } else {
            return None;
        }
    }
}

fn convert_hex(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    return Some((r, g, b));
}
