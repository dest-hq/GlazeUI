use glaze_components::{button::button, text::text, vstack::vstack};
use glaze_core::{
    component::{App, Element},
    Node,
};

fn ui() {
    let text = text("Clicker".into()).size(20.0).id(2);
    let button = button("+1".into()).width(100.0).height(50.0).id(3);
    let vstack = vstack().spacing(10.0).child(text).child(button).id(5);
    let container = Node::new(
        4,
        glaze_core::NodeElement::Container {
            children: vec![vstack],
        },
    );
    println!("{:?}", container);
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
            let text = text("Clicker".to_string()).size(20.0).id(1);
            let button = button("+1".to_string()).width(100.0).height(20.0).id(2);
            let list = vstack().child(text).child(button).id(3);

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
