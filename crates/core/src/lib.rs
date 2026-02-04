use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::style::Style;
use crate::window::control::Window;
mod align;
mod backend;
mod color;
mod helpers;
pub mod id;
mod margin;
mod padding;
pub mod style;
mod weight;
pub mod widget;
pub mod window;

pub use align::*;
pub use backend::*;
pub use color::*;
pub use helpers::*;
pub use margin::*;
pub use padding::*;
use vello::peniko::ImageBrush;
pub use weight::*;

/// Widget with a generic Message type
pub struct Widget<App: 'static> {
    /// Unique Id
    pub id: u64,

    /// Type of UI element
    /// ['WidgetElement']
    pub element: WidgetElement<App>,

    /// Callback triggered when the widget is pressed
    pub on_press: Option<Rc<RefCell<dyn FnMut(&mut App, &mut Window)>>>,

    /// Style of element for layout engine
    pub style: Style,

    _marker: PhantomData<App>,
}

impl<App> fmt::Debug for Widget<App> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Widget")
            .field("id", &self.id)
            .field("element", &self.element)
            .field("on_press", &self.on_press.as_ref().map(|_| "<callback>"))
            .finish()
    }
}

impl<App> Clone for Widget<App> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            element: self.element.clone(),
            on_press: self.on_press.clone(),
            style: self.style.clone(),
            _marker: PhantomData,
        }
    }
}

/// Types of UI elements
pub enum WidgetElement<App: 'static> {
    /// A Rectangle that holds a child
    Container {
        child: Box<Widget<App>>,
        color: (u8, u8, u8, u8),
        radius: u32,
    },

    Text {
        content: String,
        font_size: u32,
        weight: TextWeight,
        color: (u8, u8, u8, u8),
    },

    Image {
        image: ImageBrush,
    },

    /// Vertical list
    VStack {
        children: Vec<Widget<App>>,
    },

    /// Horizontal list
    HStack {
        children: Vec<Widget<App>>,
    },

    /// Custom element
    Custom {},
}

// Debug для WidgetElement
impl<App> fmt::Debug for WidgetElement<App> {
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
                color,
            } => f
                .debug_struct("Text")
                .field("content", content)
                .field("font_size", font_size)
                .field("weight", weight)
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
            WidgetElement::Custom {} => f.debug_struct("Custom").finish(),
        }
    }
}

impl<App> Clone for WidgetElement<App> {
    fn clone(&self) -> Self {
        match self {
            WidgetElement::Image { image } => WidgetElement::Image {
                image: image.clone(),
            },
            WidgetElement::Text {
                content,
                font_size,
                weight,
                color,
            } => WidgetElement::Text {
                content: content.clone(),
                font_size: *font_size,
                weight: weight.clone(),
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
            WidgetElement::Custom {} => WidgetElement::Custom {},
        }
    }
}
