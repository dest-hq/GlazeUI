use std::{cell::RefCell, rc::Rc};

use crate::{
    Window,
    core::widget::Widget,
    types::{Align, Color, Length, Padding, TextWeight},
    widgets::ui::build_button,
};

pub fn button<App>(label: &str) -> Button<App> {
    Button::new(label.to_string())
}

pub struct Button<App> {
    pub label: String,
    pub label_size: u32,
    pub label_weight: TextWeight,
    pub label_color: Color,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub radius: f32,
    pub padding: Padding,
    pub align: Option<Align>,
    pub length: Option<Length>,
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
            padding: Padding {
                top: 0.0,
                left: 0.0,
                right: 0.0,
                bottom: 0.0,
            },
            // id: None,
            on_click: None,
            align: None,
            length: None,
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

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
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

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: FnMut(&mut App, &mut Window) + 'static,
    {
        self.on_click = Some(Rc::new(RefCell::new(f)));
        self
    }

    pub fn build(self) -> Widget<App> {
        build_button(self)
    }
}
