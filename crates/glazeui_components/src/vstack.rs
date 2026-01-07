use glazeui_core::{Node, NodeElement};
use taffy::{Style, prelude::length};

use crate::ui_id::{next_id, sync_with};
// Helper to create vstack easier

#[derive(Debug)]
pub struct VStack {
    children: Vec<Node>,
    spacing: f32,
}

impl VStack {
    pub fn new(children: Vec<Node>) -> Self {
        Self {
            children,
            spacing: 0.0,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    // Transform in Node with id
    pub fn build_with(self, id: u64) -> Node {
        sync_with(id + 1);
        let mut node = Node {
            id: id,
            element: NodeElement::VStack {
                spacing: self.spacing,
                children: self.children,
            },
            style: Style::default(),
        };
        node.style = Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Column,
            gap: taffy::Size {
                width: length(0.0),
                height: length(self.spacing),
            },
            ..Default::default()
        };
        node
    }

    // Transform in Node without id
    pub fn build(self) -> Node {
        let mut node = Node {
            id: next_id(),
            element: NodeElement::VStack {
                spacing: self.spacing,
                children: self.children,
            },
            style: Style::default(),
        };
        node.style = Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Column,
            gap: taffy::Size {
                width: length(0.0),
                height: length(self.spacing),
            },
            ..Default::default()
        };
        node
    }
}

#[macro_export]
macro_rules! vstack {
    ($($child:expr),*) => {{
        let children = vec![$($child),*];
        $crate::vstack::VStack::new(children)
    }};
}
