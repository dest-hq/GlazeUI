use glazeui_core::{TextStyle, TextWeight};
use kurbo::{Affine, Line, Stroke, Vec2};
use multirender::{Glyph, PaintScene};
use parley::{
    FontContext, FontWeight, GenericFamily, Layout, LayoutContext, LineHeight,
    PositionedLayoutItem, StyleProperty,
};
use peniko::{Color, Fill};

pub fn draw_text<T: PaintScene>(
    scene: &mut T,
    x: f64,
    y: f64,
    font_cx: &mut FontContext,
    text: &str,
    text_color: Color,
    text_weight: &TextWeight,
    text_style: &TextStyle,
    text_spacing: i32,
    font_size: f32,
    scale: f32,
    layout_cx: &mut LayoutContext,
) {
    let transform = Affine::translate(Vec2::new(x, y));

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

    // Text Colors
    let r = (text_color.components[0] * 255.0) as u8;
    let g = (text_color.components[1] * 255.0) as u8;
    let b = (text_color.components[2] * 255.0) as u8;
    let a = (text_color.components[3] * 255.0) as u8;

    // Set default text colour styles
    builder.push_default(StyleProperty::Brush([r, g, b, a]));

    // Set default font family
    builder.push_default(GenericFamily::SystemUi);
    // Set font weight
    builder.push_default(StyleProperty::FontWeight(FontWeight::new(weight)));
    // Set font style (Italic, Normal)
    builder.push_default(StyleProperty::FontStyle(style));
    // Set line height (font size * 1.3)
    builder.push_default(LineHeight::FontSizeRelative(1.3));
    // Set font size
    builder.push_default(StyleProperty::FontSize(font_size));

    builder.push_default(StyleProperty::Strikethrough(striketrough));
    builder.push_default(StyleProperty::Underline(underline));
    builder.push_default(StyleProperty::LetterSpacing(text_spacing as f32));

    // Build the builder into a Layout
    let mut layout: Layout<[u8; 4]> = builder.build(&text);
    layout.break_all_lines(None);

    for line in layout.lines() {
        for item in line.items() {
            let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
                continue;
            };
            let style = glyph_run.style();

            if let Some(underline) = &style.underline {
                let underline_brush = &style.brush;
                let run_metrics = glyph_run.run().metrics();
                let offset = match underline.offset {
                    Some(offset) => offset,
                    None => run_metrics.underline_offset,
                };
                let width = match underline.size {
                    Some(size) => size,
                    None => run_metrics.underline_size,
                };

                let y = glyph_run.baseline() - offset + width / 2.;

                let line = Line::new(
                    (glyph_run.offset() as f64, y as f64),
                    ((glyph_run.offset() + glyph_run.advance()) as f64, y as f64),
                );

                let color = Color::from_rgba8(
                    underline_brush[0],
                    underline_brush[1],
                    underline_brush[2],
                    underline_brush[3],
                );

                scene.stroke(&Stroke::new(width.into()), transform, color, None, &line);
            }
            let run = glyph_run.run();
            let font = run.font();
            let font_size = run.font_size();

            let color = Color::from_rgba8(
                style.brush[0],
                style.brush[1],
                style.brush[2],
                style.brush[3],
            );

            let mut x = glyph_run.offset();
            let y = glyph_run.baseline();

            let glyphs = glyph_run.glyphs().map(|g| {
                let gx = x + g.x;
                let gy = y - g.y;
                x += g.advance;
                Glyph {
                    id: g.id,
                    x: gx,
                    y: gy,
                }
            });

            // Draw text
            scene.draw_glyphs(
                font,
                font_size,
                false,
                run.normalized_coords(),
                Fill::NonZero,
                color,
                *color.components.last().unwrap_or(&1.0),
                transform,
                None,
                glyphs.into_iter(),
            );

            if let Some(strikestrough) = &style.strikethrough {
                let strikethrough_brush = &style.brush;
                let run_metrics = glyph_run.run().metrics();
                let offset = match strikestrough.offset {
                    Some(offset) => offset,
                    None => run_metrics.strikethrough_offset,
                };
                let width = match strikestrough.size {
                    Some(size) => size,
                    None => run_metrics.strikethrough_size,
                };

                let y = glyph_run.baseline() - offset + run_metrics.strikethrough_size / 2.;

                let line = Line::new(
                    (glyph_run.offset() as f64, y as f64),
                    ((glyph_run.offset() + glyph_run.advance()) as f64, y as f64),
                );

                let color = Color::from_rgba8(
                    strikethrough_brush[0],
                    strikethrough_brush[1],
                    strikethrough_brush[2],
                    strikethrough_brush[3],
                );

                scene.stroke(&Stroke::new(width.into()), transform, color, None, &line);
            }
        }
    }
}
