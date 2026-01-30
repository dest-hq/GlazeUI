use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::weight::TextWeight;
use crate::window::control::Window;
pub mod align;
pub mod backend;
pub mod color;
mod helpers;
pub mod id;
pub mod padding;
pub mod weight;
pub mod widget;
pub mod window;

pub use helpers::*;
use vello::peniko::ImageBrush;

/// Widget with a generic Message type
pub struct Widget<App: 'static> {
    /// Unique Id
    pub id: u64,

    /// Type of UI element
    /// ['WidgetElement']
    pub element: WidgetElement<App>,

    /// Callback triggered when the widget is pressed
    pub on_press: Option<Rc<RefCell<dyn FnMut(&mut App, &mut Window)>>>,

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
            _marker: PhantomData,
        }
    }
}

impl<App: 'static> Widget<App> {
    /// Create new widget
    pub fn new(
        id: u64,
        element: WidgetElement<App>,
        on_press: Option<Rc<RefCell<dyn FnMut(&mut App, &mut Window)>>>,
    ) -> Self {
        Self {
            id,
            element,
            on_press: on_press,
            _marker: PhantomData,
        }
    }
}

/// Types of UI elements
pub enum WidgetElement<App: 'static> {
    /// A Rectangle that holds a child
    Container {
        child: Box<Widget<App>>,
        width: u32,
        height: u32,
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
        width: u32,
        height: u32,
    },

    /// Empty space
    Spacer { height: u32, width: u32 },

    /// Vertical list
    VStack {
        spacing: i32,
        children: Vec<Widget<App>>,
    },

    /// Horizontal list
    HStack {
        spacing: i32,
        children: Vec<Widget<App>>,
    },
}

// Debug для WidgetElement
impl<App> fmt::Debug for WidgetElement<App> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WidgetElement::Container {
                child,
                width,
                height,
                color,
                radius,
            } => f
                .debug_struct("Container")
                .field("child", child)
                .field("width", width)
                .field("height", height)
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
            WidgetElement::Spacer { width, height } => f
                .debug_struct("Spacer")
                .field("width", width)
                .field("height", height)
                .finish(),
            WidgetElement::Image {
                image,
                width,
                height,
            } => f
                .debug_struct("image")
                .field("image", image)
                .field("width", width)
                .field("height", height)
                .finish(),
            WidgetElement::VStack { spacing, children } => f
                .debug_struct("VStack")
                .field("spacing", spacing)
                .field("children", children)
                .finish(),
            WidgetElement::HStack { spacing, children } => f
                .debug_struct("HStack")
                .field("spacing", spacing)
                .field("children", children)
                .finish(),
        }
    }
}

impl<App> Clone for WidgetElement<App> {
    fn clone(&self) -> Self {
        match self {
            WidgetElement::Spacer { height, width } => WidgetElement::Spacer {
                height: height.clone(),
                width: width.clone(),
            },
            WidgetElement::Image {
                image,
                width,
                height,
            } => WidgetElement::Image {
                image: image.clone(),
                width: *width,
                height: *height,
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
                width,
                height,
                color,
                radius,
            } => WidgetElement::Container {
                child: Box::new((**child).clone()),
                width: *width,
                height: *height,
                color: *color,
                radius: *radius,
            },
            WidgetElement::VStack { children, spacing } => WidgetElement::VStack {
                children: children.iter().map(|c| c.clone()).collect(),
                spacing: *spacing,
            },
            WidgetElement::HStack { children, spacing } => WidgetElement::HStack {
                children: children.iter().map(|c| c.clone()).collect(),
                spacing: *spacing,
            },
        }
    }
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
