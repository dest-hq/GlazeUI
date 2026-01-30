use parley::{FontContext, GenericFamily, Layout, LayoutContext, LineHeight, StyleProperty};

pub fn measure_text(
    font_cx: &mut FontContext,
    text: &str,
    font_size: f32,
    scale: f32,
    layout_cx: &mut LayoutContext,
) -> (f32, f32) {
    // Create a RangedBuilder
    let mut builder = layout_cx.ranged_builder(font_cx, &text, scale, true);

    // Set default font family
    builder.push_default(GenericFamily::SystemUi);
    builder.push_default(LineHeight::FontSizeRelative(1.3));
    builder.push_default(StyleProperty::FontSize(font_size));

    // Build the builder into a Layout
    let mut layout: Layout<[u8; 4]> = builder.build(&text);
    layout.break_all_lines(None);

    // Return text size
    return (layout.width(), layout.height());
}
