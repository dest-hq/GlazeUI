use std::time::Duration;

use glazeui::{
    application::start,
    core::{Widget, button, task::Task, text, vstack, window::Window},
};

fn main() -> glazeui::Result {
    let init = Timer { can_start: true };

    start(init, Timer::view, Timer::update).run()
}

struct Timer {
    can_start: bool,
}

#[derive(Clone)]
enum Message {
    StartedTimer,
    EndedTimer,
}

async fn timer() -> Message {
    std::thread::sleep(Duration::from_secs(3));
    Message::EndedTimer
}

impl Timer {
    fn update(&mut self, message: Message, _: &mut Window) -> Task<Message> {
        match message {
            Message::StartedTimer => {
                if self.can_start {
                    self.can_start = false;
                    Task::new(async { timer().await })
                } else {
                    Task::none()
                }
            }
            Message::EndedTimer => {
                self.can_start = true;
                println!("Timer ended!");
                Task::none()
            }
        }
    }

    fn view(&mut self) -> Widget<Message> {
        let can_start_timer = text(&format!("You can start timer?: {}", self.can_start))
            .size(25)
            .build();
        let start_button = button("Start Timer")
            .on_press(Message::StartedTimer)
            .width(300)
            .height(100)
            .label_size(25)
            .build();

        vstack!(can_start_timer, start_button).spacing(20).build()
    }
}
