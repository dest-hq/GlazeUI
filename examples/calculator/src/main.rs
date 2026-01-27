use calculator_lib::Calculator;
use glazeui::{
    Error, Window,
    core::widget::Widget,
    hstack, start,
    types::{Align, Color, Length, Padding},
    vstack,
    widgets::{button::button, container::container, text::text},
};

fn main() -> Result<(), Error> {
    let app = CalculatorApp {
        question: String::new(),
    };
    start(app, CalculatorApp::view)
        .background(Color::rgb(30, 30, 30))
        .size(800, 800)
        .title("Calculator")
        .resizable(false)
        .run()
}

struct CalculatorApp {
    question: String,
}

impl CalculatorApp {
    fn calculate(&mut self) {
        let calculator = Calculator::new(&self.question).unwrap_or_default();
        let result = calculator.calculate(None);
        if let Ok(result) = result {
            self.question = result.to_string();
        } else {
            self.question = String::from("Error");
        }
    }

    fn view(&mut self) -> Widget<CalculatorApp> {
        let question_text = text(&self.question)
            .padding(Padding::new().right(10))
            .size(25)
            .build();
        let question_bar = container(question_text)
            .size(750, 60)
            .length(Length::Fill)
            .color(Color::rgb(35, 35, 35))
            .build();

        // Buttons

        let clear = button("Clear")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question = String::new())
            .build();
        let remove = button("Remove")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| {
                if !app.question.is_empty() {
                    let _ = app.question.remove(app.question.len() - 1);
                }
            })
            .build();
        let paranteza_left = button("(")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('('))
            .build();
        let paranteza_right = button(")")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push(')'))
            .build();

        let one = button("1")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _| app.question.push('1'))
            .build();
        let two = button("2")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('2'))
            .build();
        let three = button("3")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('3'))
            .build();
        let plus = button("+")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('+'))
            .build();

        let four = button("4")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('4'))
            .build();
        let five = button("5")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('5'))
            .build();
        let six = button("6")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('6'))
            .build();
        let multiply = button("*")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('*'))
            .build();

        let seven = button("7")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('7'))
            .build();
        let eight = button("8")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('8'))
            .build();
        let nine = button("9")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('9'))
            .build();
        let divide = button("/")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('/'))
            .build();

        let zero = button("0")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('0'))
            .build();
        let dot = button(".")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('.'))
            .build();
        let minus = button("-")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.question.push('-'))
            .build();
        let equal = button("=")
            .label_size(25)
            .size(90, 90)
            .color(Color::blue())
            .on_click(|app: &mut CalculatorApp, _w: &mut Window| app.calculate())
            .build();

        let buttons1 = hstack!(clear, remove, paranteza_left, paranteza_right)
            .spacing(20.0)
            .align(Align::Center)
            .length(Length::Fill)
            .build();
        let buttons2 = hstack!(one, two, three, plus).spacing(20.0).build();
        let buttons3 = hstack!(four, five, six, multiply).spacing(20.0).build();
        let buttons4 = hstack!(seven, eight, nine, divide).spacing(20.0).build();
        let buttons5 = hstack!(zero, dot, minus, equal).spacing(20.0).build();
        let layout = vstack!(
            question_bar,
            buttons1,
            buttons2,
            buttons3,
            buttons4,
            buttons5
        )
        .padding(Padding::new().top(30))
        .length(Length::Fill)
        .build();
        layout
    }
}
