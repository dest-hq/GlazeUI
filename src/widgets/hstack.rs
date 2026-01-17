use std::marker::PhantomData;

use crate::{
    core::widget::{Widget, WidgetElement},
    widgets::utils::types::{HorizontalAlign, VerticalAlign},
};
use taffy::{Dimension, Rect, Size, Style, prelude::length};

use crate::widgets::utils::{types::Padding, ui_id::next_id};

// Helper to create hstack easier

pub struct HStack<Message> {
    _marker: PhantomData<Message>,
    children: Vec<Widget<Message>>,
    spacing: f32,
    // id: Option<u64>,
    padding: Padding,
    vertical_align: Option<VerticalAlign>,
    horizontal_align: Option<HorizontalAlign>,
}

impl<Message> HStack<Message> {
    pub fn new(children: Vec<Widget<Message>>) -> Self {
        Self {
            children,
            spacing: 0.0,
            // id: None,
            padding: Padding {
                top: 0.0,
                left: 0.0,
                right: 0.0,
                bottom: 0.0,
            },
            _marker: PhantomData,
            vertical_align: None,
            horizontal_align: None,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
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

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn vertical_align(mut self, vertical_align: VerticalAlign) -> Self {
        self.vertical_align = Some(vertical_align);
        self
    }

    pub fn horizontal_align(mut self, horizontal_align: HorizontalAlign) -> Self {
        self.horizontal_align = Some(horizontal_align);
        self
    }
}

#[macro_export]
macro_rules! hstack {
    ($($child:expr),*) => {{
        let children = vec![$($child),*];
        glazeui::widgets::hstack::HStack::new(children)
    }};
}

// Transform in Widget
impl<Message> From<HStack<Message>> for Widget<Message> {
    fn from(builder: HStack<Message>) -> Widget<Message> {
        let mut widget = Widget {
            id: next_id(),
            element: WidgetElement::HStack {
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
        if let Some(vertical_align) = builder.vertical_align {
            widget.style.align_items = Some(match vertical_align {
                VerticalAlign::Top => taffy::AlignItems::Start,
                VerticalAlign::Center => taffy::AlignItems::Center,
                VerticalAlign::Bottom => taffy::AlignItems::End,
            });

            widget.style.size = Size {
                width: Dimension::percent(1.0),
                height: Dimension::percent(1.0),
            };
        }
        if let Some(horizontal_align) = builder.horizontal_align {
            widget.style.justify_content = Some(match horizontal_align {
                HorizontalAlign::Left => taffy::JustifyContent::Start,
                HorizontalAlign::Center => taffy::JustifyContent::Center,
                HorizontalAlign::Right => taffy::JustifyContent::End,
            });

            widget.style.size = Size {
                width: Dimension::percent(1.0),
                height: Dimension::percent(1.0),
            };
        }
        widget
    }
}
