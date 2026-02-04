use glazeui::{
    application::start,
    core::{Widget, button, hstack, text, vstack},
};

fn main() -> glazeui::Result {
    let init = Clicker {
        count: 0,
        add: 1,
        cost: 60,
    };

    start(init, Clicker::view).title("Clicker").run()
}

struct Clicker {
    count: u32,
    add: u32,
    cost: u32,
}

impl Clicker {
    fn view(&mut self) -> Widget<Clicker> {
        let add = button("+")
            .label_size(24)
            .on_press(|app: &mut Clicker, _| {
                app.count += app.add;
            })
            .build();
        let upgrade = button(&format!("Upgrade \nCost: {}", self.cost))
            .label_size(18)
            .on_press(|app: &mut Clicker, _| {
                if app.count >= app.cost {
                    app.count -= app.cost;
                    app.add *= 2;
                    app.cost *= 2;
                }
            })
            .build();
        let count = text(&self.count.to_string()).size(24).build();
        let buttons = hstack!(add, upgrade).spacing(10).build();
        vstack!(count, buttons).spacing(10).build()
    }
}
