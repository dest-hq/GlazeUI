use glazeui::{
    application::start,
    core::{Color, Widget, button, vstack},
};

fn main() -> glazeui::Result {
    let init = ModifyWindow { maximized: false };

    start(init, ModifyWindow::view)
        .title("Modify the window settings")
        .run()
}

struct ModifyWindow {
    maximized: bool,
}

impl ModifyWindow {
    fn view(&mut self) -> Widget<ModifyWindow> {
        let close = button("Close")
            .label_size(25)
            .size(340, 50)
            .on_press(|_, window| window.close())
            .build();
        let background = button("Change background to white")
            .label_size(23)
            .size(340, 50)
            .on_press(|_, window| {
                window.background(Color::rgb(255, 255, 255));
            })
            .build();
        let change_title = button("Change title to Hi")
            .label_size(25)
            .size(340, 50)
            .on_press(|_, window| window.title("Hi"))
            .build();
        let off_decorations = button("Off decorations")
            .label_size(25)
            .size(340, 50)
            .on_press(|_, window| window.decorations(false))
            .build();
        let on_decorations = button("On decorations")
            .label_size(25)
            .size(340, 50)
            .on_press(|_, window| window.decorations(true))
            .build();
        let minimize = button("Minimize")
            .label_size(25)
            .size(340, 50)
            .on_press(|_, window| window.minimize())
            .build();
        let maximize = button("Maximize")
            .label_size(25)
            .size(340, 50)
            .on_press(|app: &mut ModifyWindow, window| {
                if app.maximized {
                    window.maximize(false);
                    app.maximized = false;
                } else {
                    window.maximize(true);
                    app.maximized = true;
                }
            })
            .build();
        let off_resizable = button("Off resizable")
            .label_size(25)
            .size(340, 50)
            .on_press(|_, window| window.resizable(false))
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
