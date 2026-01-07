use glazeui_components::{button::button, container::container, hstack, text::text, vstack};

use glazeui_core::component::{App, Element};
use glazeui_layout::LayoutEngine;
use winit::window::Window;

use crate::app::run;

pub mod app;

pub fn ui() {
    let text = text("Clicker".into()).size(20.0).into();
    let button = button("+1".into()).width(100.0).height(50.0).build();
    let vstack = vstack![text, button].spacing(10.0).build();
    let hstack = hstack![vstack.clone(), vstack].spacing(10.0).build();
    let container = container(hstack).size(800.0, 600.0).build();

    let mut layout = LayoutEngine::new();
    layout.compute(&container, 800.0, 600.0);

    let app = Clicker { count: 3 };
    let window_settings = Window::default_attributes()
        .with_decorations(false)
        .with_title("Hi");
    run(app, window_settings);
}

struct Clicker {
    count: i32,
}

enum Message {
    Add,
}
impl App for Clicker {
    type Message = Message;

    fn new() -> Self {
        Clicker { count: 0 }
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Add => self.count += 1,
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let text = text("Clicker").size(20.0).into();
        let button = button("+1".to_string())
            .width(100.0)
            .height(20.0)
            .build_with(3);
        let list = vstack![text, button].build();

        let mut layout = LayoutEngine::new();
        layout.compute(&list, 800.0, 600.0);

        Element::new(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        ui();
    }
}
