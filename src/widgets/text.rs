use std::marker::PhantomData;

use crate::{
    core::widget::Widget,
    types::{Align, Color, Length, TextWeight},
    widgets::ui::build_text,
};

pub fn text<App>(content: &str) -> Text<App> {
    Text::new(content.to_string())
}

// Helper to create text easier

pub struct Text<App> {
    pub content: String,
    pub font_size: u32,
    pub weight: TextWeight,
    pub color: Color,
    pub align: Option<Align>,
    pub length: Option<Length>,
    _marker: PhantomData<App>,
}

impl<App> Text<App> {
    pub fn new(content: String) -> Self {
        Self {
            content: content,
            font_size: 14,
            weight: TextWeight::NORMAL,
            color: Color::rgb(255, 255, 255),
            align: None,
            length: None,
            _marker: PhantomData,
        }
    }

    pub fn size(mut self, font_size: u32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn center(mut self) -> Self {
        self.align = Some(Align::Center);
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    pub fn length(mut self, length: Length) -> Self {
        self.length = Some(length);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn build(self) -> Widget<App> {
        build_text(self)
    }
}
