use glazeui_core::{TextStyle, TextWeight};
use parley::{
    FontContext, FontWeight, GenericFamily, Layout, LayoutContext, LineHeight, StyleProperty,
};

pub fn measure_text(
    font_cx: &mut FontContext,
    text: &str,
    text_weight: &TextWeight,
    text_style: &TextStyle,
    text_spacing: i32,
    font_size: f32,
    scale: f32,
    layout_cx: &mut LayoutContext,
) -> (f32, f32) {
    // Create a RangedBuilder
    let mut builder = layout_cx.ranged_builder(font_cx, &text, scale, true);

    let weight = match text_weight {
        TextWeight::THIN => 100.0,
        TextWeight::EXTRALIGHT => 200.0,
        TextWeight::LIGHT => 300.0,
        TextWeight::NORMAL => 400.0,
        TextWeight::MEDIUM => 500.0,
        TextWeight::SEMIBOLD => 600.0,
        TextWeight::BOLD => 700.0,
        TextWeight::EXTRABOLD => 800.0,
        TextWeight::BLACK => 900.0,
    };

    let style = match text_style {
        TextStyle::Italic => parley::FontStyle::Italic,
        TextStyle::Normal => parley::FontStyle::Normal,
        _ => parley::FontStyle::Normal,
    };

    let striketrough = match text_style {
        TextStyle::Striketrough => true,
        _ => false,
    };

    let underline = match text_style {
        TextStyle::Underline => true,
        _ => false,
    };

    // Set default font family
    builder.push_default(GenericFamily::SystemUi);
    builder.push_default(StyleProperty::FontWeight(FontWeight::new(weight)));
    builder.push_default(StyleProperty::FontStyle(style));
    builder.push_default(LineHeight::FontSizeRelative(1.3));
    builder.push_default(StyleProperty::FontSize(font_size));
    builder.push_default(StyleProperty::Strikethrough(striketrough));
    builder.push_default(StyleProperty::Underline(underline));
    builder.push_default(StyleProperty::LetterSpacing(text_spacing as f32));

    // Build the builder into a Layout
    let mut layout: Layout<[u8; 4]> = builder.build(&text);
    layout.break_all_lines(None);

    // Return text size
    return (layout.width(), layout.height());
}
