use std::{collections::HashMap, marker::PhantomData};

use glyphon::FontSystem;

use crate::core::widget::Widget;

#[derive(Clone, Debug)]
pub struct LayoutWidget {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub parent_width: f32,
    pub parent_height: f32,
}

pub struct LayoutEngine<App> {
    nodes: HashMap<u64, LayoutWidget>,
    _marker: PhantomData<App>,
}

impl<App> LayoutEngine<App> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            _marker: PhantomData,
        }
    }

    pub fn get(&self, id: &u64) -> Option<&LayoutWidget> {
        self.nodes.get(id)
    }

    pub fn compute(
        &mut self,
        _widget: &Widget<App>,
        _width: f32,
        _height: f32,
        _font_system: &mut Option<FontSystem>,
    ) {
        todo!()
    }
}
