use glazeui::{
    application::start,
    core::{Widget, label, vstack, window::Window},
};

fn main() -> glazeui::Result {
    let init = HelloWorld {
        text: "Hello world!".to_string(),
    };

    start(init, HelloWorld::view, HelloWorld::update)
        .title("Hello world!")
        .run()
    // This starts the app with the title of "Hello world!" and with the width and height of 900 pixels
    // P.S. You can add ".vsync(false)" between "start(app)" and ".run()" to disable VSync which will make your GPU draw the app as fast as it can
    // Note: As of writing this, disabling VSync will flood your terminal with the amount of frames per second every frame and will also induce a high load on your GPU
}

struct HelloWorld {
    text: String,
}

#[derive(Clone)]
enum Message {}

impl HelloWorld {
    fn update(&mut self, _: Message, _: &mut Window) {}
    fn view(&mut self) -> Widget<Message> {
        let hello_world_text = label(&self.text)
            .size(36) // Set text size to 36 pixels
            .build(); // Turn text element into Widget

        vstack!(hello_world_text)
            .spacing(20) // VStack is used to vertically stack multiple elements vertically, but since we have only 1 element a HStack could also be used here
            .build() // Turn VStack to Widget
    }
}
