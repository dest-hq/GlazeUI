use kurbo::{Affine, RoundedRect};
use multirender::PaintScene;
use peniko::Color;

pub fn draw_rectangle<T: PaintScene>(
    scene: &mut T,
    radius: f64,
    color: Color,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    // Draw a rectangle
    let rect = RoundedRect::new(x, y, x + width, y + height, radius);
    scene.fill(peniko::Fill::NonZero, Affine::IDENTITY, color, None, &rect);
}
