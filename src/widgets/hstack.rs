use std::marker::PhantomData;

use crate::{
    core::node::{NodeElement, Widget},
    widgets::utils::types::VerticalAlign,
};
use taffy::{Rect, Style, prelude::length};

use crate::widgets::utils::{
    types::Padding,
    ui_id::{next_id, sync_with},
};

// Helper to create hstack easier

pub struct HStack<Message> {
    _marker: PhantomData<Message>,
    children: Vec<Widget<Message>>,
    spacing: f32,
    id: Option<u64>,
    padding: Padding,
    align: Option<VerticalAlign>,
}

impl<Message> HStack<Message> {
    pub fn new(children: Vec<Widget<Message>>) -> Self {
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
            _marker: PhantomData,
            align: None,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn id(mut self, mut id: u64) -> Self {
        if id < 1000 {
            id = 1000 + id;
            println!(
                "It is recommended to set the ID above 1,000 to avoid conflicts with widgets where the ID is set automatically. The ID was set automatically: {}",
                id
            );
        }
        self.id = Some(id);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn align(mut self, align: VerticalAlign) -> Self {
        self.align = Some(align);
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

// Transform in Widget
impl<Message> From<HStack<Message>> for Widget<Message> {
    fn from(builder: HStack<Message>) -> Widget<Message> {
        let id = builder.id.unwrap_or(next_id());
        sync_with(id);
        let mut widget = Widget {
            id: id,
            element: NodeElement::HStack {
                spacing: builder.spacing,
                children: builder.children,
            },
            on_click: None,
            style: Style::default(),
        };
        widget.style = Style {
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
        if let Some(vertical_align) = builder.align {
            widget.style.justify_content = Some(match vertical_align {
                VerticalAlign::Top => taffy::JustifyContent::Start,
                VerticalAlign::Center => taffy::JustifyContent::Center,
                VerticalAlign::Bottom => taffy::JustifyContent::End,
            });
        }
        widget
    }
}
