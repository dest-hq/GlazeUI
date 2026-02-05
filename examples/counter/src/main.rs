use glazeui::{
    application::start,
    core::{Color, Widget, button, hstack, spacer, vstack, window},
};

fn main() -> glazeui::Result {
    let init = Count { count: 0 };

    start(init, Count::view)
        .title("Counter App")
        .theme(window::Theme::Light) // Titlebar theme
        .background(Color::rgb(255, 255, 255)) // Sorry for flashbang :)
        .run()
}

struct Count {
    count: i32,
}

impl Count {
    fn view(&mut self) -> Widget<Count> {
        let increment = button(&self.count.to_string())
            .radius(360)
            .width(75)
            .height(75)
            .color(Color::rgb(54, 104, 237))
            .label_size(26)
            .on_press(|app: &mut Count, _| app.count += 1)
            .build();

        let decrement = button(&self.count.to_string())
            .radius(360)
            .width(75)
            .height(75)
            .color(Color::rgb(254, 55, 66))
            .label_size(26)
            .on_press(|app: &mut Count, _| app.count -= 1)
            .build();

        // Right now, we use a spacing widget because the layout engine doesn't support margins or padding
        let spacing_left = spacer().width(20).build();
        let spacing_top = spacer().height(20).build();

        let buttons = hstack!(spacing_left, increment, decrement)
            .spacing(20)
            .build();

        vstack!(spacing_top, buttons).build()
    }
}
