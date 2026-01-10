use std::marker::PhantomData;

use crate::core::node::Widget;

// App implementation
pub trait App {
    /// Type of message
    type Message;

    /// Update the state
    fn update(&mut self, message: Self::Message);

    /// Build the UI
    fn view(&self) -> Element<Self::Message>;
}

pub struct Element<Message> {
    pub widget: Widget<Message>,
    _phantom: PhantomData<Message>,
}

impl<Message> Element<Message> {
    pub fn new(widget: Widget<Message>) -> Self {
        Self {
            widget,
            _phantom: PhantomData,
        }
    }
}
