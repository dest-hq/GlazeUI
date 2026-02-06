use crate::{
    Widget,
    widget::{
        button::Button, container::Container, image_widget::ImageWidget, spacer::Spacer, text::Text,
    },
};

pub fn text<M: Clone, App>(content: &str) -> Text<M, App> {
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

pub fn container<M: Clone, App>(child: Widget<M, App>) -> Container<M, App> {
    Container::new(child)
}

pub fn button<M: Clone, App>(label: &str) -> Button<M, App> {
    Button::new(label.to_string())
}

pub fn spacer<M: Clone, App>() -> Spacer<M, App> {
    Spacer::new()
}

pub fn image<M: Clone, App>() -> ImageWidget<M, App> {
    ImageWidget::new()
}
