use std::marker::PhantomData;

use crate::core::node::{NodeElement, Widget};
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics};
use taffy::{AvailableSpace, Size, Style, prelude::length};

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

struct CosmicTextContext {
    buffer: cosmic_text::Buffer,
}

impl CosmicTextContext {
    fn new(metrics: Metrics, text: &str, attrs: Attrs, font_system: &mut FontSystem) -> Self {
        let mut buffer = Buffer::new_empty(metrics);
        buffer.set_size(font_system, None, None);
        buffer.set_text(
            font_system,
            text,
            &attrs,
            cosmic_text::Shaping::Advanced,
            Default::default(),
        );
        Self { buffer }
    }

    fn measure(
        &mut self,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<AvailableSpace>,
        font_system: &mut FontSystem,
    ) -> taffy::Size<f32> {
        // Set width constraint
        let width_constraint = known_dimensions.width.or(match available_space.width {
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
            AvailableSpace::Definite(width) => Some(width),
        });
        self.buffer.set_size(font_system, width_constraint, None);

        // Compute layout
        self.buffer.shape_until_scroll(font_system, false);

        // Determine measured size of text
        let (width, total_lines) = self
            .buffer
            .layout_runs()
            .fold((0.0, 0usize), |(width, total_lines), run| {
                (run.line_w.max(width), total_lines + 1)
            });
        let height = total_lines as f32 * self.buffer.metrics().line_height;

        Size { width, height }
    }
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

    // pub fn id(mut self, mut id: u64) -> Self {
    //     if id < 1000 {
    //         id = 1000 + id;
    //         println!(
    //             "It is recommended to set the ID above 1,000 to avoid conflicts with widgets where the ID is set automatically. The ID was set automatically: {}",
    //             id
    //         );
    //     }
    //     self.id = Some(id);
    //     self
    // }
}

// Transform in widget
impl<Message> From<Text<Message>> for Widget<Message> {
    fn from(builder: Text<Message>) -> Widget<Message> {
        // Get text weight
        let weight = match builder.weight {
            TextWeight::THIN => 100,
            TextWeight::EXTRA_LIGHT => 200,
            TextWeight::LIGHT => 300,
            TextWeight::NORMAL => 400,
            TextWeight::MEDIUM => 500,
            TextWeight::SEMIBOLD => 600,
            TextWeight::BOLD => 700,
            TextWeight::EXTRA_BOLD => 800,
            TextWeight::BLACK => 900,
        };

        // Get line height
        let line_height = builder.font_size as f32 * 1.3;

        let metrics = Metrics {
            font_size: builder.font_size as f32,
            line_height: line_height,
        };
        // Get system font
        let mut font_system = FontSystem::new();

        let mut ctx = CosmicTextContext::new(
            metrics,
            &builder.content,
            Attrs::new().weight(cosmic_text::Weight(weight)),
            &mut font_system,
        );

        let size = ctx.measure(
            Size {
                width: None,
                height: None,
            },
            Size {
                width: AvailableSpace::MaxContent,
                height: AvailableSpace::MaxContent,
            },
            &mut font_system,
        );
        let mut widget = Widget::new(
            next_id(),
            NodeElement::Text {
                content: builder.content,
                font_size: builder.font_size,
                line_height: line_height,
                weight: builder.weight,
            },
            None,
        );

        widget.style = Style {
            size: Size {
                width: length(size.width),
                height: length(size.height),
            },
            ..Default::default()
        };
        widget
    }
}
