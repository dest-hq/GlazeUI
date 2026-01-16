use crate::core::widget::{Widget, WidgetElement};
use std::{collections::HashMap, marker::PhantomData};
use taffy::{NodeId, TaffyTree};

#[derive(Debug)]
pub struct LayoutEngine<Message> {
    _marker: PhantomData<Message>,
    pub taffy: TaffyTree,
    pub node_map: HashMap<u64, NodeId>,
    pub layouts: HashMap<u64, ResolvedLayout>,
}

#[derive(Debug)]
pub struct ResolvedLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[allow(clippy::new_without_default)]
impl<Message> LayoutEngine<Message> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
            taffy: TaffyTree::new(),
            node_map: HashMap::new(),
            layouts: HashMap::new(),
        }
    }

    // Compute Layout
    pub fn compute(&mut self, root: &Widget<Message>, width: f32, height: f32) {
        // Build taffy tree
        let root_id = self.build_taffy_tree(root);

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

    fn resolve_node(
        &mut self,
        node: &Widget<Message>,
        taffy_id: NodeId,
        parent_x: f32,
        parent_y: f32,
    ) {
        let layout = self.taffy.layout(taffy_id).unwrap();

        let x = parent_x + layout.location.x;
        let y = parent_y + layout.location.y;
        let w = layout.size.width;
        let h = layout.size.height;

        self.layouts.insert(
            node.id,
            ResolvedLayout {
                x,
                y,
                width: w,
                height: h,
            },
        );

        let children: Vec<&Widget<Message>> = match &node.element {
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

    fn build_taffy_tree(&mut self, node: &Widget<Message>) -> NodeId {
        // Build children
        let child_ids: Vec<NodeId> = match &node.element {
            WidgetElement::Container { child, .. } => vec![self.build_taffy_tree(child)],
            WidgetElement::VStack { children, .. } | WidgetElement::HStack { children, .. } => {
                children
                    .iter()
                    .map(|child| self.build_taffy_tree(child))
                    .collect()
            }
            _ => Vec::new(),
        };

        // Create taffy node
        let taffy_id = if child_ids.is_empty() {
            self.taffy.new_leaf(node.style.clone()).unwrap()
        } else {
            self.taffy
                .new_with_children(node.style.clone(), &child_ids)
                .unwrap()
        };
        // Store mapping
        self.node_map.insert(node.id, taffy_id);

        taffy_id
    }
}
