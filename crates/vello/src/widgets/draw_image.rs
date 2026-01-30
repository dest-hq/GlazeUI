use vello::{
    Scene,
    kurbo::{Affine, Vec2},
    peniko::ImageBrush,
};

pub fn draw_image(scene: &mut Scene, image_brush: &ImageBrush, x: f64, y: f64) {
    let transform = Affine::translate(Vec2::new(x, y));
    scene.draw_image(image_brush, transform);
}
