<div align="center">
  
# GlazeUI
[![Discord Server](https://img.shields.io/discord/1464217371584757873?label=&labelColor=6A7EC2&logo=discord&logoColor=ffffff&color=7389D8&style=flat-square)](https://discord.gg/DvS6PxAZ4p)
[![Crates.io](https://img.shields.io/crates/v/glazeui.svg?style=flat-square&logo=rust)](https://crates.io/crates/glazeui)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg?style=flat-square)]()

> WARNING
> This is a work in progress; the API changes pretty often

Cross-platform GUI library, written in Rust

Inspired by [Iced](https://github.com/iced-rs/iced)

</div>

## Features

- **Cross-platform**: GlazeUI supports Windows, macOS and Linux with rendering using [multirender](https://github.com/dest-hq/multirender)
- Easy to use

## Quickstart

```rust
use glazeui::{
    application::start,
    core::{
        Color, Widget, button, hstack, spacer, vstack,
        window::{self, Window},
    },
};

fn main() -> glazeui::Result {
    let init = Count { count: 0 };

    start(init, Count::view, Count::update)
        .title("Counter App")
        .theme(window::Theme::Light) // Titlebar theme
        .background(Color::rgb(255, 255, 255))
        .run()
}

struct Count {
    count: i32,
}

#[derive(Clone)]
enum Message {
    Increment,
    Decrement,
}

impl Count {
    fn update(&mut self, message: Message, _: &mut Window) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }

    fn view(&mut self) -> Widget<Message> {
        let increment = button(&self.count.to_string())
            .radius(360)
            .width(75)
            .height(75)
            .color(Color::rgb(54, 104, 237))
            .label_size(26)
            .on_press(Message::Increment)
            .build();

        let decrement = button(&self.count.to_string())
            .radius(360)
            .width(75)
            .height(75)
            .color(Color::rgb(254, 55, 66))
            .label_size(26)
            .on_press(Message::Decrement)
            .build();

        let spacing_left = spacer().width(20).build();
        let spacing_top = spacer().height(20).build();

        let buttons = hstack!(spacing_left, increment, decrement)
            .spacing(20)
            .build();

        vstack!(spacing_top, buttons).build()
    }
}
```
<img width="500" height="500" alt="Counter" src="https://github.com/user-attachments/assets/4e9ba8b7-bb97-44a1-99f8-7b264e645a44" />

## License
This library is dual licensed with both the MIT license ([LICENSE-MIT](LICENSE-MIT)) and the Apache-2.0 license ([LICENSE-APACHE](LICENSE-APACHE)), meaning that you can use either the MIT license or the Apache-2.0 license, depending on your needs
