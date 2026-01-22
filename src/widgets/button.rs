use std::{cell::RefCell, rc::Rc};

use crate::{
    core::{ui::Ui, widget::Widget},
    types::{Align, Length},
    widgets::text::TextWeight,
};

use crate::types::{Color, Padding};

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
    pub on_click: Option<Rc<RefCell<dyn FnMut(&mut App)>>>,
}

pub struct ButtonHandle<'a, App> {
    pub ui: &'a mut Ui<App>,
    pub button: Button<App>,
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
}

impl<'a, App> ButtonHandle<'a, App> {
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.button.width = width;
        self.button.height = height;
        self
    }

    pub fn label_size(mut self, font_size: u32) -> Self {
        self.button.label_size = font_size;
        self
    }

    pub fn label_weight(mut self, weight: TextWeight) -> Self {
        self.button.label_weight = weight;
        self
    }

    pub fn label_color(mut self, color: Color) -> Self {
        self.button.label_color = color;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.button.color = color;
        self
    }

    pub fn radius(mut self, corner_radius: f32) -> Self {
        self.button.radius = corner_radius;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.button.padding = padding;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.button.align = Some(align);
        self
    }

    pub fn length(mut self, length: Length) -> Self {
        self.button.length = Some(length);
        self
    }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: FnMut(&mut App) + 'static,
    {
        self.button.on_click = Some(Rc::new(RefCell::new(f)));
        self
    }

    pub fn show(self) {
        self.ui.push_button(self.button);
    }

    pub fn build(self) -> Widget<App> {
        self.ui.build_button(self.button)
    }
}
