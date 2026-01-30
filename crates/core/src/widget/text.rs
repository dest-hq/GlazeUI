use std::marker::PhantomData;

use crate::{Widget, color::Color, id::next_id, weight::TextWeight};

pub struct Text<App> {
    pub content: String,
    pub font_size: u32,
    pub weight: TextWeight,
    pub color: Color,
    _marker: PhantomData<App>,
}

impl<App> Text<App> {
    pub fn new(content: String) -> Self {
        Self {
            content: content,
            font_size: 14,
            weight: TextWeight::NORMAL,
            color: Color::rgb(255, 255, 255),
            _marker: PhantomData,
        }
    }

    pub fn size(mut self, font_size: u32) -> Self {
        self.font_size = font_size;
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
        let (r, g, b, a) = (self.color.r, self.color.g, self.color.b, self.color.a);
        return Widget::new(
            next_id(),
            crate::WidgetElement::Text {
                content: self.content,
                font_size: self.font_size,
                weight: self.weight,
                color: (r, g, b, a),
            },
            None,
        );
    }
}
