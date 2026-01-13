use glazeui::{
    core::app::{App, Element},
    hstack, vstack,
    widgets::{
        container::container,
        text::{self, text},
    },
};
use winit::{
    dpi::{PhysicalSize, Size},
    window::WindowAttributes,
};

fn generate_numbers() -> (i32, i32, i32) {
    let a = rand::random_range(1..=100);
    let b = rand::random_range(a..=200);
    let what = rand::random_bool(0.5);
    if what {
        let c = a.clone();
        (a, b, c)
    } else {
        let c = b.clone();
        (a, b, c)
    }
}

fn main() {
    let (n1, n2, n3) = generate_numbers();

    let app = Random {
        number1: n1,
        number2: n2,
        number3: n3,
        text: "Guess what number".to_string(),
    };

    let window = WindowAttributes::default()
        .with_inner_size(Size::Physical(PhysicalSize {
            width: 900,
            height: 900,
        }))
        .with_title("Guess game");

    glazeui::run(app, window);
}

struct Random {
    number1: i32,
    number2: i32,
    number3: i32,
    text: String,
}

enum Messages {
    Guess(i32),
}

impl App for Random {
    type Message = Messages;

    fn update(&mut self, message: Self::Message) {
        match message {
            Messages::Guess(guess) => {
                if guess == self.number3 {
                    self.text = "You win, the game started again".to_string();
                } else {
                    self.text = format!(
                        "You lose, the game started again (the number was {})",
                        self.number3
                    );
                }

                let (n1, n2, n3) = generate_numbers();
                self.number1 = n1;
                self.number2 = n2;
                self.number3 = n3;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let number = text("?").size(30.0).into();

        let mission = text(&self.text)
            .size(20.0)
            .weight(text::TextWeight::LIGHT)
            .into();

        let button1 = container(
            vstack!(
                text(&self.number1.to_string())
                    .size(20.0)
                    .weight(text::TextWeight::NORMAL)
                    .into()
            )
            .vertical_align(glazeui::widgets::utils::types::VerticalAlign::Center)
            .horizontal_align(glazeui::widgets::utils::types::HorizontalAlign::Center)
            .into(),
        )
        .on_click(Messages::Guess(self.number1))
        .radius(20.0)
        .color(255, 255, 255, 1)
        .into();

        let button2 = container(
            vstack!(
                text(&self.number2.to_string())
                    .size(20.0)
                    .weight(text::TextWeight::NORMAL)
                    .into()
            )
            .vertical_align(glazeui::widgets::utils::types::VerticalAlign::Center)
            .horizontal_align(glazeui::widgets::utils::types::HorizontalAlign::Center)
            .into(),
        )
        .on_click(Messages::Guess(self.number2))
        .radius(20.0)
        .color(255, 255, 255, 1)
        .into();

        let buttons = hstack!(button1, button2).spacing(20.0).into();

        let layout = vstack!(number, mission, buttons)
            .spacing(20.0)
            .horizontal_align(glazeui::widgets::utils::types::HorizontalAlign::Center)
            .vertical_align(glazeui::widgets::utils::types::VerticalAlign::Center)
            .into();

        Element::new(layout)
    }
}
