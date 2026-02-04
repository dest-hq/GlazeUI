use std::marker::PhantomData;

use crate::{Margin, Widget, color::Color, id::next_id, style::Style, weight::TextWeight};

pub struct Text<App> {
    pub content: String,
    pub font_size: u32,
    pub weight: TextWeight,
    pub color: Color,
    pub margin: Margin,
    _marker: PhantomData<App>,
}

impl<App> Text<App> {
    pub fn new(content: String) -> Self {
        Self {
            content: content,
            font_size: 14,
            weight: TextWeight::NORMAL,
            color: Color::rgb(255, 255, 255),
            margin: Margin::new(),
            _marker: PhantomData,
        }
    }

    pub fn size(mut self, font_size: u32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
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

        // Text style
        let text_style = Style {
            margin: self.margin,
            ..Default::default()
        };

        Widget {
            id: next_id(),
            element: crate::WidgetElement::Text {
                content: self.content,
                font_size: self.font_size,
                weight: self.weight,
                color: (r, g, b, a),
            },
            on_press: None,
            style: text_style,
            _marker: PhantomData,
        }
    }
}
