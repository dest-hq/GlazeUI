use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{Widget, color::Color, id::next_id, weight::TextWeight, window::control::Window};

pub struct Button<App> {
    pub label: String,
    pub label_size: u32,
    pub label_weight: TextWeight,
    pub label_color: Color,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub radius: f32,
    pub on_click: Option<Rc<RefCell<dyn FnMut(&mut App, &mut Window)>>>,
}

impl<App> Button<App> {
    pub fn new(label: String) -> Self {
        Self {
            label,
            label_size: 14,
            label_weight: TextWeight::NORMAL,
            label_color: Color::rgb(255, 255, 255),
            width: 100.0,
            height: 50.0,
            color: Color::rgb(50, 50, 51),
            radius: 0.0,
            // id: None,
            on_click: None,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
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

    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = color;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, corner_radius: f32) -> Self {
        self.radius = corner_radius;
        self
    }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: FnMut(&mut App, &mut Window) + 'static,
    {
        self.on_click = Some(Rc::new(RefCell::new(f)));
        self
    }

    pub fn build(self) -> Widget<App> {
        let (r, g, b, a) = (self.color.r, self.color.g, self.color.b, self.color.a);
        let (r2, g2, b2, a2) = (
            self.label_color.a,
            self.label_color.g,
            self.label_color.b,
            self.label_color.a,
        );
        let child = Widget {
            id: next_id(),
            element: crate::WidgetElement::Text {
                content: self.label,
                font_size: self.label_size,
                line_height: 0.0,
                color: (r2, g2, b2, a2),
            },
            on_click: None,
            _marker: PhantomData,
        };
        Widget {
            id: next_id(),
            element: crate::WidgetElement::Container {
                child: Box::new(child),
                width: self.width,
                height: self.height,
                color: (r, g, b, a),
                radius: self.radius,
            },
            on_click: self.on_click,
            _marker: PhantomData,
        }
    }
}
