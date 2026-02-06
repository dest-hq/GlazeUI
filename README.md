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
use std::path::Path;

use glazeui::{
    application::start,
    core::{Widget, image, text, vstack, window::Window},
};

fn main() -> glazeui::Result {
    let init = Image {};

    start(init, Image::view, Image::update)
        .title("Ferris Image")
        .run()
}

struct Image {}

#[derive(Clone)]
enum Message {}

impl Image {
    fn update(&mut self, _: Message, _: &mut Window) {}
    fn view(&mut self) -> Widget<Message, Image> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("assets")
            .join("ferris.png");
        let ferris_text = text("Ferris").size(35).build();
        let image = image()
            .from_path(path, Some(300), Some(200)) // If one of size's (width, height) is set to None it will be set auto to image native size
            .unwrap()
            .build();
        vstack!(ferris_text, image).spacing(20).build()
    }
}
```
<img width="500" height="500" alt="Ferris" src="https://github.com/user-attachments/assets/226a6194-d575-42e5-bb26-faa822d89d26" />

## License
This library is dual licensed with both the MIT license ([LICENSE-MIT](LICENSE-MIT)) and the Apache-2.0 license ([LICENSE-APACHE](LICENSE-APACHE)) meaning that you can use either the MIT license or the Apache-2.0 license, depending on your needs
