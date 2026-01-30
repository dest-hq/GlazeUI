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
use glazeui::{
    application::start,
    core::{Widget, button, text, vstack},
};

fn main() -> glazeui::Result {
    let init = Clicker { count: 0 };
    start(init, Clicker::view).title("Clicker").run()
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
```
<img width="500" height="500" alt="Clicker" src="https://github.com/user-attachments/assets/7cf293d1-ffbc-498e-928e-f5a9b00ad44a" />
