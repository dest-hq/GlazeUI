use std::collections::HashMap;

use glaze_core::{Node, NodeElement};
use taffy::{NodeId, TaffyTree};

pub struct LayoutEngine {
    taffy: TaffyTree,
    node_map: HashMap<u64, NodeId>,
}

#[allow(clippy::new_without_default)]
impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            taffy: TaffyTree::new(),
            node_map: HashMap::new(),
        }
    }

    // Compute Layout
    pub fn compute(&mut self, root: &Node, width: f32, height: f32) {
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
        self.taffy.print_tree(root_id);
    }

    fn build_taffy_tree(&mut self, node: &Node) -> NodeId {
        // Build children
        let child_ids: Vec<NodeId> = match &node.element {
            NodeElement::Container { children } | NodeElement::VStack { children, .. } => children
                .iter()
                .map(|child| self.build_taffy_tree(child))
                .collect(),
            _ => Vec::new(),
        };

        // Create taffy node
        let taffy_id = self
            .taffy
            .new_with_children(node.style.clone(), &child_ids)
            .unwrap();

        // Store mapping
        self.node_map.insert(node.id, taffy_id);

        taffy_id
    }
}
