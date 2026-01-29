use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::control::Window;
pub mod align;
pub mod color;
pub mod control;
pub mod id;
pub mod padding;
pub mod renderer;
pub mod window;

/// Widget with a generic Message type
pub struct Widget<App> {
    /// Unique Id
    pub id: u64,

    /// Type of UI element
    /// ['WidgetElement']
    pub element: WidgetElement<App>,
    // /// Styles
    // pub style: Style,
    /// Message that will be sent on click
    pub on_click: Option<Rc<RefCell<dyn FnMut(&mut App, &mut Window)>>>,

    _marker: PhantomData<App>,
}

impl<App> fmt::Debug for Widget<App> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Widget")
            .field("id", &self.id)
            .field("element", &self.element)
            // .field("style", &self.style)
            // .field("on_click", &self.on_click.as_ref().map(|_| "<callback>"))
            .finish()
    }
}

impl<App> Clone for Widget<App> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            element: self.element.clone(),
            on_click: None,
            _marker: PhantomData,
            // style: self.style.clone(),
            // on_click: self.on_click.clone(),
        }
    }
}

impl<App> Widget<App> {
    /// Create a new widget
    pub fn new(
        id: u64,
        element: WidgetElement<App>,
        on_click: Option<Rc<RefCell<dyn FnMut(&mut App, &mut Window)>>>,
        // style: Style,
    ) -> Self {
        Self {
            id,
            element,
            on_click: on_click,
            _marker: PhantomData,
            // style: style,
            // on_click,
        }
    }
}

/// Types of UI elements
pub enum WidgetElement<App> {
    /// A container that holds a child
    Container {
        child: Box<Widget<App>>,
        width: f32,
        height: f32,
        color: (u8, u8, u8, u8),
        radius: f32,
    },

    Text {
        content: String,
        font_size: u32,
        line_height: f32,
        // weight: TextWeight,
        color: (u8, u8, u8, u8),
    },

    /// Empty space
    Spacer { height: f32, width: f32 },

    /// Vertical list
    VStack {
        spacing: f32,
        children: Vec<Widget<App>>,
    },

    /// Horizontal list
    HStack {
        spacing: f32,
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
                line_height,
                // weight,
                color,
            } => f
                .debug_struct("Text")
                .field("content", content)
                .field("font_size", font_size)
                .field("line_height", line_height)
                // .field("weight", weight)
                .field("color", color)
                .finish(),
            WidgetElement::Spacer { width, height } => f
                .debug_struct("Spacer")
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
            WidgetElement::Text {
                content,
                font_size,
                line_height,
                // weight,
                color,
            } => WidgetElement::Text {
                content: content.clone(),
                font_size: *font_size,
                line_height: *line_height,
                // weight: weight.clone(),
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
