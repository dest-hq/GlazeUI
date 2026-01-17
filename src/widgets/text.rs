use std::marker::PhantomData;

use crate::core::widget::{Widget, WidgetElement};

use crate::widgets::utils::ui_id::next_id;

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

#[allow(dead_code)]
pub fn text<Message>(content: &str) -> Text<Message> {
    Text::new(content.to_string())
}

pub struct Text<Message> {
    _marker: PhantomData<Message>,
    content: String,
    font_size: u32,
    weight: TextWeight,
    // id: Option<u64>,
}

impl<Message> Text<Message> {
    pub fn new(content: String) -> Self {
        Self {
            _marker: PhantomData,
            content: content,
            font_size: 14,
            weight: TextWeight::NORMAL,
            // id: None,
        }
    }

    pub fn size(mut self, font_size: u32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = weight;
        self
    }
}

// Transform in widget
impl<Message> From<Text<Message>> for Widget<Message> {
    fn from(builder: Text<Message>) -> Widget<Message> {
        // Get line height
        let line_height = builder.font_size as f32 * 1.3;

        let widget = Widget::new(
            next_id(),
            WidgetElement::Text {
                content: builder.content,
                font_size: builder.font_size,
                line_height: line_height,
                weight: builder.weight,
            },
            None,
        );
        widget
    }
}
