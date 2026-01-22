#![allow(dead_code)]

use std::sync::Arc;

//TODO: replace shape enum with a single definition with optional corner radius
//Squash rectangles into rounded rectangles. Ignore corner radius on Ellipse
pub use crate::renderer::components::shape;
pub use shape::Shape as ShapeType;

use image::RgbaImage;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CanvasColor(pub u8, pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shape {
    pub shape: ShapeType,
    pub color: CanvasColor,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub shape: ShapeType,
    pub image: Arc<RgbaImage>,
    pub color: Option<CanvasColor>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Shape(Shape),
    Image(Image),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Area {
    pub offset: (f32, f32),
    pub bounds: Option<(f32, f32, f32, f32)>,
}
