use glazeui::{
    application::start,
    core::{Widget, button, text, vstack},
};

fn main() -> glazeui::Result {
    let init = Clicker { count: 0 };
    start(init, Clicker::view)
        .size(900, 900)
        .title("Clicker")
        .run()
}

struct Clicker {
    count: i32,
}

impl Clicker {
    fn view(&mut self) -> Widget<Clicker> {
        let add = button("+")
            .label_size(24)
            .on_click(|app: &mut Clicker, _| app.count += 1)
            .build();
        let count = text(&self.count.to_string()).size(24).build();
        let minus = button("-")
            .label_size(24)
            .on_click(|app: &mut Clicker, _| app.count -= 1)
            .build();
        vstack!(add, count, minus).build()
    }
}
