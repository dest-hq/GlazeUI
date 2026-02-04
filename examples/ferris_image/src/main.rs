// Right now, this example doesn't work, because i didnt implement image in wgpu yet

use std::path::Path;

use glazeui::{
    application::start,
    core::{image, text, vstack, Widget},
};

fn main() -> glazeui::Result {
    let init = Image {};

    start(init, Image::view).title("Ferris Image").run()
}

struct Image {}

impl Image {
    fn view(&mut self) -> Widget<Image> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("assets")
            .join("ferris.png");
        let ferris_text = text("Ferris").size(35).build();
        let image = image()
            .from_file(path, Some(300), Some(200)) // If one of size's (width, height) is set to None it will be set auto to image native size
            .unwrap()
            .build();
        vstack!(ferris_text, image).spacing(20).build()
    }
}
