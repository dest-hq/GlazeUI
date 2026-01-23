use taffy::{Dimension, Rect, Size, Style, prelude::length};

use crate::{
    core::widget::{Widget, WidgetElement},
    types::{self, Align},
    widgets::{
        button::Button, container::Container, hstack::HStack, spacer::Spacer, text::Text,
        utils::ui_id::next_id, vstack::VStack,
    },
};

// Vstack
pub fn build_vstack<App>(builder: VStack<App>) -> Widget<App> {
    let mut widget = Widget {
        id: next_id(),
        element: WidgetElement::VStack {
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
    if let Some(align) = builder.align {
        // Vertical

        widget.style.align_items = Some(match align {
            Align::TopLeft => taffy::AlignItems::Start,
            Align::CenterLeft => taffy::AlignItems::Start,
            Align::BottomLeft => taffy::AlignItems::Start,

            Align::Top => taffy::AlignItems::Center,
            Align::Center => taffy::AlignItems::Center,
            Align::Bottom => taffy::AlignItems::Center,

            Align::TopRight => taffy::AlignItems::End,
            Align::CenterRight => taffy::AlignItems::End,
            Align::BottomRight => taffy::AlignItems::End,
        });

        // Horizontal

        widget.style.justify_content = Some(match align {
            Align::TopLeft => taffy::JustifyContent::Start,
            Align::CenterLeft => taffy::JustifyContent::Start,
            Align::BottomLeft => taffy::JustifyContent::Start,

            Align::Top => taffy::JustifyContent::Center,
            Align::Center => taffy::JustifyContent::Center,
            Align::Bottom => taffy::JustifyContent::Center,

            Align::TopRight => taffy::JustifyContent::End,
            Align::CenterRight => taffy::JustifyContent::End,
            Align::BottomRight => taffy::JustifyContent::End,
        });
    }

    if let Some(length) = builder.length {
        match length {
            types::Length::Fill => {
                widget.style.size.height = Dimension::percent(1.0);
                widget.style.size.width = Dimension::percent(1.0);
            }
            types::Length::FillHeight => widget.style.size.height = Dimension::percent(1.0),
            types::Length::FillWidth => widget.style.size.width = Dimension::percent(1.0),
            types::Length::Fixed(height, width) => {
                widget.style.size.height = Dimension::length(height);
                widget.style.size.width = Dimension::length(width);
            }
            types::Length::FillPercent(percent) => {
                widget.style.size.height = Dimension::percent(percent as f32 / 100.0);
                widget.style.size.width = Dimension::percent(percent as f32 / 100.0);
            }
        }
    }

    widget
}

// Hstack
pub fn build_hstack<App>(builder: HStack<App>) -> Widget<App> {
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
    if let Some(align) = builder.align {
        // Vertical

        widget.style.align_items = Some(match align {
            Align::TopLeft => taffy::AlignItems::Start,
            Align::CenterLeft => taffy::AlignItems::Start,
            Align::BottomLeft => taffy::AlignItems::Start,

            Align::Top => taffy::AlignItems::Center,
            Align::Center => taffy::AlignItems::Center,
            Align::Bottom => taffy::AlignItems::Center,

            Align::TopRight => taffy::AlignItems::End,
            Align::CenterRight => taffy::AlignItems::End,
            Align::BottomRight => taffy::AlignItems::End,
        });

        // Horizontal

        widget.style.justify_content = Some(match align {
            Align::TopLeft => taffy::JustifyContent::Start,
            Align::CenterLeft => taffy::JustifyContent::Start,
            Align::BottomLeft => taffy::JustifyContent::Start,

            Align::Top => taffy::JustifyContent::Center,
            Align::Center => taffy::JustifyContent::Center,
            Align::Bottom => taffy::JustifyContent::Center,

            Align::TopRight => taffy::JustifyContent::End,
            Align::CenterRight => taffy::JustifyContent::End,
            Align::BottomRight => taffy::JustifyContent::End,
        });
    }

    if let Some(length) = builder.length {
        match length {
            types::Length::Fill => {
                widget.style.size.height = Dimension::percent(1.0);
                widget.style.size.width = Dimension::percent(1.0);
            }
            types::Length::FillHeight => widget.style.size.height = Dimension::percent(1.0),
            types::Length::FillWidth => widget.style.size.width = Dimension::percent(1.0),
            types::Length::Fixed(height, width) => {
                widget.style.size.height = Dimension::length(height);
                widget.style.size.width = Dimension::length(width);
            }
            types::Length::FillPercent(percent) => {
                widget.style.size.height = Dimension::percent(percent as f32 / 100.0);
                widget.style.size.width = Dimension::percent(percent as f32 / 100.0);
            }
        }
    }

    widget
}

// Button
pub fn build_button<App>(builder: Button<App>) -> Widget<App> {
    let mut child_style = Style::default();
    child_style.justify_content = Some(taffy::JustifyContent::Center);
    child_style.align_items = Some(taffy::AlignItems::Center);
    child_style.display = taffy::Display::Flex;
    child_style.flex_direction = taffy::FlexDirection::Column;
    child_style.size.width = Dimension::percent(1.0);
    child_style.size.height = Dimension::percent(1.0);

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
    let mut widget = Widget::new(
        next_id(),
        WidgetElement::Container {
            child: Box::new(child),
            width: builder.width,
            height: builder.height,
            color: (rgba.r, rgba.g, rgba.b, rgba.a),
            radius: builder.radius,
        },
        builder.on_click,
        Style::default(),
    );
    widget.style = Style {
        display: taffy::Display::Flex,
        flex_direction: taffy::FlexDirection::Column,
        gap: taffy::Size {
            width: length(0.0),
            height: length(0.0),
        },
        size: Size {
            width: length(builder.width),
            height: length(builder.height),
        },
        padding: Rect {
            top: length(builder.padding.top),
            left: length(builder.padding.left),
            right: length(builder.padding.right),
            bottom: length(builder.padding.bottom),
        },
        ..Default::default()
    };

    if let Some(align) = builder.align {
        // Vertical

        widget.style.align_items = Some(match align {
            Align::TopLeft => taffy::AlignItems::Start,
            Align::CenterLeft => taffy::AlignItems::Start,
            Align::BottomLeft => taffy::AlignItems::Start,

            Align::Top => taffy::AlignItems::Center,
            Align::Center => taffy::AlignItems::Center,
            Align::Bottom => taffy::AlignItems::Center,

            Align::TopRight => taffy::AlignItems::End,
            Align::CenterRight => taffy::AlignItems::End,
            Align::BottomRight => taffy::AlignItems::End,
        });

        // Horizontal

        widget.style.justify_content = Some(match align {
            Align::TopLeft => taffy::JustifyContent::Start,
            Align::CenterLeft => taffy::JustifyContent::Start,
            Align::BottomLeft => taffy::JustifyContent::Start,

            Align::Top => taffy::JustifyContent::Center,
            Align::Center => taffy::JustifyContent::Center,
            Align::Bottom => taffy::JustifyContent::Center,

            Align::TopRight => taffy::JustifyContent::End,
            Align::CenterRight => taffy::JustifyContent::End,
            Align::BottomRight => taffy::JustifyContent::End,
        });
    }

    if let Some(length) = builder.length {
        match length {
            types::Length::Fill => {
                widget.style.size.height = Dimension::percent(1.0);
                widget.style.size.width = Dimension::percent(1.0);
            }
            types::Length::FillHeight => widget.style.size.height = Dimension::percent(1.0),
            types::Length::FillWidth => widget.style.size.width = Dimension::percent(1.0),
            types::Length::Fixed(height, width) => {
                widget.style.size.height = Dimension::length(height);
                widget.style.size.width = Dimension::length(width);
            }
            types::Length::FillPercent(percent) => {
                widget.style.size.height = Dimension::percent(percent as f32 / 100.0);
                widget.style.size.width = Dimension::percent(percent as f32 / 100.0);
            }
        }
    }

    widget
}

// Spacer
pub fn build_spacer<App>(builder: Spacer<App>) -> Widget<App> {
    let mut widget = Widget {
        id: next_id(),
        element: WidgetElement::Spacer {
            width: builder.width,
            height: builder.height,
        },
        on_click: None,
        style: Style::default(),
    };
    widget.style = Style {
        size: Size {
            width: length(builder.width),
            height: length(builder.height),
        },
        ..Default::default()
    };
    widget
}

// Text
pub fn build_text<App>(builder: Text<App>) -> Widget<App> {
    // Get line height
    let line_height = builder.font_size as f32 * 1.3;
    let rgba = builder.color;
    let mut vstack_style = Style::default();
    vstack_style.display = taffy::Display::Flex;
    vstack_style.flex_direction = taffy::FlexDirection::Column;

    let mut widget = if builder.align.is_some() {
        let child: Widget<App> = Widget::new(
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

        Widget::new(
            next_id(),
            WidgetElement::VStack {
                spacing: 0.0,
                children: vec![child],
            },
            None,
            vstack_style,
        )
    } else {
        Widget::new(
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
        )
    };

    if let Some(align) = builder.align {
        // Vertical

        widget.style.align_items = Some(match align {
            Align::TopLeft => taffy::AlignItems::Start,
            Align::CenterLeft => taffy::AlignItems::Start,
            Align::BottomLeft => taffy::AlignItems::Start,

            Align::Top => taffy::AlignItems::Center,
            Align::Center => taffy::AlignItems::Center,
            Align::Bottom => taffy::AlignItems::Center,

            Align::TopRight => taffy::AlignItems::End,
            Align::CenterRight => taffy::AlignItems::End,
            Align::BottomRight => taffy::AlignItems::End,
        });

        // Horizontal

        widget.style.justify_content = Some(match align {
            Align::TopLeft => taffy::JustifyContent::Start,
            Align::CenterLeft => taffy::JustifyContent::Start,
            Align::BottomLeft => taffy::JustifyContent::Start,

            Align::Top => taffy::JustifyContent::Center,
            Align::Center => taffy::JustifyContent::Center,
            Align::Bottom => taffy::JustifyContent::Center,

            Align::TopRight => taffy::JustifyContent::End,
            Align::CenterRight => taffy::JustifyContent::End,
            Align::BottomRight => taffy::JustifyContent::End,
        });
    }

    if let Some(length) = builder.length {
        match length {
            types::Length::Fill => {
                widget.style.size.height = Dimension::percent(1.0);
                widget.style.size.width = Dimension::percent(1.0);
            }
            types::Length::FillHeight => widget.style.size.height = Dimension::percent(1.0),
            types::Length::FillWidth => widget.style.size.width = Dimension::percent(1.0),
            types::Length::Fixed(height, width) => {
                widget.style.size.height = Dimension::length(height);
                widget.style.size.width = Dimension::length(width);
            }
            types::Length::FillPercent(percent) => {
                widget.style.size.height = Dimension::percent(percent as f32 / 100.0);
                widget.style.size.width = Dimension::percent(percent as f32 / 100.0);
            }
        }
    }

    widget
}

// Container
pub fn build_container<App>(builder: Container<App>) -> Widget<App> {
    let rgba = builder.color;
    let mut widget = Widget::new(
        next_id(),
        WidgetElement::Container {
            child: Box::new(builder.child),
            width: builder.width,
            height: builder.height,
            color: (rgba.r, rgba.g, rgba.b, rgba.a),
            radius: builder.radius,
        },
        builder.on_click,
        Style::default(),
    );
    widget.style = Style {
        display: taffy::Display::Flex,
        flex_direction: taffy::FlexDirection::Column,
        gap: taffy::Size {
            width: length(0.0),
            height: length(0.0),
        },
        size: Size {
            width: length(builder.width),
            height: length(builder.height),
        },
        padding: Rect {
            top: length(builder.padding.top),
            left: length(builder.padding.left),
            right: length(builder.padding.right),
            bottom: length(builder.padding.bottom),
        },
        ..Default::default()
    };

    if let Some(align) = builder.align {
        // Vertical

        widget.style.align_items = Some(match align {
            Align::TopLeft => taffy::AlignItems::Start,
            Align::CenterLeft => taffy::AlignItems::Start,
            Align::BottomLeft => taffy::AlignItems::Start,

            Align::Top => taffy::AlignItems::Center,
            Align::Center => taffy::AlignItems::Center,
            Align::Bottom => taffy::AlignItems::Center,

            Align::TopRight => taffy::AlignItems::End,
            Align::CenterRight => taffy::AlignItems::End,
            Align::BottomRight => taffy::AlignItems::End,
        });

        // Horizontal

        widget.style.justify_content = Some(match align {
            Align::TopLeft => taffy::JustifyContent::Start,
            Align::CenterLeft => taffy::JustifyContent::Start,
            Align::BottomLeft => taffy::JustifyContent::Start,

            Align::Top => taffy::JustifyContent::Center,
            Align::Center => taffy::JustifyContent::Center,
            Align::Bottom => taffy::JustifyContent::Center,

            Align::TopRight => taffy::JustifyContent::End,
            Align::CenterRight => taffy::JustifyContent::End,
            Align::BottomRight => taffy::JustifyContent::End,
        });
    }

    if let Some(length) = builder.length {
        match length {
            types::Length::Fill => {
                widget.style.size.height = Dimension::percent(1.0);
                widget.style.size.width = Dimension::percent(1.0);
            }
            types::Length::FillHeight => widget.style.size.height = Dimension::percent(1.0),
            types::Length::FillWidth => widget.style.size.width = Dimension::percent(1.0),
            types::Length::Fixed(height, width) => {
                widget.style.size.height = Dimension::length(height);
                widget.style.size.width = Dimension::length(width);
            }
            types::Length::FillPercent(percent) => {
                widget.style.size.height = Dimension::percent(percent as f32 / 100.0);
                widget.style.size.width = Dimension::percent(percent as f32 / 100.0);
            }
        }
    }

    widget
}
