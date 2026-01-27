use glyphon::{Attrs, Buffer, FontSystem, Metrics, Shaping};

use crate::layout::style::{Size, SizeOptions};

pub struct TextContext {
    buffer: Buffer,
}

impl TextContext {
    pub fn new(metrics: Metrics, text: &str, attrs: Attrs, font_system: &mut FontSystem) -> Self {
        let mut buffer = Buffer::new_empty(metrics);
        buffer.set_text(
            font_system,
            text,
            &attrs,
            Shaping::Advanced,
            Default::default(),
        );
        Self { buffer }
    }

    pub fn measure(
        &mut self,
        available_space: f32, // Width
        font_system: &mut FontSystem,
        line_height: f32,
    ) -> Size {
        self.buffer
            .set_size(font_system, Some(available_space), Some(line_height));

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

        Size {
            width: SizeOptions::Fixed(width as u32),
            height: SizeOptions::Fixed(height as u32),
        }
    }
}
