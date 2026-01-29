use core::{Widget, color::Color, renderer::backend::Backend};
use glazeui_core::{WidgetElement, id::clear_counter};
use glazeui_layout::LayoutEngine;
use glyphon::FontSystem;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    error::EventLoopError,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Window as WinitWindow, WindowAttributes, WindowId},
};

pub mod application;

pub mod vello {
    pub use glazeui_vello::*;
}
pub mod layout {
    pub use glazeui_layout::*;
}
pub mod widgets {
    pub use glazeui_widget::*;
}
pub mod core {
    pub use glazeui_core::*;
}
pub mod winitwindow {
    pub use glazeui_winit::*;
}

pub type Error = EventLoopError;
pub type Result = std::result::Result<(), Error>;

#[derive(Default)]
struct UserApp<App> {
    user_struct: App,
    view_fn: Option<fn(&mut App) -> Widget<App>>,
    background: Color,
    font_system: Option<FontSystem>,
    position: PhysicalPosition<f64>,
    layout: Option<LayoutEngine<App>>,
}

#[derive(Default)]
struct UserWindow<'window, App> {
    window: Option<Arc<WinitWindow>>,
    wgpu_ctx: Option<WgpuCtx<'window, App>>,
    backend: Option<Backend>,
    window_settings: WindowAttributes,
    user_app: UserApp<App>,
}

impl<'window, App> ApplicationHandler for UserWindow<'window, App> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let win_attr = self.window_settings.clone();
            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("Creating window error"),
            );
            self.window = Some(window.clone());
            let backend = if let Some(backend) = self.backend.clone() {
                backend
            } else {
                Backend::Auto
            };
            let wgpu_ctx = WgpuCtx::new(window.clone(), backend);
            self.wgpu_ctx = Some(wgpu_ctx);
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if event_loop.control_flow() == ControlFlow::Poll {
            if let Some(window) = self.window.as_ref() {
                window.request_redraw();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
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
                if let (Some(wgpu_ctx), Some(window), Some(view)) = (
                    self.wgpu_ctx.as_mut(),
                    self.window.as_ref(),
                    self.user_app.view_fn.as_ref(),
                ) {
                    let size = window.inner_size();

                    clear_counter();

                    let mut layout = LayoutEngine::new();
                    let ui = view(&mut self.user_app.user_struct);

                    // Create vstack widget that will contain all widgets
                    // Like if you write at widgets .show() it will be automatic put in vstack
                    //

                    layout.compute(&ui, size.width as f32, size.height as f32);
                    if let Some(font_system) = self.user_app.font_system.as_mut() {
                        wgpu_ctx.draw(&&ui, &layout, font_system, self.user_app.background);
                    }
                    self.user_app.layout = Some(layout);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    if let (Some(window), Some(view), Some(layout)) = (
                        self.window.as_ref(),
                        self.user_app.view_fn.as_ref(),
                        self.user_app.layout.as_ref(),
                    ) {
                        clear_counter();
                        let ui = view(&mut self.user_app.user_struct);
                        let mut user_window = Window {
                            window: self.window.as_ref().unwrap().clone(),
                            background: &mut self.user_app.background,
                            eventloop: event_loop,
                        };
                        let layout_resolved = layout.get(ui.id).unwrap();

                        let clicked = check_clicked(layout_resolved, self.user_app.position);

                        if clicked {
                            if let WidgetElement::VStack { children, .. }
                            | WidgetElement::HStack { children, .. } = &ui.element
                            {
                                for child in children {
                                    if let WidgetElement::HStack { children, .. }
                                    | WidgetElement::VStack { children, .. } = &child.element
                                    {
                                        for child in children {
                                            // Get widget information (position, width and height)
                                            let layout_resolved = layout.get(child.id).unwrap();
                                            // Check if the widget was clicked
                                            let clicked = check_clicked(
                                                layout_resolved,
                                                self.user_app.position,
                                            );
                                            if clicked {
                                                if let Some(callback) = &child.on_click {
                                                    let mut cb = callback.borrow_mut();
                                                    cb(&mut self.user_app.app, &mut user_window);
                                                    window.request_redraw();
                                                }
                                            }
                                        }
                                    }

                                    // Get widget information (position, width and height)
                                    let layout_resolved = layout.layouts.get(&child.id).unwrap();
                                    // Check if the widget was clicked
                                    let clicked =
                                        check_clicked(layout_resolved, self.user_app.position);
                                    if clicked {
                                        if let Some(callback) = &child.on_click {
                                            let mut cb = callback.borrow_mut();
                                            cb(&mut self.user_app.app, &mut user_window);
                                            window.request_redraw();
                                        }
                                    }
                                }
                            } else if let WidgetElement::Container { child, .. } = &ui.element {
                                // Get widget information (position, width and height)
                                let layout_resolved = layout.layouts.get(&ui.id).unwrap();
                                // Check if the widget was clicked
                                let clicked =
                                    check_clicked(layout_resolved, self.user_app.position);
                                if clicked {
                                    if let Some(callback) = &ui.on_click {
                                        let mut cb = callback.borrow_mut();
                                        cb(&mut self.user_app.app, &mut user_window);
                                        window.request_redraw();
                                    }
                                } else {
                                    // Check the child of container

                                    // Get widget information (position, width and height)
                                    let layout_resolved = layout.layouts.get(&child.id).unwrap();
                                    // Check if the widget was clicked
                                    let clicked =
                                        check_clicked(layout_resolved, self.user_app.position);
                                    if clicked {
                                        if let Some(callback) = &child.on_click {
                                            let mut cb = callback.borrow_mut();
                                            cb(&mut self.user_app.app, &mut user_window);
                                            window.request_redraw();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.user_app.position = position;
            }
            _ => (),
        }
    }
}

fn check_clicked(layout: &ResolvedLayout, click: PhysicalPosition<f64>) -> bool {
    if click.x >= layout.x as f64
        && click.x <= layout.x as f64 + layout.width as f64
        && click.y >= layout.y as f64
        && click.y <= layout.y as f64 + layout.height as f64
    {
        return true;
    }
    false
}
