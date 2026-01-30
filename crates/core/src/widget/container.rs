use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{Widget, color::Color, id::next_id, window::control::Window};

pub struct Container<App: 'static> {
    pub child: Widget<App>,
    pub width: u32,
    pub height: u32,
    pub color: Color,
    pub radius: u32,
    pub on_press: Option<Rc<RefCell<dyn FnMut(&mut App, &mut Window)>>>,
}

impl<App> Container<App> {
    pub fn new(child: Widget<App>) -> Self {
        Self {
            child,
            width: 100,
            height: 50,
            color: Color::rgb(50, 50, 51),
            radius: 0,
            on_press: None,
        }
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, corner_radius: u32) -> Self {
        self.radius = corner_radius;
        self
    }

    pub fn on_press<F>(mut self, f: F) -> Self
    where
        F: FnMut(&mut App, &mut Window) + 'static,
    {
        self.on_press = Some(Rc::new(RefCell::new(f)));
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
            on_press: self.on_press,
            _marker: PhantomData,
        }
    }
}
