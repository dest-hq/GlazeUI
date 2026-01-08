use crate::core::node::{Node, NodeElement};
use taffy::{Rect, Style, prelude::length};

use crate::widgets::utils::{
    types::Padding,
    ui_id::{next_id, sync_with},
};
// Helper to create vstack easier

#[derive(Debug)]
pub struct VStack {
    children: Vec<Node>,
    spacing: f32,
    padding: Padding,
    id: Option<u64>,
}

impl VStack {
    pub fn new(children: Vec<Node>) -> Self {
        Self {
            children,
            spacing: 0.0,
            padding: Padding {
                top: 0.0,
                left: 0.0,
                right: 0.0,
                bottom: 0.0,
            },
            id: None,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }
}

#[macro_export]
macro_rules! vstack {
    ($($child:expr),*) => {{
        let children = vec![$($child),*];
        glazeui::widgets::vstack::VStack::new(children)
    }};
}

// Transform in Node
impl From<VStack> for Node {
    fn from(builder: VStack) -> Node {
        let id = builder.id.unwrap_or(next_id());
        let mut node = Node {
            id: id,
            element: NodeElement::VStack {
                spacing: builder.spacing,
                children: builder.children,
            },
            style: Style::default(),
        };
        node.style = Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Column,
            gap: taffy::Size {
                width: length(0.0),
                height: length(builder.spacing),
            },
            padding: Rect {
                top: length(builder.padding.top),
                left: length(builder.padding.left),
                right: length(builder.padding.right),
                bottom: length(builder.padding.bottom),
            },
            ..Default::default()
        };
        node
    }
}
