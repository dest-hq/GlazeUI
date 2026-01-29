use vello::{
    Scene,
    kurbo::{Affine, RoundedRect},
    peniko::Color,
};

pub fn draw_rectangle(
    scene: &mut Scene,
    radius: f64,
    color: Color,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    // Draw a rectangle
    let rect = RoundedRect::new(x, y, x + width, y + height, radius);
    scene.fill(
        vello::peniko::Fill::NonZero,
        Affine::IDENTITY,
        color,
        None,
        &rect,
    );
}
