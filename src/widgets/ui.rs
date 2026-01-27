use crate::{
    core::widget::{Widget, WidgetElement},
    layout::style::Style,
    widgets::{
        button::Button, container::Container, hstack::HStack, spacer::Spacer, text::Text,
        utils::ui_id::next_id, vstack::VStack,
    },
};

// Vstack
pub fn build_vstack<App>(builder: VStack<App>) -> Widget<App> {
    let widget = Widget {
        id: next_id(),
        element: WidgetElement::VStack {
            spacing: builder.spacing,
            children: builder.children,
        },
        on_click: None,
        style: Style::default(),
    };

    widget
}

// Hstack
pub fn build_hstack<App>(builder: HStack<App>) -> Widget<App> {
    let widget = Widget {
        id: next_id(),
        element: WidgetElement::HStack {
            spacing: builder.spacing,
            children: builder.children,
        },
        on_click: None,
        style: Style::default(),
    };
    widget
}

// Button
pub fn build_button<App>(builder: Button<App>) -> Widget<App> {
    let child_style = Style::default();

    let rgba = builder.label_color;

    let label_child = Widget::new(
        next_id(),
        WidgetElement::Text {
            content: builder.label,
            font_size: builder.label_size,
            line_height: builder.label_size as f32 * 1.3,
            weight: builder.label_weight,
            color: (rgba.a, rgba.g, rgba.b, rgba.a),
        },
        None,
        Style::default(),
    );

    let child = Widget::new(
        next_id(),
        WidgetElement::VStack {
            spacing: 0.0,
            children: vec![label_child],
        },
        None,
        child_style,
    );

    let rgba = builder.color;
    let widget = Widget::new(
        next_id(),
        WidgetElement::Container {
            child: Box::new(child),
            width: builder.width as f32,
            height: builder.height as f32,
            color: (rgba.r, rgba.g, rgba.b, rgba.a),
            radius: builder.radius,
        },
        builder.on_click,
        Style::default(),
    );
    widget
}

// Spacer
pub fn build_spacer<App>(builder: Spacer<App>) -> Widget<App> {
    let widget = Widget {
        id: next_id(),
        element: WidgetElement::Spacer {
            width: builder.width,
            height: builder.height,
        },
        on_click: None,
        style: Style::default(),
    };
    widget
}

// Text
pub fn build_text<App>(builder: Text<App>) -> Widget<App> {
    // Get line height
    let line_height = builder.font_size as f32 * 1.3;
    let rgba = builder.color;

    let widget = Widget::new(
        next_id(),
        WidgetElement::Text {
            content: builder.content,
            font_size: builder.font_size,
            line_height: line_height,
            weight: builder.weight,
            color: (rgba.r, rgba.g, rgba.b, rgba.a),
        },
        None,
        Style::default(),
    );

    widget
}

// Container
pub fn build_container<App>(builder: Container<App>) -> Widget<App> {
    let rgba = builder.color;
    let widget = Widget::new(
        next_id(),
        WidgetElement::Container {
            child: Box::new(builder.child),
            width: builder.width as f32,
            height: builder.height as f32,
            color: (rgba.r, rgba.g, rgba.b, rgba.a),
            radius: builder.radius,
        },
        builder.on_click,
        Style::default(),
    );
    widget
}
