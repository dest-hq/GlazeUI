use std::sync::Arc;

use glazeui_components::container::container;
use glazeui_components::hstack::hstack;
use glazeui_components::{hstack, spacer::spacer, text::text, ui_id::clear_counter, vstack};
use glazeui_core::node::TextWeight;
use glazeui_core::{Node, component::App};
use glazeui_layout::LayoutEngine;
use glazeui_renderer::wgpu_ctx::WgpuCtx;
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

    let mut window = UserWindow {
        window_settings: window_settings,
        ..Default::default()
    };
    let _ = event_loop.run_app(&mut window);
}

#[derive(Default)]
struct UserWindow<'window> {
    window: Option<Arc<Window>>,
    wgpu_ctx: Option<WgpuCtx<'window>>,
    window_settings: WindowAttributes,
    count: i32,
    position: PhysicalPosition<f64>,
}

impl<'window> ApplicationHandler for UserWindow<'window> {
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
            self.count = 0;
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
                            self.count = self.count + 1;
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
                    let text1 = text("Care patratel are culoarea albastra? 🤔")
                        .size(35.0)
                        .weight(TextWeight::MEDIUM)
                        .into();
                    let rectangle = container(spacer().build())
                        .color(155, 252, 0, 255)
                        .size(100.0, 100.0)
                        .radius(15.0)
                        .build();
                    let rectangle2 = container(spacer().build())
                        .color(219, 48, 39, 255)
                        .size(100.0, 100.0)
                        .radius(15.0)
                        .build();
                    let rectangle3 = container(spacer().build())
                        .color(219, 38, 177, 255)
                        .size(100.0, 100.0)
                        .radius(15.0)
                        .build();
                    let rectangle4 = container(spacer().build())
                        .color(26, 36, 224, 255)
                        .size(100.0, 100.0)
                        .radius(15.0)
                        .build();
                    let vstack1 = vstack!(spacer().width(30.0).build(), rectangle, rectangle4)
                        .spacing(20.0)
                        .build();
                    let vstack2 = vstack!(spacer().width(30.0).build(), rectangle2, rectangle3)
                        .spacing(20.0)
                        .build();
                    let hstack1 = hstack!(spacer().width(30.0).build(), vstack1, vstack2)
                        .spacing(20.0)
                        .build();
                    let hstack2 = hstack!(spacer().width(50.0).build(), text1).build();
                    let vstack3 = vstack!(spacer().width(30.0).build(), hstack2, hstack1)
                        .spacing(20.0)
                        .build();
                    layout.compute(&vstack3, size.width as f32, size.height as f32);
                    wgpu_ctx.draw(&vstack3, &layout);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    if let Some(window) = self.window.as_ref() {
                        let py = (1.0 - (-0.5)) * 0.5 * window.inner_size().height as f32;
                        let px = (-0.1 + 1.0) * 0.5 * window.inner_size().width as f32;
                        if self.position.x == px as f64 && self.position.y == py as f64 {
                            self.count = self.count + 1;
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
