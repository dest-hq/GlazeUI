use crate::{
    core::widget::{Widget, WidgetElement},
    widgets::text::TextWeight,
};
use glyphon::{Attrs, Buffer, FontSystem, Metrics, Shaping, Weight};
use std::{collections::HashMap, marker::PhantomData};
use taffy::{AvailableSpace, Dimension, NodeId, Size, Style, TaffyTree};

#[derive(Debug)]
pub struct LayoutEngine<App> {
    pub taffy: TaffyTree,
    pub node_map: HashMap<u64, NodeId>,
    pub layouts: HashMap<u64, ResolvedLayout>,
    _marker: PhantomData<App>,
}

#[derive(Debug)]
pub struct ResolvedLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[allow(clippy::new_without_default)]
impl<App> LayoutEngine<App> {
    pub fn new() -> Self {
        Self {
            taffy: TaffyTree::new(),
            node_map: HashMap::new(),
            layouts: HashMap::new(),
            _marker: PhantomData,
        }
    }

    // Compute Layout
    pub fn compute(
        &mut self,
        root: &Widget<App>,
        width: f32,
        height: f32,
        font_system: &mut Option<FontSystem>,
    ) {
        // Build taffy tree
        if let Some(font_system) = font_system.as_mut() {
            let root_id = self.build_taffy_tree(root, font_system);

            // Compute layout
            self.taffy
                .compute_layout(
                    root_id,
                    taffy::Size {
                        width: taffy::AvailableSpace::Definite(width),
                        height: taffy::AvailableSpace::Definite(height),
                    },
                )
                .unwrap();

            self.resolve_node(root, root_id, 0.0, 0.0);
        }
    }

    fn resolve_node(&mut self, node: &Widget<App>, taffy_id: NodeId, parent_x: f32, parent_y: f32) {
        let layout = self.taffy.layout(taffy_id).unwrap();

        let w = layout.size.width;
        let h = layout.size.height;

        let x = parent_x + layout.location.x;
        let y = parent_y + layout.location.y;

        self.layouts.insert(
            node.id,
            ResolvedLayout {
                x,
                y,
                width: w,
                height: h,
            },
        );

        let children: Vec<&Widget<App>> = match &node.element {
            WidgetElement::Container { child, .. } => vec![child],
            WidgetElement::VStack { children, .. } | WidgetElement::HStack { children, .. } => {
                children.iter().collect()
            }
            _ => vec![],
        };

        for (child_node, child_taffy) in children.iter().zip(self.taffy.children(taffy_id).unwrap())
        {
            self.resolve_node(child_node, child_taffy, x, y);
        }
    }

    fn build_taffy_tree(&mut self, node: &Widget<App>, font_system: &mut FontSystem) -> NodeId {
        // Build children
        let child_ids: Vec<NodeId> = match &node.element {
            WidgetElement::Container { child, .. } => {
                vec![self.build_taffy_tree(child, font_system)]
            }
            WidgetElement::VStack { children, .. } | WidgetElement::HStack { children, .. } => {
                children
                    .iter()
                    .map(|child| self.build_taffy_tree(child, font_system))
                    .collect()
            }
            _ => Vec::new(),
        };

        let style = if let WidgetElement::Text {
            content,
            font_size,
            line_height,
            weight,
            ..
        } = &node.element
        {
            let weight = match weight {
                TextWeight::THIN => 100,
                TextWeight::EXTRA_LIGHT => 200,
                TextWeight::LIGHT => 300,
                TextWeight::NORMAL => 400,
                TextWeight::MEDIUM => 500,
                TextWeight::SEMIBOLD => 600,
                TextWeight::BOLD => 700,
                TextWeight::EXTRA_BOLD => 800,
                TextWeight::BLACK => 900,
            };

            let metrics = Metrics {
                font_size: *font_size as f32,
                line_height: *line_height as f32,
            };

            // Return the size of the rendered text
            let mut ctx = TextContext::new(
                metrics,
                &content,
                Attrs::new().weight(Weight(weight)),
                font_system,
            );

            let size = ctx.measure(
                Size {
                    width: None,
                    height: None,
                },
                Size {
                    width: AvailableSpace::MaxContent,
                    height: AvailableSpace::MaxContent,
                },
                font_system,
                *line_height,
            );
            Style {
                size: Size {
                    width: Dimension::length(size.width),
                    height: Dimension::length(size.height),
                },
                ..Default::default()
            }
        } else {
            node.style.clone()
        };

        // Create taffy node
        let taffy_id = if child_ids.is_empty() {
            self.taffy.new_leaf(style).unwrap()
        } else {
            self.taffy.new_with_children(style, &child_ids).unwrap()
        };
        // Store mapping
        self.node_map.insert(node.id, taffy_id);

        taffy_id
    }
}

struct TextContext {
    buffer: Buffer,
}

impl TextContext {
    pub fn new(metrics: Metrics, text: &str, attrs: Attrs, font_system: &mut FontSystem) -> Self {
        let mut buffer = Buffer::new_empty(metrics);
        buffer.set_text(
            font_system,
            text,
            &attrs,
            Shaping::Advanced,
            Default::default(),
        );
        Self { buffer }
    }

    pub fn measure(
        &mut self,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<AvailableSpace>,
        font_system: &mut FontSystem,
        line_height: f32,
    ) -> taffy::Size<f32> {
        // Set width constraint
        let width_constraint = known_dimensions.width.or(match available_space.width {
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
            AvailableSpace::Definite(width) => Some(width),
        });

        self.buffer
            .set_size(font_system, width_constraint, Some(line_height));

        // Compute layout
        self.buffer.shape_until_scroll(font_system, false);

        // Determine measured size of text
        let (width, total_lines) = self
            .buffer
            .layout_runs()
            .fold((0.0, 0usize), |(width, total_lines), run| {
                (run.line_w.max(width), total_lines + 1)
            });
        let height = total_lines as f32 * self.buffer.metrics().line_height;

        Size { width, height }
    }
}
