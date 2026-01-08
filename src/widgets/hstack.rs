use crate::core::node::{Node, NodeElement};
use taffy::{Rect, Style, prelude::length};

use crate::widgets::utils::{
    types::Padding,
    ui_id::{next_id, sync_with},
};

// Helper to create hstack easier

pub struct HStack {
    children: Vec<Node>,
    spacing: f32,
    id: Option<u64>,
    padding: Padding,
}

#[allow(dead_code)]
pub fn hstack(children: &[Node]) -> HStack {
    HStack::new(children.to_vec())
}

impl HStack {
    pub fn new(children: Vec<Node>) -> Self {
        Self {
            children,
            spacing: 0.0,
            id: None,
            padding: Padding {
                top: 0.0,
                left: 0.0,
                right: 0.0,
                bottom: 0.0,
            },
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }
    pub fn paddint(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
}

#[macro_export]
macro_rules! hstack {
    ($($child:expr),*) => {{
        let children = vec![$($child),*];
        $crate::hstack::HStack::new(children)
    }};
}

// Transform in Node
impl From<HStack> for Node {
    fn from(builder: HStack) -> Node {
        let id = builder.id.unwrap_or(next_id());
        sync_with(id);
        let mut node = Node {
            id: id,
            element: NodeElement::HStack {
                spacing: builder.spacing,
                children: builder.children,
            },
            style: Style::default(),
        };
        node.style = Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Row,
            gap: taffy::Size {
                width: length(builder.spacing),
                height: length(0.0),
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
