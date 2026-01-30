use crate::{
    Widget,
    widget::{button::Button, container::Container, image_widget::ImageWidget, text::Text},
};

pub fn text<App>(content: &str) -> Text<App> {
    Text::new(content.to_string())
}

#[macro_export]
macro_rules! vstack {
    ($($child:expr),*) => {{
        let children = vec![$($child),*];
        glazeui::core::widget::vstack::VStack::new(children)
    }};
}

#[macro_export]
macro_rules! hstack {
    ($($child:expr),*) => {{
        let children = vec![$($child),*];
        glazeui::core::widget::hstack::HStack::new(children)
    }};
}

pub fn container<App>(child: Widget<App>) -> Container<App> {
    Container::new(child)
}

pub fn button<App>(label: &str) -> Button<App> {
    Button::new(label.to_string())
}

pub fn image<App>() -> ImageWidget<App> {
    ImageWidget::new()
}
