use crate::{Margin, Padding, Widget, color::Color, id::next_id, style::Style};

pub struct Container<M: Clone + Send + 'static> {
    pub child: Widget<M>,
    pub width: u32,
    pub height: u32,
    pub color: Color,
    pub radius: u32,
    pub on_press: Option<M>,
    pub margin: Margin,
    pub padding: Padding,
}

impl<M: Clone + Send + 'static> Container<M> {
    pub fn new(child: Widget<M>) -> Self {
        Self {
            child,
            width: 100,
            height: 50,
            color: Color::rgb(50, 50, 51),
            radius: 0,
            on_press: None,
            margin: Margin::new(),
            padding: Padding::new(),
        }
    }

    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
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

    pub fn on_press(mut self, m: M) -> Self {
        self.on_press = Some(m);
        self
    }

    pub fn build(self) -> Widget<M> {
        let (r, g, b, a) = (self.color.r, self.color.g, self.color.b, self.color.a);

        // Container style
        let container_style = Style {
            width: self.width,
            height: self.height,
            padding: self.padding,
            margin: self.margin,
            ..Default::default()
        };

        Widget {
            id: next_id(),
            element: crate::WidgetElement::Container {
                child: Box::new(self.child),
                color: (r, g, b, a),
                radius: self.radius,
            },
            on_press: self.on_press,
            style: container_style,
        }
    }
}
