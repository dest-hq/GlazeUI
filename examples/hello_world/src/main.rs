use glazeui::{Error, core::ui::Ui, start};

fn main() -> Result<(), Error> {
    let app = HelloWorld {
        text: "Hello world!".to_string(),
    };

    start(app, HelloWorld::view)
        .title("Hello world!")
        .size(900, 900)
        .run()
    // This starts the app with the title of "Hello world!" and with the width and height of 900 pixels
    // P.S. You can add ".vsync(false)" between "start(app)" and ".run()" to disable VSync which will make your GPU draw the app as fast as it can
    // Note: As of writing this, disabling VSync will flood your terminal with the amount of frames per second every frame and will also induce a high load on your GPU
}

struct HelloWorld {
    text: String,
}

impl HelloWorld {
    fn view(&mut self, ui: &mut Ui<HelloWorld>) {
        let hello_world_text = ui
            .text(&self.text)
            .size(36) // Set text size to 36 pixels
            .build(); // Turn text element into Widget

        ui.vstack(vec![hello_world_text]) // VStack is used to vertically stack multiple elements vertically, but since we have only 1 element a HStack could also be used here
            .align(glazeui::types::Align::Center)
            .length(glazeui::types::Length::Fill) // Fill all the space
            .show() // Turn VStack to Widget
    }
}
