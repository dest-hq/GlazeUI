use glaze_components::{
    button::button, container::container, hstack::hstack, text::text, vstack::vstack,
};
use glaze_core::component::{App, Element};
use glaze_layout::LayoutEngine;

fn ui() {
    let text = text("Clicker".into())
        .font_size(20.0)
        .line_height(16.0)
        .id(2);
    let button = button("+1".into()).width(100.0).height(50.0).id(3);
    let vstack = vstack(&[text, button]).spacing(10.0).id(5);
    let hstack = hstack(&[vstack.clone(), vstack.clone()])
        .spacing(10.0)
        .id(6);
    let container = container(hstack).width(800.0).height(600.0).id(4);

    let mut layout = LayoutEngine::new();
    layout.compute(&container, 800.0, 600.0);
}

fn app() {
    enum Message {
        Add,
    }
    struct Clicker {
        count: i32,
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

        fn view(&self) -> glaze_core::component::Element<Self::Message> {
            let text = text("Clicker".to_string()).font_size(20.0).id(1);
            let button = button("+1".to_string()).width(100.0).height(20.0).id(2);
            let list = vstack(&[text, button]).id(3);

            Element::new(list)
        }
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
