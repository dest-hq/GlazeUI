use glazeui_core::{Widget, WidgetElement};
use glazeui_layout::LayoutEngine;
use parley::{FontContext, LayoutContext};
use vello::{Scene, peniko::color::AlphaColor};

use crate::widgets::{
    draw_image::draw_image, draw_rectangle::draw_rectangle, draw_text::draw_text,
};

pub mod widgets;

pub fn draw<App>(
    scene: &mut Scene,
    font_context: &mut FontContext,
    layout_context: &mut LayoutContext,
    layout_engine: &mut LayoutEngine<App>,
    scale: f32,
    widget: &Widget<App>,
) {
    let widget_layout = layout_engine.get(widget.id).unwrap();

    // Check if widget is text
    if let WidgetElement::Text {
        content,
        font_size,
        weight,
        color,
    } = &widget.element
    {
        let color = AlphaColor::from_rgba8(color.0, color.1, color.2, color.3);

        draw_text(
            scene,
            widget_layout.x as f64,
            widget_layout.y as f64,
            font_context,
            content,
            color,
            weight,
            *font_size as f32,
            scale,
            layout_context,
        );
    }

    // Check if widget is image
    if let WidgetElement::Image { image, .. } = &widget.element {
        draw_image(scene, image, widget_layout.x as f64, widget_layout.y as f64);
    }

    // Check if widget is container
    if let WidgetElement::Container {
        child,
        color,
        radius,
    } = &widget.element
    {
        let color = AlphaColor::from_rgba8(color.0, color.1, color.2, color.3);

        let width = widget.style.width as f64;
        let height = widget.style.height as f64;

        // Draw container (rectangle)
        draw_rectangle(
            scene,
            *radius as f64,
            color,
            widget_layout.x as f64,
            widget_layout.y as f64,
            width,
            height,
        );

        // Draw container child
        draw(
            scene,
            font_context,
            layout_context,
            layout_engine,
            scale,
            child,
        );
    }

    // Check if widget is vstack
    if let WidgetElement::VStack { children, .. } = &widget.element {
        for child in children.iter() {
            draw(
                scene,
                font_context,
                layout_context,
                layout_engine,
                scale,
                child,
            );
        }
    }

    // Check if widget is hstack
    if let WidgetElement::HStack { children, .. } = &widget.element {
        for child in children.iter() {
            draw(
                scene,
                font_context,
                layout_context,
                layout_engine,
                scale,
                child,
            );
        }
    }
}
