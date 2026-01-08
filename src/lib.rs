use std::sync::Arc;
pub mod core;
pub mod layout;
pub mod renderer;
pub mod widgets;
use core::app::App;
use layout::LayoutEngine;
use renderer::wgpu::WgpuCtx;
use widgets::utils::ui_id::clear_counter;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::Key,
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::{Window, WindowAttributes, WindowId},
};

// Function to run the app
pub fn run<A: App>(_app: A, window_settings: WindowAttributes) {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut window = UserWindow::<A> {
        window_settings: window_settings,
        app: _app,
        window: None,
        wgpu_ctx: None,
        position: PhysicalPosition::new(0.0, 0.0),
    };
    let _ = event_loop.run_app(&mut window);
}
#[derive(Default)]
struct UserWindow<'window, A: App> {
    window: Option<Arc<Window>>,
    wgpu_ctx: Option<WgpuCtx<'window>>,
    window_settings: WindowAttributes,
    app: A,
    position: PhysicalPosition<f64>,
}

impl<'window, A: App> ApplicationHandler for UserWindow<'window, A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let win_attr = self.window_settings.clone();
            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("Creating window error"),
            );
            self.window = Some(window.clone());
            let wgpu_ctx = WgpuCtx::new(window.clone());
            self.wgpu_ctx = Some(wgpu_ctx);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed && !event.repeat {
                    match event.key_without_modifiers().as_ref() {
                        Key::Named(winit::keyboard::NamedKey::Space) => {
                            if let Some(window) = self.window.as_ref() {
                                window.request_redraw();
                            }
                        }
                        _ => (),
                    }
                }
            }
            WindowEvent::Resized(new_size) => {
                if let (Some(wgpu_ctx), Some(window)) =
                    (self.wgpu_ctx.as_mut(), self.window.as_ref())
                {
                    wgpu_ctx.resize((new_size.width, new_size.height));
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(wgpu_ctx), Some(window)) =
                    (self.wgpu_ctx.as_mut(), self.window.as_ref())
                {
                    clear_counter();
                    let size = window.inner_size();

                    let mut layout = LayoutEngine::new();
                    let element = self.app.view().node;
                    layout.compute(&element, size.width as f32, size.height as f32);
                    wgpu_ctx.draw(&element, &layout);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    if let Some(window) = self.window.as_ref() {
                        let py = (1.0 - (-0.5)) * 0.5 * window.inner_size().height as f32;
                        let px = (-0.1 + 1.0) * 0.5 * window.inner_size().width as f32;
                        if self.position.x == px as f64 && self.position.y == py as f64 {
                            if let Some(window) = self.window.as_ref() {
                                window.request_redraw();
                            }
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.position = position;
            }
            _ => (),
        }
    }
}
