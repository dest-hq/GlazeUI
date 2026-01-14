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
    dpi::{PhysicalPosition, PhysicalSize, Size},
    error::EventLoopError,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Theme as WinitTheme, Window, WindowAttributes, WindowId},
};

use crate::{core::node::NodeElement, layout::ResolvedLayout};

pub type Error = EventLoopError;

pub enum Theme {
    Dark,
    Light,
}

// Helper to start app
pub fn start<A: App>(app: A) -> Run<A> {
    Run::new(app)
}

pub struct Run<A: App> {
    app: A,
    window_settings: WindowAttributes,
}

impl<A: App> Run<A> {
    pub fn new(app: A) -> Self {
        Self {
            app: app,
            window_settings: WindowAttributes::default().with_title("GlazeUI"),
        }
    }

    pub fn title(mut self, name: &str) -> Self {
        self.window_settings = self.window_settings.with_title(name);
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.window_settings = self
            .window_settings
            .with_inner_size(Size::Physical(PhysicalSize {
                width: width,
                height: height,
            }));
        self
    }

    pub fn max_size(mut self, width: u32, height: u32) -> Self {
        self.window_settings =
            self.window_settings
                .with_max_inner_size(Size::Physical(PhysicalSize {
                    width: width,
                    height: height,
                }));
        self
    }

    pub fn min_size(mut self, width: u32, height: u32) -> Self {
        self.window_settings =
            self.window_settings
                .with_min_inner_size(Size::Physical(PhysicalSize {
                    width: width,
                    height: height,
                }));
        self
    }

    pub fn blur(mut self, blur: bool) -> Self {
        self.window_settings = self.window_settings.with_blur(blur);
        self
    }

    pub fn transparent(mut self, transparent: bool) -> Self {
        self.window_settings = self.window_settings.with_transparent(transparent);
        self
    }

    pub fn decorations(mut self, decorations: bool) -> Self {
        self.window_settings = self.window_settings.with_decorations(decorations);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.window_settings = self.window_settings.with_resizable(resizable);
        self
    }

    // The theme of titlebar
    pub fn theme(mut self, theme: Theme) -> Self {
        let theme = match theme {
            Theme::Dark => WinitTheme::Dark,
            Theme::Light => WinitTheme::Light,
        };
        self.window_settings = self.window_settings.with_theme(Some(theme));
        self
    }

    // Function to run the app
    pub fn run(self) -> Result<(), Error> {
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Wait);

        let mut window = UserWindow::<A> {
            window_settings: self.window_settings,
            app: self.app,
            window: None,
            wgpu_ctx: None,
            layout: None,
            position: PhysicalPosition::new(0.0, 0.0),
        };
        match event_loop.run_app(&mut window) {
            Ok(()) => return Ok(()),
            Err(e) => return Err(e),
        }
    }
}

#[derive(Default)]
struct UserWindow<'window, A: App> {
    window: Option<Arc<Window>>,
    wgpu_ctx: Option<WgpuCtx<'window, A::Message>>,
    window_settings: WindowAttributes,
    app: A,
    position: PhysicalPosition<f64>,
    layout: Option<LayoutEngine<A::Message>>,
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
                    let size = window.inner_size();

                    clear_counter();

                    let mut layout: LayoutEngine<<A as App>::Message> = LayoutEngine::new();
                    let element = self.app.view();
                    layout.compute(&element.widget, size.width as f32, size.height as f32);
                    wgpu_ctx.draw(&element.widget, &layout);
                    self.layout = Some(layout);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    if let Some(window) = self.window.as_ref() {
                        if let Some(layout) = &self.layout {
                            clear_counter();
                            let widget = self.app.view();
                            let layout_resolved = layout.layouts.get(&widget.widget.id).unwrap();

                            let clicked = check_clicked(layout_resolved, self.position);

                            if clicked {
                                if let NodeElement::VStack { children, .. }
                                | NodeElement::HStack { children, .. } = widget.widget.element
                                {
                                    for child in children {
                                        if let NodeElement::HStack { children, .. }
                                        | NodeElement::VStack { children, .. } = child.element
                                        {
                                            for child in children {
                                                // Get widget information (position, width and height)
                                                let layout_resolved =
                                                    layout.layouts.get(&child.id).unwrap();
                                                // Check if the widget was clicked
                                                let clicked =
                                                    check_clicked(layout_resolved, self.position);
                                                if clicked {
                                                    if let Some(message) = child.on_click {
                                                        self.app.update(message);
                                                        window.request_redraw();
                                                    }
                                                }
                                            }
                                        }

                                        // Get widget information (position, width and height)
                                        let layout_resolved =
                                            layout.layouts.get(&child.id).unwrap();
                                        // Check if the widget was clicked
                                        let clicked = check_clicked(layout_resolved, self.position);
                                        if clicked {
                                            if let Some(message) = child.on_click {
                                                self.app.update(message);
                                                window.request_redraw();
                                            }
                                        }
                                    }
                                } else if let NodeElement::Container { child, .. } =
                                    widget.widget.element
                                {
                                    // Get widget information (position, width and height)
                                    let layout_resolved =
                                        layout.layouts.get(&widget.widget.id).unwrap();
                                    // Check if the widget was clicked
                                    let clicked = check_clicked(layout_resolved, self.position);
                                    if clicked {
                                        if let Some(message) = child.on_click {
                                            self.app.update(message);
                                            window.request_redraw();
                                        }
                                    } else {
                                        // Check the child of container

                                        // Get widget information (position, width and height)
                                        let layout_resolved =
                                            layout.layouts.get(&child.id).unwrap();
                                        // Check if the widget was clicked
                                        let clicked = check_clicked(layout_resolved, self.position);
                                        if clicked {
                                            if let Some(message) = child.on_click {
                                                self.app.update(message);
                                                window.request_redraw();
                                            }
                                        }
                                    }
                                }
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
