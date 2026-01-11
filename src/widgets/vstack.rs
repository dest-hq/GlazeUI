use std::marker::PhantomData;

use crate::{
    core::node::{NodeElement, Widget},
    widgets::utils::types::HorizontalAlign,
};
use taffy::{Rect, Style, prelude::length};

use crate::widgets::utils::{types::Padding, ui_id::next_id};

// Helper to create vstack easier

#[derive(Debug)]
pub struct VStack<Message> {
    _marker: PhantomData<Message>,
    children: Vec<Widget<Message>>,
    spacing: f32,
    padding: Padding,
    // id: Option<u64>,
    align: Option<HorizontalAlign>,
}

impl<Message> VStack<Message> {
    pub fn new(children: Vec<Widget<Message>>) -> Self {
        Self {
            _marker: PhantomData,
            children,
            spacing: 0.0,
            padding: Padding {
                top: 0.0,
                left: 0.0,
                right: 0.0,
                bottom: 0.0,
            },
            // id: None,
            align: None,
        }
    }

    pub fn align(mut self, align: HorizontalAlign) -> Self {
        self.align = Some(align);
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    // pub fn id(mut self, mut id: u64) -> Self {
    //     if id < 1000 {
    //         id = 1000 + id;
    //         println!(
    //             "It is recommended to set the ID above 1,000 to avoid conflicts with widgets where the ID is set automatically. The ID was set automatically: {}",
    //             id
    //         );
    //     }
    //     self.id = Some(id);
    //     self
    // }
}

#[macro_export]
macro_rules! vstack {
    ($($child:expr),*) => {{
        let children = vec![$($child),*];
        glazeui::widgets::vstack::VStack::new(children)
    }};
}

// Transform in widget
impl<Message> From<VStack<Message>> for Widget<Message> {
    fn from(builder: VStack<Message>) -> Widget<Message> {
        let mut widget = Widget {
            id: next_id(),
            element: NodeElement::VStack {
                spacing: builder.spacing,
                children: builder.children,
            },
            on_click: None,
            style: Style::default(),
        };
        widget.style = Style {
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
        if let Some(horizontal_align) = builder.align {
            widget.style.align_items = Some(match horizontal_align {
                HorizontalAlign::Left => taffy::AlignItems::Start,
                HorizontalAlign::Center => taffy::AlignItems::Center,
                HorizontalAlign::Right => taffy::AlignItems::End,
            });
        }
        widget
    }
}
