use crate::{
    core::{ui::Ui, widget::Widget},
    types::{Align, Color, Length},
};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TextWeight {
    THIN,        // 100
    EXTRA_LIGHT, // 200
    LIGHT,       // 300
    NORMAL,      // 400,
    MEDIUM,      // 500
    SEMIBOLD,    // 600
    BOLD,        // 700
    EXTRA_BOLD,  // 800
    BLACK,       // 900
}

// Helper to create text easier

pub struct Text {
    pub content: String,
    pub font_size: u32,
    pub weight: TextWeight,
    pub color: Color,
    pub align: Option<Align>,
    pub length: Option<Length>,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self {
            content: content,
            font_size: 14,
            weight: TextWeight::NORMAL,
            color: Color::rgb(255, 255, 255),
            align: None,
            length: None,
        }
    }
}

pub struct TextHandle<'a, App> {
    pub ui: &'a mut Ui<App>,
    pub text: Text,
}

impl<'a, App> TextHandle<'a, App> {
    pub fn size(mut self, font_size: u32) -> Self {
        self.text.font_size = font_size;
        self
    }

    pub fn center(mut self) -> Self {
        self.text.align = Some(Align::Center);
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.text.align = Some(align);
        self
    }

    pub fn length(mut self, length: Length) -> Self {
        self.text.length = Some(length);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.text.color = color;
        self
    }

    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.text.weight = weight;
        self
    }

    pub fn show(self) {
        self.ui.push_text(self.text);
    }

    pub fn build(self) -> Widget<App> {
        self.ui.build_text(self.text)
    }
}
