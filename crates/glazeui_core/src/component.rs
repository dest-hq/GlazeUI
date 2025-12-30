// Export node
use crate::node;

use std::marker::PhantomData;

pub use node::{Node, NodeElement};

// App implementation
pub trait App: Sized {
    /// Type of message
    type Message;

    /// Create the App
    fn new() -> Self;

    /// Update state
    fn update(&mut self, message: Self::Message);

    /// Build the UI
    fn view(&self) -> Element<Self::Message>;
}

pub struct Element<Message> {
    pub node: Node,
    _phantom: PhantomData<Message>,
}

impl<Message> Element<Message> {
    pub fn new(node: Node) -> Self {
        Self {
            node,
            _phantom: PhantomData,
        }
    }
}
