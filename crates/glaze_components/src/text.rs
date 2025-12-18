use cosmic_text::{Attrs, Buffer, FontSystem, Metrics};
use glaze_core::{Node, NodeElement};
use taffy::{AvailableSpace, Size, Style, prelude::length};

// Helper to create text easier

pub fn text(content: String) -> Text {
    Text::new(content)
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
    line_height: Option<f32>,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self {
            content,
            font_size: 14.0,
            line_height: None,
        }
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(line_height);
        self
    }

    // Transform in Node with id
    pub fn build_with(self, id: u64) -> Node {
        // Get line height
        let line_height = if self.line_height.is_none() {
            self.font_size * 1.2
        } else {
            self.line_height.unwrap_or(16.0)
        };

        let metrics = Metrics {
            font_size: self.font_size,
            line_height: line_height,
        };
        // Get system font
        let mut font_system = FontSystem::new();

        let mut ctx =
            CosmicTextContext::new(metrics, &self.content, Attrs::new(), &mut font_system);

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

        let mut node = Node::new(
            Some(id),
            NodeElement::Text {
                content: self.content,
                font_size: self.font_size,
                line_height: line_height,
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

    // Transform in Node without id
    pub fn build(self) -> Node {
        // Get line height
        let line_height = if self.line_height.is_none() {
            self.font_size * 1.2
        } else {
            self.line_height.unwrap_or(16.0)
        };

        let metrics = Metrics {
            font_size: self.font_size,
            line_height: line_height,
        };
        // Get system font
        let mut font_system = FontSystem::new();

        let mut ctx =
            CosmicTextContext::new(metrics, &self.content, Attrs::new(), &mut font_system);

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

        let mut node = Node::new(
            None,
            NodeElement::Text {
                content: self.content,
                font_size: self.font_size,
                line_height: line_height,
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
