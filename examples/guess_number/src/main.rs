use glazeui::{
    Error,
    core::ui::Ui,
    start,
    types::{Align, Length},
    widgets::text::TextWeight,
};

fn main() -> Result<(), Error> {
    let (n1, n2, n3) = generate_numbers();

    let app = Random {
        number1: n1,
        number2: n2,
        number3: n3,
        text: "Guess the number".to_string(),
    };

    start(app, Random::view)
        .title("Guess game")
        .size(900, 900)
        .run()
}

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

struct Random {
    number1: i32,
    number2: i32,
    number3: i32,
    text: String,
}

impl Random {
    fn verify(&mut self, guess: i32) {
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

    fn view(&mut self, ui: &mut Ui<Random>) {
        let number = ui.text("?").size(30).build();
        let mission = ui
            .text(&self.text)
            .size(20)
            .weight(TextWeight::LIGHT)
            .build();
        let button1 = ui
            .button(&self.number1.to_string())
            .label_size(20)
            .on_click(|app: &mut Random| {
                app.verify(app.number1);
            })
            .build();
        let button2 = ui
            .button(&self.number2.to_string())
            .label_size(20)
            .on_click(|app: &mut Random| {
                app.verify(app.number2);
            })
            .build();

        let hstack1 = ui.hstack(vec![button1, button2]).spacing(20.0).build();

        ui.vstack(vec![number, mission, hstack1])
            .spacing(20.0)
            .align(Align::Center)
            .length(Length::Fill)
            .show();
    }
}
