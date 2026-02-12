use kurbo::{Affine, RoundedRect};
use multirender::PaintScene;
use peniko::Color;

pub fn draw_rectangle<T: PaintScene>(
    scene: &mut T,
    radius: f64,
    color: &(u8, u8, u8, u8),
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    // Draw a rectangle
    let rect = RoundedRect::new(x, y, x + width, y + height, radius);
    let pen_color = Color::from_rgba8(color.0, color.1, color.2, color.3);
    scene.fill(
        peniko::Fill::NonZero,
        Affine::IDENTITY,
        pen_color,
        None,
        &rect,
    );
}
