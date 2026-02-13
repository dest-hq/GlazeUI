use std::sync::Arc;
#[cfg(feature = "async")]
use std::sync::mpsc::Receiver;
#[cfg(feature = "async")]
use std::sync::mpsc::Sender;

#[cfg(feature = "async")]
use glazeui_core::task::Task;

use glazeui_core::{Backend, Color, Widget, window::Window};
use glazeui_layout::LayoutEngine;
use glazeui_render::{
    RenderState, Renderer as GlazeuiRenderer, draw::draw, widgets::draw_rect::draw_rectangle,
};
use multirender::{PaintScene, WindowRenderer};
#[cfg(feature = "skia")]
use multirender_skia::SkiaWindowRenderer;
#[cfg(feature = "vello")]
use multirender_vello::VelloWindowRenderer;
#[cfg(feature = "cpu")]
use multirender_vello_cpu::SoftbufferWindowRenderer;
#[cfg(feature = "hybrid")]
use multirender_vello_hybrid::VelloHybridWindowRenderer;
use parley::{FontContext, LayoutContext};
use winit::{
    dpi::PhysicalPosition,
    event_loop::ActiveEventLoop,
    window::{Window as WinitWindow, WindowAttributes},
};

pub mod window;

pub struct Application<M: Clone + Send + 'static, App: 'static> {
    pub user_struct: App,
    pub view_fn: fn(&mut App) -> Widget<M>,
    #[cfg(feature = "async")]
    pub update_fn: fn(&mut App, M, &mut Window) -> Option<Task<M>>,
    #[cfg(not(feature = "async"))]
    pub update_fn: fn(&mut App, M, &mut Window),
    pub background: Color,
    pub position: PhysicalPosition<f64>,
    #[cfg(feature = "async")]
    pub runtime: tokio::runtime::Runtime,
    #[cfg(feature = "async")]
    pub tx: Sender<M>,
    #[cfg(feature = "async")]
    pub rx: Receiver<M>,
}

pub struct Renderer<M: Clone + Send + 'static> {
    pub render_state: RenderState,
    pub backend: Backend,
    pub fallback_backend: Backend,
    pub font_context: FontContext,
    pub layout_context: LayoutContext,
    pub layout: LayoutEngine<M>,
}

pub struct Program<M: Clone + Send + 'static, App: 'static> {
    pub window: Option<Arc<WinitWindow>>,
    pub window_attributes: WindowAttributes,
    pub width: u32,
    pub height: u32,
    pub renderer: Renderer<M>,
    pub application: Application<M, App>,
}

impl<M: Clone + Send + 'static, App: 'static> Program<M, App> {
    fn request_redraw(&mut self) {
        let window = match &self.renderer.render_state {
            RenderState::Active { window, renderer } => {
                if renderer.is_active() {
                    Some(window)
                } else {
                    None
                }
            }
            RenderState::Suspended(_) => None,
        };

        if let Some(window) = window {
            window.request_redraw();
        }
    }

    fn draw_scene<T: PaintScene>(
        scene: &mut T,
        font_context: &mut FontContext,
        layout_context: &mut LayoutContext,
        layout_engine: &mut LayoutEngine<M>,
        scale: f32,
        widget: &Widget<M>,
        background: Color,
        window_size: (u32, u32),
    ) {
        let color = &(background.r, background.g, background.b, background.a);

        draw_rectangle(
            scene,
            0.0,
            color,
            0.0,
            0.0,
            window_size.0 as f64,
            window_size.1 as f64,
        ); // Background

        draw(
            scene,
            font_context,
            layout_context,
            layout_engine,
            scale,
            widget,
        );
    }

    fn set_backend(
        &mut self,
        mut renderer: GlazeuiRenderer,
        fallback_backend: &Backend,
        event_loop: &ActiveEventLoop,
    ) {
        let mut window = match &self.renderer.render_state {
            RenderState::Active { window, .. } => Some(window.clone()),
            RenderState::Suspended(cached_window) => cached_window.clone(),
        };
        let window = window.take().unwrap_or_else(|| {
            Arc::new(
                event_loop
                    .create_window(self.window_attributes.clone())
                    .unwrap(),
            )
        });
        self.window = Some(window.clone());

        if match &mut renderer {
            #[cfg(feature = "cpu")]
            GlazeuiRenderer::CpuSoftbuffer(r) => {
                r.resume(window.clone(), self.width, self.height).is_err()
            }
            #[cfg(feature = "skia")]
            GlazeuiRenderer::Skia(r) => r.resume(window.clone(), self.width, self.height).is_err(),
            #[cfg(feature = "vello")]
            GlazeuiRenderer::Gpu(r) => r.resume(window.clone(), self.width, self.height).is_err(),
            #[cfg(feature = "hybrid")]
            GlazeuiRenderer::Hybrid(r) => {
                r.resume(window.clone(), self.width, self.height).is_err()
            }
            GlazeuiRenderer::Null(r) => r.resume(window.clone(), self.width, self.height).is_err(),
        } {
            let mut fallback_renderer = match fallback_backend {
                #[cfg(feature = "skia")]
                Backend::Skia => GlazeuiRenderer::Skia(Box::new(SkiaWindowRenderer::new())),
                #[cfg(feature = "cpu")]
                Backend::Cpu => {
                    GlazeuiRenderer::CpuSoftbuffer(Box::new(SoftbufferWindowRenderer::new()))
                }
                #[cfg(feature = "vello")]
                Backend::Vello => GlazeuiRenderer::Gpu(Box::new(VelloWindowRenderer::new())),
                #[cfg(feature = "hybrid")]
                Backend::Hybrid => {
                    GlazeuiRenderer::Hybrid(Box::new(VelloHybridWindowRenderer::new()))
                }
            };

            match &mut fallback_renderer {
                #[cfg(feature = "cpu")]
                GlazeuiRenderer::CpuSoftbuffer(r) => {
                    r.resume(window.clone(), self.width, self.height).unwrap();
                }
                #[cfg(feature = "skia")]
                GlazeuiRenderer::Skia(r) => {
                    r.resume(window.clone(), self.width, self.height).unwrap();
                }
                #[cfg(feature = "vello")]
                GlazeuiRenderer::Gpu(r) => {
                    r.resume(window.clone(), self.width, self.height).unwrap();
                }
                #[cfg(feature = "hybrid")]
                GlazeuiRenderer::Hybrid(r) => {
                    r.resume(window.clone(), self.width, self.height).unwrap();
                }
                GlazeuiRenderer::Null(r) => {
                    r.resume(window.clone(), self.width, self.height).unwrap();
                }
            }
            self.renderer.render_state = RenderState::Active {
                window,
                renderer: fallback_renderer,
            };
            println!("Error, switched to fallback");
        } else {
            let renderer = renderer.into();
            self.renderer.render_state = RenderState::Active { window, renderer };
        };
        self.request_redraw();
    }
}
