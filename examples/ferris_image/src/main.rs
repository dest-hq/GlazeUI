use std::path::Path;

use glazeui::{
    application::start,
    core::{Widget, image, label, vstack, window::Window},
};

fn main() -> glazeui::Result {
    let init = Image {};

    start(init, Image::view, Image::update)
        .title("Ferris Image")
        .run()
}

struct Image {}

#[derive(Clone)]
enum Message {}

impl Image {
    fn update(&mut self, _: Message, _: &mut Window) {}

    fn view(&mut self, _: &mut Window) -> Widget<Message> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("assets")
            .join("ferris.png");
        let ferris_text = label("Ferris").size(35).build();
        let image = image()
            .from_path(path, Some(300), Some(200)) // If one of size's (width, height) is set to None it will be set auto to image native size
            .unwrap()
            .build();
        vstack!(ferris_text, image).spacing(20).build()
    }
}
