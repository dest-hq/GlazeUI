use std::{collections::HashMap, marker::PhantomData};

use glazeui_core::{Widget, WidgetElement};
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
        node: &Widget<App>,
        parent_x: f32,
        parent_y: f32,
        available_width: f32,
        available_height: f32,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext,
    ) {
        match &node.element {
            WidgetElement::VStack { spacing, children } => {
                self.layout_vstack(
                    node.id,
                    children,
                    parent_x,
                    parent_y,
                    available_height,
                    available_width,
                    *spacing,
                    font_cx,
                    layout_cx,
                );
            }
            WidgetElement::HStack { spacing, children } => {
                self.layout_hstack(
                    node.id,
                    children,
                    parent_x,
                    parent_y,
                    available_height,
                    available_width,
                    *spacing,
                    font_cx,
                    layout_cx,
                );
            }
            WidgetElement::Spacer { height, width } => {
                let spacer_node = LayoutNode {
                    x: parent_x,
                    y: parent_y,
                    width: *width,
                    height: *height,
                    parent_width: available_width,
                    parent_height: available_height,
                };
                self.nodes.insert(node.id, spacer_node);
            }
            WidgetElement::Container {
                child,
                width,
                height,
                ..
            } => {
                let container_node = LayoutNode {
                    x: parent_x,
                    y: parent_y,
                    width: *width,
                    height: *height,
                    parent_width: available_width,
                    parent_height: available_height,
                };
                self.nodes.insert(node.id, container_node);

                // Layout the child inside the container
                self.resolve_node(
                    child, parent_x, parent_y, *width, *height, font_cx, layout_cx,
                );
            }
            WidgetElement::Text {
                content, font_size, ..
            } => {
                let (width, height) =
                    measure_text(font_cx, content, *font_size as f32, 1.0, layout_cx);

                let text_node = LayoutNode {
                    x: parent_x,
                    y: parent_y,
                    width: width.min(available_width),
                    height: height,
                    parent_height: available_height,
                    parent_width: available_width,
                };
                self.nodes.insert(node.id, text_node);
            }
        }
    }

    /// Layout children vertically (VStack)
    fn layout_vstack(
        &mut self,
        node_id: u64,
        children: &Vec<Widget<App>>,
        parent_x: f32,
        parent_y: f32,
        available_height: f32,
        available_width: f32,
        spacing: f32,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext,
    ) {
        let mut current_y = parent_y;
        let mut total_height = 0.0;
        let mut max_width = 0.0;

        // Layout all children
        for child in children {
            // Layout the child
            self.resolve_node(
                &child,
                parent_x,
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
        self.nodes.insert(node_id, vstack_node);
    }

    /// Layout children horizontally (HStack)
    fn layout_hstack(
        &mut self,
        node_id: u64,
        children: &Vec<Widget<App>>,
        parent_x: f32,
        parent_y: f32,
        available_height: f32,
        available_width: f32,
        spacing: f32,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext,
    ) {
        let mut current_x = parent_x;
        let mut total_width = 0.0;
        let mut max_height = 0.0;

        // Layout all children
        for child in children {
            // Layout the child
            self.resolve_node(
                &child,
                current_x,
                parent_y,
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
        self.nodes.insert(node_id, hstack_node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct App {}

    #[test]
    fn it_works() {
        let mut font_cx = FontContext::new();
        let mut layout_cx = LayoutContext::new();
        // Spacer test
        let spacer_widget: Widget<App> = Widget::new(
            2,
            WidgetElement::Spacer {
                height: 300.0,
                width: 300.0,
            },
            None,
        );
        let mut layout: LayoutEngine<App> = LayoutEngine::new();
        layout.compute(&spacer_widget, 700.0, 700.0, &mut font_cx, &mut layout_cx);
        let node_info = layout.get(2).unwrap();

        // Verify parent size
        assert_eq!(node_info.parent_height, 700.0);
        assert_eq!(node_info.parent_width, 700.0);

        // Verify size
        assert_eq!(node_info.height, 300.0);
        assert_eq!(node_info.width, 300.0);

        // Verify pos
        assert_eq!(node_info.x, 0.0);
        assert_eq!(node_info.y, 0.0);

        // VStack test

        let spacer1_widget: Widget<App> = Widget::new(
            1,
            WidgetElement::Spacer {
                height: 300.0,
                width: 300.0,
            },
            None,
        );

        let spacer2_widget = Widget::new(
            2,
            WidgetElement::Spacer {
                height: 300.0,
                width: 300.0,
            },
            None,
        );

        let vstack_widget = Widget::new(
            3,
            WidgetElement::VStack {
                spacing: 20.0,
                children: vec![spacer1_widget, spacer2_widget],
            },
            None,
        );

        let mut layout = LayoutEngine::new();
        layout.compute(&vstack_widget, 700.0, 700.0, &mut font_cx, &mut layout_cx);
        let node_info = layout.get(3).unwrap();

        // Verify parent size
        assert_eq!(node_info.parent_height, 700.0);
        assert_eq!(node_info.parent_width, 700.0);

        // Verify size
        assert_eq!(node_info.height, 620.0);
        assert_eq!(node_info.width, 300.0);

        // Verify pos
        assert_eq!(node_info.x, 0.0);
        assert_eq!(node_info.y, 0.0);

        // Verify childs
        let spacer1_node_info = layout.get(1).unwrap();

        {
            // Verify parent size
            assert_eq!(spacer1_node_info.parent_height, 620.0);
            assert_eq!(spacer1_node_info.parent_width, 300.0);

            // Verify size
            assert_eq!(spacer1_node_info.height, 300.0);
            assert_eq!(spacer1_node_info.width, 300.0);

            // Verify pos
            assert_eq!(spacer1_node_info.x, 0.0);
            assert_eq!(spacer1_node_info.y, 0.0);
        }

        let spacer2_node_info = layout.get(2).unwrap();

        {
            // Verify parent size
            assert_eq!(spacer2_node_info.parent_height, 620.0);
            assert_eq!(spacer2_node_info.parent_width, 300.0);

            // Verify size
            assert_eq!(spacer2_node_info.height, 300.0);
            assert_eq!(spacer2_node_info.width, 300.0);

            // Verify pos
            assert_eq!(spacer2_node_info.x, 0.0);
            assert_eq!(spacer2_node_info.y, 320.0);
        }

        // HStack test

        let spacer1_widget: Widget<App> = Widget::new(
            1,
            WidgetElement::Spacer {
                height: 300.0,
                width: 300.0,
            },
            None,
        );

        let spacer2_widget = Widget::new(
            2,
            WidgetElement::Spacer {
                height: 300.0,
                width: 300.0,
            },
            None,
        );

        let hstack_widget = Widget::new(
            3,
            WidgetElement::HStack {
                spacing: 20.0,
                children: vec![spacer1_widget, spacer2_widget],
            },
            None,
        );

        let mut layout = LayoutEngine::new();
        layout.compute(&hstack_widget, 700.0, 700.0, &mut font_cx, &mut layout_cx);
        let node_info = layout.get(3).unwrap();

        // Verify parent size
        assert_eq!(node_info.parent_height, 700.0);
        assert_eq!(node_info.parent_width, 700.0);

        // Verify size
        assert_eq!(node_info.height, 300.0);
        assert_eq!(node_info.width, 620.0);

        // Verify pos
        assert_eq!(node_info.x, 0.0);
        assert_eq!(node_info.y, 0.0);

        // Verify childs
        let spacer1_node_info = layout.get(1).unwrap();

        {
            // Verify parent size
            assert_eq!(spacer1_node_info.parent_height, 300.0);
            assert_eq!(spacer1_node_info.parent_width, 620.0);

            // Verify size
            assert_eq!(spacer1_node_info.height, 300.0);
            assert_eq!(spacer1_node_info.width, 300.0);

            // Verify pos
            assert_eq!(spacer1_node_info.x, 0.0);
            assert_eq!(spacer1_node_info.y, 0.0);
        }

        let spacer2_node_info = layout.get(2).unwrap();

        {
            // Verify parent size
            assert_eq!(spacer2_node_info.parent_height, 300.0);
            assert_eq!(spacer2_node_info.parent_width, 620.0);

            // Verify size
            assert_eq!(spacer2_node_info.height, 300.0);
            assert_eq!(spacer2_node_info.width, 300.0);

            // Verify pos
            assert_eq!(spacer2_node_info.x, 320.0);
            assert_eq!(spacer2_node_info.y, 0.0);
        }
    }
}
