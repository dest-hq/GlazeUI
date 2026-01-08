use crate::core::node::{Node, NodeElement};
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics};
use taffy::{AvailableSpace, Size, Style, prelude::length};

use crate::core::node::TextWeight;
use crate::widgets::utils::ui_id::next_id;

// Helper to create text easier

#[allow(dead_code)]
pub fn text(content: &str) -> Text {
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

pub struct Text {
    content: String,
    font_size: f32,
    weight: TextWeight,
    id: Option<u64>,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self {
            content: content,
            font_size: 14.0,
            weight: TextWeight::NORMAL,
            id: None,
        }
    }

    pub fn size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }
}

// Transform in Node
impl From<Text> for Node {
    fn from(builder: Text) -> Node {
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
        let line_height = builder.font_size * 1.3;

        let metrics = Metrics {
            font_size: builder.font_size,
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
        // Get id
        let id = builder.id.unwrap_or(next_id());
        let mut node = Node::new(
            id,
            NodeElement::Text {
                content: builder.content,
                font_size: builder.font_size,
                line_height: line_height,
                weight: builder.weight,
            },
        );

        node.style = Style {
            size: Size {
                width: length(size.width),
                height: length(size.height),
            },
            ..Default::default()
        };
        node
    }
}
