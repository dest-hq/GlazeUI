#[derive(Debug)]
pub struct Padding {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Padding {
    pub fn all(padding: f32) -> Self {
        Padding {
            top: padding,
            left: padding,
            right: padding,
            bottom: padding,
        }
    }

    pub fn new() -> Self {
        Padding {
            top: 0.0,
            left: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }

    pub fn left(mut self, padding: f32) -> Self {
        self.left = padding;
        self
    }

    pub fn right(mut self, padding: f32) -> Self {
        self.right = padding;
        self
    }

    pub fn bottom(mut self, padding: f32) -> Self {
        self.bottom = padding;
        self
    }

    pub fn top(mut self, padding: f32) -> Self {
        self.bottom = padding;
        self
    }
}

#[derive(Debug, Clone)]
pub enum Align {
    Top,
    TopRight,
    TopLeft,
    Center,
    CenterRight,
    CenterLeft,
    Bottom,
    BottomRight,
    BottomLeft,
}

#[derive(Debug)]
pub enum Length {
    Fill, // Will be better in future to change the fill all space to remaining space
    FillWidth,
    FillHeight,
    FillPercent(u16),
    Fixed(f32, f32),
}

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
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
            a: a as u8,
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
