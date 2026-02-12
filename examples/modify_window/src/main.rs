use glazeui::{
    application::start,
    core::{Color, Widget, button, vstack, window::Window},
};

fn main() -> glazeui::Result {
    let init = ModifyWindow { maximized: false };

    start(init, ModifyWindow::view, ModifyWindow::update)
        .title("Modify the window settings")
        .run()
}

struct ModifyWindow {
    maximized: bool,
}

#[derive(Clone)]
enum Message {
    Close,
    Background,
    Title,
    Decorations(bool),
    Minimize,
    Maximize,
    Resizable,
}

impl ModifyWindow {
    fn update(&mut self, message: Message, window: &mut Window) {
        match message {
            Message::Background => window.background(Color::rgb(255, 255, 255)),
            Message::Title => window.title("Hi"),
            Message::Resizable => window.resizable(false),
            Message::Decorations(decorations) => window.decorations(decorations),
            Message::Minimize => window.minimize(),
            Message::Maximize => {
                if self.maximized {
                    window.maximize(false);
                    self.maximized = false;
                } else {
                    window.maximize(true);
                    self.maximized = true;
                }
            }
            Message::Close => window.close(),
        }
    }

    fn view(&mut self) -> Widget<Message> {
        let close = button("Close")
            .label_size(25)
            .width(340)
            .height(50)
            .on_press(Message::Close)
            .build();
        let background = button("Change background to white")
            .label_size(23)
            .width(340)
            .height(50)
            .on_press(Message::Background)
            .build();
        let change_title = button("Change title to Hi")
            .label_size(25)
            .width(340)
            .height(50)
            .on_press(Message::Title)
            .build();
        let off_decorations = button("Off decorations")
            .label_size(25)
            .width(340)
            .height(50)
            .on_press(Message::Decorations(false))
            .build();
        let on_decorations = button("On decorations")
            .label_size(25)
            .width(340)
            .height(50)
            .on_press(Message::Decorations(true))
            .build();
        let minimize = button("Minimize")
            .label_size(25)
            .width(340)
            .height(50)
            .on_press(Message::Minimize)
            .build();
        let maximize = button("Maximize")
            .label_size(25)
            .width(340)
            .height(50)
            .on_press(Message::Maximize)
            .build();
        let off_resizable = button("Off resizable")
            .label_size(25)
            .width(340)
            .height(50)
            .on_press(Message::Resizable)
            .build();

        vstack!(
            background,
            change_title,
            off_decorations,
            on_decorations,
            minimize,
            maximize,
            off_resizable,
            close
        )
        .spacing(20)
        .build()
    }
}
