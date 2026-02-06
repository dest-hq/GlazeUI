use glazeui::{
    application::start,
    core::{
        Color, Widget, button, hstack, spacer, vstack,
        window::{self, Window},
    },
};

fn main() -> glazeui::Result {
    let init = ArrayCount {
        count: 0,
        increment_len_counters: 1,
        decrement_len_counters: 1,
    };

    start(init, ArrayCount::view, ArrayCount::update)
        .title("Array of Counters App")
        .theme(window::Theme::Light) // Titlebar theme
        .background(Color::rgb(255, 255, 255)) // Sorry for flashbang :)
        .run()
}

struct ArrayCount {
    count: i32,
    increment_len_counters: u8,
    decrement_len_counters: u8,
}

#[derive(Clone)]
enum Message {
    AddIncrementCounter,
    AddDecrementCounter,
    Increment,
    Decrement,
}

impl ArrayCount {
    fn update(&mut self, message: Message, _: &mut Window) {
        match message {
            Message::AddDecrementCounter => self.decrement_len_counters += 1,
            Message::AddIncrementCounter => self.increment_len_counters += 1,
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }

    fn view(&mut self) -> Widget<Message, ArrayCount> {
        let add_increment = button("Add Increment Counter")
            .color(Color::rgb(54, 104, 237))
            .width(250)
            .height(50)
            .radius(160)
            .label_size(20)
            .on_press(Message::AddIncrementCounter)
            .build();
        let add_decrement = button("Add Decrement Counter")
            .color(Color::rgb(254, 55, 66))
            .width(250)
            .height(50)
            .radius(160)
            .label_size(20)
            .on_press(Message::AddDecrementCounter)
            .build();

        // Right now, we use a spacing widget because the layout engine doesn't support margins or padding
        let spacing_left = spacer().width(10).build();
        let spacing_top = spacer().height(10).build();
        let space_add_increment = hstack!(spacer().width(20).build(), add_increment).build();
        let space_add_decrement = hstack!(spacer().width(20).build(), add_decrement).build();

        let add_buttons = hstack!(space_add_increment, space_add_decrement).build();

        let mut increment_buttons = hstack!(spacing_left.clone()).spacing(10);
        for _index in 0..self.increment_len_counters {
            let new_button = button(&self.count.to_string())
                .radius(360)
                .width(75)
                .height(75)
                .color(Color::rgb(54, 104, 237))
                .label_size(26)
                .on_press(Message::Increment)
                .build();

            increment_buttons.push(new_button);
        }

        let mut decrement_buttons = hstack!(spacing_left).spacing(10);
        for _index in 0..self.decrement_len_counters {
            let new_button = button(&self.count.to_string())
                .radius(360)
                .width(75)
                .height(75)
                .color(Color::rgb(254, 55, 66))
                .label_size(26)
                .on_press(Message::Decrement)
                .build();

            decrement_buttons.push(new_button);
        }

        vstack!(
            spacing_top,
            add_buttons,
            increment_buttons.build(),
            decrement_buttons.build()
        )
        .spacing(20)
        .build()
    }
}
