use std::marker::PhantomData;

use crate::{
    Margin, TextStyle, Widget, color::Color, id::next_id, style::Style, weight::TextWeight,
};

pub struct Text<M: Clone, App> {
    pub content: String,
    pub font_size: u32,
    pub weight: TextWeight,
    pub style: TextStyle,
    pub spacing: i32,
    pub color: Color,
    pub margin: Margin,
    _marker_app: PhantomData<App>,
    _marker_message: PhantomData<M>,
}

impl<M: Clone, App> Text<M, App> {
    pub fn new(content: String) -> Self {
        Self {
            content: content,
            font_size: 14,
            weight: TextWeight::NORMAL,
            style: TextStyle::Normal,
            spacing: 0,
            color: Color::rgb(255, 255, 255),
            margin: Margin::new(),
            _marker_app: PhantomData,
            _marker_message: PhantomData,
        }
    }

    pub fn size(mut self, font_size: u32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    /// Extra spacing between letters
    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = spacing;
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

    pub fn build(self) -> Widget<M, App> {
        let (r, g, b, a) = (self.color.r, self.color.g, self.color.b, self.color.a);

        // Text style
        let text_style = Style {
            margin: self.margin,
            spacing: self.spacing,
            ..Default::default()
        };

        Widget {
            id: next_id(),
            element: crate::WidgetElement::Text {
                content: self.content,
                font_size: self.font_size,
                weight: self.weight,
                style: self.style,
                color: (r, g, b, a),
            },
            on_press: None,
            style: text_style,
            _marker: PhantomData,
        }
    }
}
