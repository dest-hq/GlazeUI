use glazeui::{Error, Window, core::ui::Ui, start, types::Color};

fn main() -> Result<(), Error> {
    let app = ModifyWindow {};
    start(app, ModifyWindow::view).run()
}

struct ModifyWindow {}

impl ModifyWindow {
    fn view(&mut self, ui: &mut Ui<ModifyWindow>) {
        let close = ui
            .button("Close")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|_app: &mut ModifyWindow, window: &mut Window| window.close())
            .build();
        let background = ui
            .button("Change background to white")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|_app: &mut ModifyWindow, window: &mut Window| {
                window.background(Color::rgb(255, 255, 255));
            })
            .build();
        let change_title = ui
            .button("Change title to Hi")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|_app: &mut ModifyWindow, window: &mut Window| window.title("Hi"))
            .build();
        let off_decorations = ui
            .button("Off decorations")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|_app: &mut ModifyWindow, window: &mut Window| window.decorations(false))
            .build();
        let on_decorations = ui
            .button("On decorations")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|_app: &mut ModifyWindow, window: &mut Window| window.decorations(true))
            .build();
        let off_resizable = ui
            .button("Off resizable")
            .label_size(25)
            .size(340.0, 50.0)
            .on_click(|_app: &mut ModifyWindow, window: &mut Window| window.resizable(false))
            .build();

        ui.vstack(vec![
            close,
            background,
            change_title,
            off_decorations,
            on_decorations,
            off_resizable,
        ])
        .align(glazeui::types::Align::Center)
        .spacing(20.0)
        .length(glazeui::types::Length::Fill)
        .show();
    }
}
