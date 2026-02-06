use std::marker::PhantomData;

use crate::{
    Margin, Padding, TextStyle, Widget, align::Align, color::Color, id::next_id, style::Style,
    weight::TextWeight,
};

pub struct Button<M: Clone, App: 'static> {
    pub label: String,
    pub label_size: u32,
    pub label_weight: TextWeight,
    pub label_color: Color,
    pub label_style: TextStyle,
    pub label_spacing: i32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
    pub radius: u32,
    pub on_press: Option<M>,
    pub margin: Margin,
    pub padding: Padding,
    _marker: PhantomData<App>,
}

impl<M: Clone, App> Button<M, App> {
    pub fn new(label: String) -> Self {
        Self {
            label,
            label_size: 14,
            label_weight: TextWeight::NORMAL,
            label_color: Color::rgb(255, 255, 255),
            label_style: TextStyle::Normal,
            label_spacing: 0,
            width: 100,
            height: 50,
            color: Color::rgb(50, 50, 51),
            radius: 0,
            on_press: None,
            margin: Margin::new(),
            padding: Padding::new(),
            _marker: PhantomData,
        }
    }

    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn label_style(mut self, style: TextStyle) -> Self {
        self.label_style = style;
        self
    }

    pub fn label_size(mut self, font_size: u32) -> Self {
        self.label_size = font_size;
        self
    }

    pub fn label_weight(mut self, weight: TextWeight) -> Self {
        self.label_weight = weight;
        self
    }

    /// Extra spacing between letters
    pub fn label_spacing(mut self, spacing: i32) -> Self {
        self.label_spacing = spacing;
        self
    }

    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = color;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, corner_radius: u32) -> Self {
        self.radius = corner_radius;
        self
    }

    pub fn on_press(mut self, m: M) -> Self {
        self.on_press = Some(m);
        self
    }

    pub fn build(self) -> Widget<M, App> {
        // Button color
        let (r, g, b, a) = (self.color.r, self.color.g, self.color.b, self.color.a);
        // Text color
        let (r2, g2, b2, a2) = (
            self.label_color.a,
            self.label_color.g,
            self.label_color.b,
            self.label_color.a,
        );

        // Text style
        let text_style = Style {
            align: Some(Align::Center),
            spacing: self.label_spacing,
            ..Default::default()
        };

        // Create text widget
        let child = Widget::<M, App> {
            id: next_id(),
            element: crate::WidgetElement::Text {
                content: self.label,
                font_size: self.label_size,
                weight: self.label_weight,
                style: self.label_style,
                color: (r2, g2, b2, a2),
            },
            on_press: None,
            style: text_style,
            _marker: PhantomData,
        };

        // Container style
        let container_style = Style {
            width: self.width,
            height: self.height,
            padding: self.padding,
            margin: self.margin,
            ..Default::default()
        };

        // Create container widget
        Widget {
            id: next_id(),
            element: crate::WidgetElement::Container {
                child: Box::new(child),
                color: (r, g, b, a),
                radius: self.radius,
            },
            on_press: self.on_press,
            style: container_style,
            _marker: PhantomData,
        }
    }
}
