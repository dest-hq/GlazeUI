#[derive(Debug)]
pub struct Padding {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Debug)]
pub enum HorizontalAlign {
    Center,
    Left,
    Right,
}
#[derive(Debug)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}
