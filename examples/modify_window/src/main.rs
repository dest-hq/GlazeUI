use glazeui::{
    application::start,
    core::{Widget, button, color::Color, vstack, window::control::Window},
};

fn main() -> glazeui::Result {
    let app = ModifyWindow { maximized: false };
    start(app, ModifyWindow::view).run()
}

struct ModifyWindow {
    maximized: bool,
}

impl ModifyWindow {
    fn view(&mut self) -> Widget<ModifyWindow> {
        let close = button("Close")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|_, window: &mut Window| window.close())
            .build();
        let background = button("Change background to white")
            .label_size(23)
            .size(340.0, 50.0)
            .on_click(|_, window: &mut Window| {
                window.background(Color::rgb(255, 255, 255));
            })
            .build();
        let change_title = button("Change title to Hi")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|_, window: &mut Window| window.title("Hi"))
            .build();
        let off_decorations = button("Off decorations")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|_, window: &mut Window| window.decorations(false))
            .build();
        let on_decorations = button("On decorations")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|_, window: &mut Window| window.decorations(true))
            .build();
        let off_resizable = button("Off resizable")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|_, window: &mut Window| window.resizable(false))
            .build();
        let minimize = button("Minimize")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|_, window: &mut Window| window.minimize())
            .build();
        let maximize = button("Maximize")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|app: &mut ModifyWindow, window: &mut Window| {
                if app.maximized {
                    window.maximize(false);
                    app.maximized = false;
                } else {
                    window.maximize(true);
                    app.maximized = true;
                }
            })
            .build();

        vstack!(
            close,
            background,
            change_title,
            off_decorations,
            on_decorations,
            off_resizable,
            minimize,
            maximize
        )
        .spacing(20.0)
        .build()
    }
}
