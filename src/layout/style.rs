// All styles
#[derive(Debug, Clone)]
pub struct Style {
    pub size: Size,
    pub length: Option<Length>,
    pub padding: Option<PaddingOptions>,
    pub vertical_align: Option<VerticalAlign>,
    pub horizontal_align: Option<HorizontalAlign>,
    pub spacing: Option<SpacingOptions>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            size: Size {
                width: SizeOptions::Fixed(0),
                height: SizeOptions::Fixed(0),
            },
            length: None,
            padding: None,
            vertical_align: None,
            horizontal_align: None,
            spacing: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: SizeOptions,
    pub height: SizeOptions,
}

#[derive(Debug, Clone)]
pub enum SizeOptions {
    Fixed(u32),
    ParentSize,
    Percent(f32), // Range from 0.0 to 1.0
}

#[derive(Debug, Clone)]
pub enum Length {
    Fill,             // Fill all the remaining space
    FillPortion(u16), // Fill a portion of remaining space
    Fixed(u32, u32),  // Fill fixed amount of space
}

#[derive(Debug, Clone)]
pub struct SpacingOptions {
    pub vertical: u32,
    pub horizontal: u32,
}

#[derive(Debug, Clone)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub struct PaddingOptions {
    pub top: u32,
    pub left: u32,
    pub right: u32,
    pub bottom: u32,
}
