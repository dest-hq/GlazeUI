use crate::{
    Widget,
    widget::{
        button::Button, container::Container, image_widget::ImageWidget, spacer::Spacer, text::Text,
    },
};

pub fn text<M: Clone>(content: &str) -> Text<M> {
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

pub fn container<M: Clone>(child: Widget<M>) -> Container<M> {
    Container::new(child)
}

pub fn button<M: Clone>(label: &str) -> Button<M> {
    Button::new(label.to_string())
}

pub fn spacer<M: Clone>() -> Spacer<M> {
    Spacer::new()
}

pub fn image<M: Clone>() -> ImageWidget<M> {
    ImageWidget::new()
}
