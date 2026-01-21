use glazeui::{
    Error,
    core::{app::Application, widget::Widget},
    start, vstack,
    widgets::{
        text::{self, text}
    },
};

fn main() -> Result<(), Error> {

    let app = HelloWorld {
        text: "Hello world!".to_string(),
    };

    start(app).title("Hello world!").size(900, 900).run()
    // This starts the app with the title of "Hello world!" and with the width and height of 900 pixels
    // P.S. You can add ".vsync(false)" between "start(app)" and ".run()" to disable VSync which will make your GPU draw the app as fast as it can
    // Note: As of writing this, disabling VSync will flood your terminal with the amount of frames per second every frame and will also induce a high load on your GPU
}

struct HelloWorld {
    text: String,
}

enum Message {
    // Message is used to do stuff like modifying variables on click, etc
    // Since this app is static (not changing), there's nothing inside this enum
}

impl Application for HelloWorld {
    type Message = Message;

    fn update(&mut self, _message: Self::Message) {
        // Nothing here since this app is static
        // P.S. "_message" would be usually called "message" but is instead called that to avoid unused variable warning from compiler
    }

    fn view(&self) -> Widget<Self::Message> {
        let hello_world_text = text(&self.text)
            .size(36) // Set text size to 36 pixels
            .into(); // Turn text element into Widget
        
        vstack!(hello_world_text) // VStack is used to vertically stack multiple elements vertically, but since we have only 1 element a HStack could also be used here
        .horizontal_align(glazeui::widgets::utils::types::HorizontalAlign::Center)
        .vertical_align(glazeui::widgets::utils::types::VerticalAlign::Center)
        .into() // Turn VStack to Widget
    }
}
