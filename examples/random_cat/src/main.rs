use glazeui::{
    application::start,
    core::{Widget, button, image, label, task::Task, vstack, window::Window},
};
use image::EncodableLayout;

fn main() -> glazeui::Result {
    let init = RandomCat { image_bytes: None };

    start(init, RandomCat::view, RandomCat::update)
        .size(700, 700)
        .resizable(false)
        .run()
}

struct RandomCat {
    image_bytes: Option<Vec<u8>>,
}

#[derive(Clone)]
enum Message {
    GetBytesOfCatImage,
    UpdateBytesOfCatImage(Vec<u8>),
}

async fn get_cat_image() -> Message {
    let img_bytes = reqwest::blocking::get("https://cataas.com/cat")
        .unwrap()
        .bytes()
        .unwrap();

    Message::UpdateBytesOfCatImage(img_bytes.into())
}

impl RandomCat {
    fn update(&mut self, message: Message, _: &mut Window) -> Task<Message> {
        match message {
            Message::GetBytesOfCatImage => Task::new(async { get_cat_image().await }),
            Message::UpdateBytesOfCatImage(bytes) => {
                self.image_bytes = Some(bytes);
                Task::none()
            }
        }
    }

    fn view(&mut self, _: &mut Window) -> Widget<Message> {
        let cat_image = if let Some(bytes) = &self.image_bytes {
            image()
                .from_bytes(bytes.as_bytes(), Some(700), Some(500))
                .unwrap()
                .build()
        } else {
            label("No cat image").size(25).build()
        };
        let update_image = button("Update cat image")
            .width(300)
            .height(100)
            .label_size(25)
            .on_press(Message::GetBytesOfCatImage)
            .build();

        vstack!(cat_image, update_image).spacing(20).build()
    }
}
