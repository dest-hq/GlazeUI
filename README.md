<div align="center">
  
# GlazeUI
[![Discord Server](https://img.shields.io/discord/1464217371584757873?label=&labelColor=6A7EC2&logo=discord&logoColor=ffffff&color=7389D8&style=flat-square)](https://discord.gg/DvS6PxAZ4p)
[![Crates.io](https://img.shields.io/crates/v/glazeui.svg?style=flat-square&logo=rust)](https://crates.io/crates/glazeui)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg?style=flat-square)]()

> WARNING
> This is a work in progress; the API changes pretty often

Cross-platform GUI library, written in Rust

Inspired by [Iced](https://github.com/iced-rs/iced) and [Egui](https://github.com/emilk/egui)

</div>

## Features

- **Cross-platform**: GlazeUI supports Windows, macOS* and Linux with rendering using [wgpu](https://github.com/gfx-rs/wgpu) via [vello](https://github.com/linebender/vello)
- Easy to use
 
<sub>* - Untested, but should work</sub>

## Quickstart

```rust
use glazeui::{Error, core::widget::Widget, start, vstack, widgets::text::text};

fn main() -> Result<(), Error> {
    let app = HelloWorld {
        text: "Hello world!".to_string(),
    };

    start(app, HelloWorld::view)
        .title("Hello world!")
        .size(900, 900)
        .run()
}

struct HelloWorld {
    text: String,
}

impl HelloWorld {
    fn view(&mut self) -> Widget<HelloWorld> {
        let hello_world_text = text(&self.text)
            .size(36)
            .build();

        vstack!(hello_world_text
            .align(glazeui::types::Align::Center)
            .length(glazeui::types::Length::Fill) 
            .build()
    }
}
```
