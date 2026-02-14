use crate::{
    Widget,
    widget::{
        button::Button, container::Container, image_widget::ImageWidget, label::Label,
        spacer::Spacer,
    },
};

pub fn label<M: Clone + Send + 'static>(content: &str) -> Label<M> {
    Label::new(content.to_string())
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

pub fn container<M: Clone + Send + 'static>(child: Widget<M>) -> Container<M> {
    Container::new(child)
}

pub fn button<M: Clone + Send + 'static>(label: &str) -> Button<M> {
    Button::new(label.to_string())
}

pub fn spacer<M: Clone + Send + 'static>() -> Spacer<M> {
    Spacer::new()
}

pub fn image<M: Clone + Send + 'static>() -> ImageWidget<M> {
    ImageWidget::new()
}
