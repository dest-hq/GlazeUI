use crate::core::widget::Widget;

// App implementation
pub trait Application {
    /// Type of message
    type Message;

    /// Update the state
    fn update(&mut self, message: Self::Message);

    /// Build the UI
    fn view(&self) -> Widget<Self::Message>;
}
