use crate::core::node::{Node, NodeElement};
use taffy::{Rect, Size, Style, prelude::length};

use crate::widgets::utils::{
    types::Padding,
    ui_id::{next_id, sync_with},
};

// Helper to create container easier

#[allow(dead_code)]
pub fn container(child: Node) -> Container {
    Container::new(child)
}

pub struct Container {
    child: Node,
    width: f32,
    height: f32,
    color: (u8, u8, u8, u8),
    radius: f32,
    padding: Padding,
    id: Option<u64>,
}

impl Container {
    pub fn new(child: Node) -> Self {
        Self {
            child,
            width: 100.0,
            height: 50.0,
            color: (50, 50, 51, 255),
            radius: 0.0,
            padding: Padding {
                top: 0.0,
                left: 0.0,
                right: 0.0,
                bottom: 0.0,
            },
            id: None,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.color = (r, g, b, a);
        self
    }
    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }
    pub fn radius(mut self, corner_radius: f32) -> Self {
        self.radius = corner_radius;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
}

// Transform in Node
impl From<Container> for Node {
    fn from(builder: Container) -> Node {
        let id = builder.id.unwrap_or(next_id());
        let mut node = Node::new(
            id,
            NodeElement::Container {
                child: Box::new(builder.child),
                width: builder.width,
                height: builder.height,
                color: builder.color,
                radius: builder.radius,
            },
        );
        node.style = Style {
            size: Size {
                width: length(builder.width),
                height: length(builder.height),
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
