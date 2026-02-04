use crate::{align::Align, margin::Margin, padding::Padding};

#[derive(Clone, Debug, Default)]
pub struct Style {
    pub width: u32,
    pub height: u32,
    pub padding: Padding,
    pub margin: Margin,
    pub spacing: i32,
    pub align: Option<Align>,
}
