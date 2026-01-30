use std::sync::Arc;

use glazeui_core::{Widget, backend::Backend, color::Color};
use glazeui_layout::LayoutEngine;
use parley::{FontContext, LayoutContext};
use vello::{
    Renderer as VelloRenderer, Scene,
    util::{RenderContext, RenderSurface},
};
use winit::{
    dpi::PhysicalPosition,
    window::{Window, WindowAttributes},
};

pub mod window;

pub struct Application<App: 'static> {
    pub user_struct: App,
    pub view_fn: Option<fn(&mut App) -> Widget<App>>,
    pub background: Color,
    pub position: PhysicalPosition<f64>,
}

pub struct Renderer<App> {
    pub context: RenderContext,
    pub scene: Scene,
    pub surface: Option<RenderSurface<'static>>,
    pub renderers: Vec<Option<VelloRenderer>>,
    pub backend: Option<Backend>,
    pub font_context: Option<FontContext>,
    pub layout_context: Option<LayoutContext>,
    pub layout: Option<LayoutEngine<App>>,
    pub vsync: bool,
}

pub struct Program<App: 'static> {
    pub window: Option<Arc<Window>>,
    pub window_attributes: WindowAttributes,
    pub renderer: Renderer<App>,
    pub application: Application<App>,
}
