use std::fmt;

use crate::id::next_id;
use crate::style::Style;
mod align;
mod backend;
mod color;
mod helpers;
pub mod id;
mod margin;
mod padding;
pub mod style;
mod text_style;
mod weight;
pub mod widget;
pub mod window;

pub use align::*;
pub use backend::*;
pub use color::*;
pub use helpers::*;
pub use margin::*;
pub use padding::*;
pub use text_style::*;
use vello::peniko::ImageBrush;
pub use weight::*;

/// Widget with a generic Message type
pub struct Widget<M: Clone> {
    /// Unique Id
    pub id: u64,

    /// Type of UI element
    /// ['WidgetElement']
    pub element: WidgetElement<M>,

    /// Callback triggered when the widget is pressed
    pub on_press: Option<M>,

    /// Style of element for layout engine
    pub style: Style,
}

impl<M: Clone> fmt::Debug for Widget<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Widget")
            .field("id", &self.id)
            .field("element", &self.element)
            .finish()
    }
}

impl<M: Clone> Clone for Widget<M> {
    fn clone(&self) -> Self {
        Self {
            id: next_id(),
            element: self.element.clone(),
            on_press: self.on_press.clone(),
            style: self.style.clone(),
        }
    }
}

/// Types of UI elements
pub enum WidgetElement<M: Clone> {
    /// A Rectangle that holds a child
    Container {
        child: Box<Widget<M>>,
        color: (u8, u8, u8, u8),
        radius: u32,
    },

    Text {
        content: String,
        font_size: u32,
        weight: TextWeight,
        style: TextStyle,
        color: (u8, u8, u8, u8),
    },

    Image {
        image: ImageBrush,
    },

    /// Vertical list
    VStack {
        children: Vec<Widget<M>>,
    },

    /// Horizontal list
    HStack {
        children: Vec<Widget<M>>,
    },

    /// Empty space
    Spacer {},
}

// Debug for WidgetElement
impl<M: Clone> fmt::Debug for WidgetElement<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WidgetElement::Container {
                child,
                color,
                radius,
            } => f
                .debug_struct("Container")
                .field("child", child)
                .field("color", color)
                .field("radius", radius)
                .finish(),
            WidgetElement::Text {
                content,
                font_size,
                weight,
                style,
                color,
            } => f
                .debug_struct("Text")
                .field("content", content)
                .field("font_size", font_size)
                .field("weight", weight)
                .field("style", style)
                .field("color", color)
                .finish(),
            WidgetElement::Image { image } => {
                f.debug_struct("image").field("image", image).finish()
            }
            WidgetElement::VStack { children } => f
                .debug_struct("VStack")
                .field("children", children)
                .finish(),
            WidgetElement::HStack { children } => f
                .debug_struct("HStack")
                .field("children", children)
                .finish(),
            WidgetElement::Spacer {} => f.debug_struct("Spacer").finish(),
        }
    }
}

impl<M: Clone> Clone for WidgetElement<M> {
    fn clone(&self) -> Self {
        match self {
            WidgetElement::Image { image } => WidgetElement::Image {
                image: image.clone(),
            },
            WidgetElement::Text {
                content,
                font_size,
                weight,
                style,
                color,
            } => WidgetElement::Text {
                content: content.clone(),
                font_size: *font_size,
                weight: weight.clone(),
                style: style.clone(),
                color: *color,
            },
            WidgetElement::Container {
                child,
                color,
                radius,
            } => WidgetElement::Container {
                child: Box::new((**child).clone()),
                color: *color,
                radius: *radius,
            },
            WidgetElement::VStack { children } => WidgetElement::VStack {
                children: children.iter().map(|c| c.clone()).collect(),
            },
            WidgetElement::HStack { children } => WidgetElement::HStack {
                children: children.iter().map(|c| c.clone()).collect(),
            },
            WidgetElement::Spacer {} => WidgetElement::Spacer {},
        }
    }
}
