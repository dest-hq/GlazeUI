use glazeui_core::{Node, NodeElement};
use taffy::{Style, prelude::length};

// Helper to create hstack easier

pub struct HStack {
    children: Vec<Node>,
    spacing: f32,
}

pub fn hstack(children: &[Node]) -> HStack {
    HStack::new(children.to_vec())
}

impl HStack {
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
        let mut node = Node {
            id: Some(id),
            element: NodeElement::HStack {
                spacing: self.spacing,
                children: self.children,
            },
            style: Style::default(),
        };
        node.style = Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Row,
            gap: taffy::Size {
                width: length(self.spacing),
                height: length(0.0),
            },
            ..Default::default()
        };
        node
    }

    // Transform in Node without id
    pub fn build(self) -> Node {
        let mut node = Node {
            id: None,
            element: NodeElement::HStack {
                spacing: self.spacing,
                children: self.children,
            },
            style: Style::default(),
        };
        node.style = Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Row,
            gap: taffy::Size {
                width: length(self.spacing),
                height: length(0.0),
            },
            ..Default::default()
        };
        node
    }
}

#[macro_export]
macro_rules! hstack {
    ($($child:expr),*) => {{
        let children = vec![$($child),*];
        $crate::hstack::HStack::new(children)
    }};
}
