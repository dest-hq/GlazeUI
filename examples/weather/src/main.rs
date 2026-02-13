use glazeui::{
    application::start,
    core::{Widget, button, task::Task, text, vstack, window::Window},
};
use serde::Deserialize;

fn main() -> glazeui::Result {
    let init = Weather {
        current: CurrentWeather {
            temperature_2m: 0.0,
        },
    };

    start(init, Weather::view, Weather::update).run()
}

async fn fetch_weather() -> Result<Message, String> {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=47&longitude=28.9&current=temperature_2m,wind_speed_10m&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m"; // Moldova, Chisinau
    let response = reqwest::get(url)
        .await
        .map_err(|_| "Error, can't get the temperature".to_string())?;
    if response.status().is_success() {
        let weather_data: Weather = response
            .json()
            .await
            .map_err(|_| "Error parsing json".to_string())?;
        let temperature = weather_data.current.temperature_2m;
        return Ok(Message::UpdateTemperature(temperature));
    } else {
        return Err(format!("Error: {}", response.status()));
    }
}

#[derive(Deserialize)]
struct Weather {
    current: CurrentWeather,
}

#[derive(Deserialize)]
struct CurrentWeather {
    temperature_2m: f64,
}

#[derive(Clone)]
enum Message {
    GetTemperature,
    UpdateTemperature(f64),
}

impl Weather {
    fn update(&mut self, message: Message, _: &mut Window) -> Option<Task<Message>> {
        match message {
            Message::GetTemperature => {
                return Some(Task::new(async { fetch_weather().await.unwrap() }));
            }
            Message::UpdateTemperature(temperature) => {
                self.current.temperature_2m = temperature;
                None
            }
        }
    }

    fn view(&mut self) -> Widget<Message> {
        let temperature = self.current.temperature_2m;
        let temperature = text(&format!("{}°C", temperature)).size(25).build();
        let get_temperature = button("Get Temperature")
            .height(100)
            .width(300)
            .on_press(Message::GetTemperature)
            .label_size(25)
            .build();

        vstack!(temperature, get_temperature).spacing(20).build()
    }
}
