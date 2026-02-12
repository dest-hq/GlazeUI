use kurbo::{Affine, Vec2};
use multirender::PaintScene;
use peniko::{ImageBrush, ImageData};

pub fn draw_image<T: PaintScene>(
    scene: &mut T,
    image_brush: ImageBrush<&ImageData>,
    x: f64,
    y: f64,
) {
    let transform = Affine::translate(Vec2::new(x, y));
    scene.draw_image(image_brush, transform);
}
