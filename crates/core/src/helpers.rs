use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{Widget, color::Color, id::next_id, weight::TextWeight, window::control::Window};

pub fn text<App>(content: &str) -> Text<App> {
    Text::new(content.to_string())
}

pub struct Text<App> {
    pub content: String,
    pub font_size: u32,
    pub weight: TextWeight,
    pub color: Color,
    // pub align: Option<Align>,
    // pub length: Option<Length>,
    _marker: PhantomData<App>,
}

impl<App> Text<App> {
    pub fn new(content: String) -> Self {
        Self {
            content: content,
            font_size: 14,
            weight: TextWeight::NORMAL,
            color: Color::rgb(255, 255, 255),
            // align: None,
            // length: None,
            _marker: PhantomData,
        }
    }

    pub fn size(mut self, font_size: u32) -> Self {
        self.font_size = font_size;
        self
    }

    // pub fn center(mut self) -> Self {
    //     self.align = Some(Align::Center);
    //     self
    // }

    // pub fn align(mut self, align: Align) -> Self {
    //     self.align = Some(align);
    //     self
    // }

    // pub fn length(mut self, length: Length) -> Self {
    //     self.length = Some(length);
    //     self
    // }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn build(self) -> Widget<App> {
        let (r, g, b, a) = (self.color.r, self.color.g, self.color.b, self.color.a);
        return Widget::new(
            next_id(),
            crate::WidgetElement::Text {
                content: self.content,
                font_size: self.font_size,
                line_height: 0.0,
                color: (r, g, b, a),
            },
            None,
        );
    }
}

// VStack
#[macro_export]
macro_rules! vstack {
    ($($child:expr),*) => {{
        let children = vec![$($child),*];
        glazeui::core::VStack::new(children)
    }};
}

#[derive(Debug)]
pub struct VStack<App> {
    pub children: Vec<Widget<App>>,
    pub spacing: f32,
    // pub padding: Padding,
    // pub align: Option<Align>,
    // pub length: Option<Length>,
}

impl<App> VStack<App> {
    pub fn new(children: Vec<Widget<App>>) -> Self {
        Self {
            children,
            spacing: 10.0,
            // padding: Padding {
            //     top: 0.0,
            //     left: 0.0,
            //     right: 0.0,
            //     bottom: 0.0,
            // },
            // // id: None,
            // align: None,
            // length: None,
        }
    }

    pub fn extend(mut self, children: Vec<Widget<App>>) -> Self {
        self.children = children;
        self
    }

    pub fn child(mut self, child: Widget<App>) -> Self {
        self.children.push(child);
        self
    }

    // pub fn align(mut self, align: Align) -> Self {
    //     self.align = Some(align);
    //     self
    // }

    // pub fn length(mut self, length: Length) -> Self {
    //     self.length = Some(length);
    //     self
    // }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    // pub fn padding(mut self, padding: Padding) -> Self {
    //     self.padding = padding;
    //     self
    // }

    pub fn build(self) -> Widget<App> {
        Widget {
            id: next_id(),
            element: crate::WidgetElement::VStack {
                spacing: self.spacing,
                children: self.children,
            },
            on_click: None,
            _marker: PhantomData,
        }
    }
}

pub fn container<App>(child: Widget<App>) -> Container<App> {
    Container::new(child)
}

pub struct Container<App> {
    pub child: Widget<App>,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub radius: f32,
    // pub padding: Padding,
    // pub align: Option<Align>,
    // pub length: Option<Length>,
    pub on_click: Option<Rc<RefCell<dyn FnMut(&mut App, &mut Window)>>>,
}

impl<App> Container<App> {
    pub fn new(child: Widget<App>) -> Self {
        Self {
            child,
            width: 100.0,
            height: 50.0,
            color: Color::rgb(50, 50, 51),
            radius: 0.0,
            // padding: Padding {
            //     top: 0.0,
            //     left: 0.0,
            //     right: 0.0,
            //     bottom: 0.0,
            // },
            // id: None,
            // align: None,
            // length: None,
            on_click: None,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    // pub fn align(mut self, align: Align) -> Self {
    //     self.align = Some(align);
    //     self
    // }

    // pub fn length(mut self, length: Length) -> Self {
    //     self.length = Some(length);
    //     self
    // }

    pub fn radius(mut self, corner_radius: f32) -> Self {
        self.radius = corner_radius;
        self
    }

    // pub fn padding(mut self, padding: Padding) -> Self {
    //     self.padding = padding;
    //     self
    // }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: FnMut(&mut App, &mut Window) + 'static,
    {
        self.on_click = Some(Rc::new(RefCell::new(f)));
        self
    }

    pub fn build(self) -> Widget<App> {
        let (r, g, b, a) = (self.color.r, self.color.g, self.color.b, self.color.a);
        Widget {
            id: next_id(),
            element: crate::WidgetElement::Container {
                child: Box::new(self.child),
                width: self.width,
                height: self.height,
                color: (r, g, b, a),
                radius: self.radius,
            },
            on_click: self.on_click,
            _marker: PhantomData,
        }
    }
}
