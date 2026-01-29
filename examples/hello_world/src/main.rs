use glazeui::{
    Result,
    application::start,
    core::{Widget, container, text, vstack},
};

fn main() -> Result {
    let app = HelloWorld {
        text: "Hello world!".to_string(),
    };

    start(app, HelloWorld::view)
        .title("Hello world!")
        .size(900, 900)
        .backend(glazeui::core::backend::Backend::OpenGL)
        .run()
    // This starts the app with the title of "Hello world!" and with the width and height of 900 pixels
    // P.S. You can add ".vsync(false)" between "start(app)" and ".run()" to disable VSync which will make your GPU draw the app as fast as it can
    // Note: As of writing this, disabling VSync will flood your terminal with the amount of frames per second every frame and will also induce a high load on your GPU
}

struct HelloWorld {
    text: String,
}

impl HelloWorld {
    fn view(&mut self) -> Widget<HelloWorld> {
        let hello_world_text = text("HIIIIIIIII\n hii")
            .size(36) // Set text size to 36 pixels
            .build(); // Turn text element into Widget

        let container2 = container(hello_world_text.clone())
            .size(200.0, 200.0)
            .build();
        let container3 = container(hello_world_text).size(200.0, 200.0).build();

        vstack!(container2, container3)
            .spacing(20.0) // VStack is used to vertically stack multiple elements vertically, but since we have only 1 element a HStack could also be used here
            .build() // Turn VStack to Widget
    }
}
