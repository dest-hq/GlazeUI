use std::{collections::HashMap, marker::PhantomData};

use glazeui_core::{Align, Widget, WidgetElement, style::Style};
use parley::{FontContext, LayoutContext};

use crate::measure::text::measure_text;
pub mod measure;

#[derive(Clone, Debug)]
pub struct LayoutNode {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub parent_width: f32,
    pub parent_height: f32,
}

pub struct LayoutEngine<App> {
    nodes: HashMap<u64, LayoutNode>,
    _marker: PhantomData<App>,
}

impl<App> LayoutEngine<App> {
    /// Initialize the layout engine
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            _marker: PhantomData,
        }
    }

    /// Get information of widget (pos, size, parent size) by id
    pub fn get(&self, id: u64) -> Option<&LayoutNode> {
        self.nodes.get(&id)
    }

    /// Compute layout
    pub fn compute(
        &mut self,
        root: &Widget<App>,
        width: f32,
        height: f32,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext,
    ) {
        // Start at (0, 0) with available window size
        self.resolve_node(root, 0.0, 0.0, width, height, font_cx, layout_cx);
    }

    /// Resolve layout for a node and its children
    pub fn resolve_node(
        &mut self,
        widget: &Widget<App>,
        parent_x: f32,
        parent_y: f32,
        available_width: f32,
        available_height: f32,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext,
    ) {
        match &widget.element {
            WidgetElement::VStack { children } => {
                self.layout_vstack(
                    widget.id,
                    &widget.style,
                    children,
                    parent_x,
                    parent_y,
                    available_height,
                    available_width,
                    widget.style.spacing as f32,
                    font_cx,
                    layout_cx,
                );
            }
            WidgetElement::HStack { children } => {
                self.layout_hstack(
                    widget.id,
                    &widget.style,
                    children,
                    parent_x,
                    parent_y,
                    available_height,
                    available_width,
                    widget.style.spacing as f32,
                    font_cx,
                    layout_cx,
                );
            }
            WidgetElement::Custom { .. } => {
                let width = widget.style.width as f32;
                let height = widget.style.height as f32;

                let custom_node = LayoutNode {
                    x: parent_x,
                    y: parent_y,
                    width: width,
                    height: height,
                    parent_width: available_width,
                    parent_height: available_height,
                };
                self.nodes.insert(widget.id, custom_node);
            }
            WidgetElement::Container { child, .. } => {
                let container_width = widget.style.width as f32;
                let container_height = widget.style.height as f32;

                let container_node = LayoutNode {
                    x: parent_x,
                    y: parent_y,
                    width: container_width,
                    height: container_height,
                    parent_width: available_width,
                    parent_height: available_height,
                };
                self.nodes.insert(widget.id, container_node);

                // Layout the child inside the container
                self.resolve_node(
                    child,
                    parent_x,
                    parent_y,
                    container_width,
                    container_height,
                    font_cx,
                    layout_cx,
                );
            }
            WidgetElement::Image { .. } => {
                let width = widget.style.width as f32;
                let height = widget.style.height as f32;

                let image_node = LayoutNode {
                    x: parent_x,
                    y: parent_y,
                    width: width.min(available_width),
                    height: height,
                    parent_height: available_height,
                    parent_width: available_width,
                };
                self.nodes.insert(widget.id, image_node);
            }
            WidgetElement::Text {
                content,
                font_size,
                weight,
                style,
                ..
            } => {
                let (width, height) = measure_text(
                    font_cx,
                    content,
                    weight,
                    style,
                    widget.style.spacing,
                    *font_size as f32,
                    1.0,
                    layout_cx,
                );

                let (x_offset, y_offset) = self.get_align_offset(
                    available_height,
                    available_width,
                    width,
                    height,
                    &widget.style.align,
                );

                let text_node = LayoutNode {
                    x: parent_x + x_offset,
                    y: parent_y + y_offset,
                    width: width.min(available_width),
                    height: height,
                    parent_height: available_height,
                    parent_width: available_width,
                };
                self.nodes.insert(widget.id, text_node);
            }
        }
    }

    /// Get x and y offset for align
    fn get_align_offset(
        &mut self,
        available_height: f32,
        available_width: f32,
        width: f32,
        height: f32,
        align: &Option<Align>,
    ) -> (f32, f32) // x, y
    {
        let align = if let Some(align) = align {
            align
        } else {
            return (0.0, 0.0);
        };

        let y_offset = match align {
            Align::Top | Align::TopLeft | Align::TopRight => 0.0,
            Align::Center | Align::CenterLeft | Align::CenterRight => {
                (available_height - height) / 2.0
            }
            Align::Bottom | Align::BottomLeft | Align::BottomRight => available_height - height,
        };

        let x_offset = match align {
            Align::TopLeft | Align::CenterLeft | Align::BottomLeft => 0.0,
            Align::Top | Align::Center | Align::Bottom => (available_width - width) / 2.0,
            Align::TopRight | Align::CenterRight | Align::BottomRight => available_width - width,
        };

        (x_offset, y_offset)
    }

    /// Layout children vertically (VStack)
    fn layout_vstack(
        &mut self,
        widget_id: u64,
        style: &Style,
        children: &Vec<Widget<App>>,
        parent_x: f32,
        parent_y: f32,
        available_height: f32,
        available_width: f32,
        spacing: f32,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext,
    ) {
        let (x_offset, y_offset) =
            self.get_align_offset(available_height, available_width, 0.0, 0.0, &style.align);

        let mut current_y = parent_y + y_offset;
        let current_x = parent_x + x_offset;
        let mut total_height = 0.0;
        let mut max_width = 0.0;

        // Layout all children
        for child in children {
            // Layout the child
            self.resolve_node(
                &child,
                current_x,
                current_y,
                available_width,
                available_height,
                font_cx,
                layout_cx,
            );

            // Get the child computed height
            if let Some(child_node) = self.nodes.get(&child.id) {
                let child_height = child_node.height;

                current_y += child_height + spacing;
                total_height += child_height + spacing;

                if child_node.width > max_width {
                    max_width = child_node.width;
                }
            }
        }
        // Remove last spacing
        if !children.is_empty() {
            total_height -= spacing;
        }

        for child in children {
            if let Some(child_node) = self.nodes.get_mut(&child.id) {
                child_node.parent_width = max_width;
                child_node.parent_height = total_height;
            }
        }

        // Store the VStack own node
        let vstack_node = LayoutNode {
            x: parent_x,
            y: parent_y,
            width: max_width,
            height: total_height,
            parent_height: available_height,
            parent_width: available_width,
        };
        self.nodes.insert(widget_id, vstack_node);
    }

    /// Layout children horizontally (HStack)
    fn layout_hstack(
        &mut self,
        widget_id: u64,
        style: &Style,
        children: &Vec<Widget<App>>,
        parent_x: f32,
        parent_y: f32,
        available_height: f32,
        available_width: f32,
        spacing: f32,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext,
    ) {
        let (x_offset, y_offset) =
            self.get_align_offset(available_height, available_width, 0.0, 0.0, &style.align);

        let mut current_x = parent_x + x_offset;
        let current_y = parent_y + y_offset;
        let mut total_width = 0.0;
        let mut max_height = 0.0;

        // Layout all children
        for child in children {
            // Layout the child
            self.resolve_node(
                &child,
                current_x,
                current_y,
                available_width,
                available_height,
                font_cx,
                layout_cx,
            );

            // Get the child computed width
            if let Some(child_node) = self.nodes.get(&child.id) {
                let child_width = child_node.width;

                current_x += child_width + spacing;
                total_width += child_width + spacing;

                if child_node.height > max_height {
                    max_height = child_node.height;
                }
            }
        }
        // Remove last spacing
        if !children.is_empty() {
            total_width -= spacing;
        }

        for child in children {
            if let Some(child_node) = self.nodes.get_mut(&child.id) {
                child_node.parent_width = total_width;
                child_node.parent_height = max_height;
            }
        }

        // Store the HStack own node
        let hstack_node = LayoutNode {
            x: parent_x,
            y: parent_y,
            width: total_width,
            height: max_height,
            parent_height: available_height,
            parent_width: available_width,
        };
        self.nodes.insert(widget_id, hstack_node);
    }
}
