use std::sync::Arc;

use glazeui_core::{Backend, Color, Widget, window::Window};
use glazeui_layout::LayoutEngine;
use parley::{FontContext, LayoutContext};
use vello::{
    Renderer as VelloRenderer, Scene,
    util::{RenderContext, RenderSurface},
};
use winit::{
    dpi::PhysicalPosition,
    window::{Window as WinitWindow, WindowAttributes},
};

pub mod window;

pub struct Application<M: Clone, App: 'static> {
    pub user_struct: App,
    pub view_fn: fn(&mut App) -> Widget<M, App>,
    pub update_fn: fn(&mut App, M, &mut Window),
    pub background: Color,
    pub position: PhysicalPosition<f64>,
}

pub struct Renderer<M: Clone, App> {
    pub context: RenderContext,
    pub scene: Scene,
    pub surface: Option<RenderSurface<'static>>,
    pub renderers: Vec<Option<VelloRenderer>>,
    pub backend: Option<Backend>,
    pub font_context: Option<FontContext>,
    pub layout_context: Option<LayoutContext>,
    pub layout: Option<LayoutEngine<M, App>>,
    pub vsync: bool,
}

pub struct Program<M: Clone, App: 'static> {
    pub window: Option<Arc<WinitWindow>>,
    pub window_attributes: WindowAttributes,
    pub renderer: Renderer<M, App>,
    pub application: Application<M, App>,
}
