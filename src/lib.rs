use std::{marker::PhantomData, sync::Arc};
pub mod core;
pub mod layout;
pub mod renderer;
pub mod types;
pub mod widgets;
use glyphon::FontSystem;
use layout::LayoutEngine;
use renderer::wgpu::WgpuCtx;
use widgets::utils::ui_id::clear_counter;
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize, Size},
    error::EventLoopError,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{
        Theme as WinitTheme, Window as WinitWindow, WindowAttributes, WindowId,
        WindowLevel as WinitWindowLevel,
    },
};

use crate::{
    core::{ui::Ui, widget::WidgetElement},
    layout::ResolvedLayout,
    types::{Backend, Color, Theme, UserAttention, WindowLevel},
};

pub type Error = EventLoopError;

pub struct Window<'window> {
    window: Arc<WinitWindow>,
    eventloop: &'window ActiveEventLoop,
}

impl<'window> Window<'window> {
    pub fn title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw();
    }

    pub fn request_user_attention(&mut self, attention: UserAttention) {
        let attention = match attention {
            UserAttention::Critical => winit::window::UserAttentionType::Critical,
            UserAttention::Informational => winit::window::UserAttentionType::Informational,
        };
        self.window.request_user_attention(Some(attention));
    }

    pub fn decorations(&mut self, decorations: bool) {
        self.window.set_decorations(decorations);
    }

    pub fn resizable(&mut self, resizable: bool) {
        self.window.set_resizable(resizable);
    }

    pub fn theme(&mut self, theme: Theme) {
        let theme = match theme {
            Theme::Dark => WinitTheme::Dark,
            Theme::Light => WinitTheme::Light,
        };
        self.window.set_theme(Some(theme));
    }

    pub fn close(&mut self) {
        self.eventloop.exit();
    }
}

// Helper to start app
pub fn start<App>(app: App, view_fn: fn(&mut App, &mut Ui<App>)) -> Run<App> {
    Run::new(app, view_fn)
}

pub struct Run<App> {
    app: App,
    window_settings: WindowAttributes,
    vsync: bool,
    view_fn: fn(&mut App, &mut Ui<App>),
    backend: Backend,
    background: Color,
    _marker: PhantomData<App>,
}

impl<App> Run<App> {
    pub fn new(app: App, view_fn: fn(&mut App, &mut Ui<App>)) -> Self {
        Self {
            app: app,
            window_settings: WindowAttributes::default().with_title("GlazeUI"),
            vsync: true,
            view_fn: view_fn,
            backend: Backend::Auto,
            background: Color::rgb(0, 0, 0),
            _marker: PhantomData,
        }
    }

    pub fn backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }

    pub fn title(mut self, name: &str) -> Self {
        self.window_settings = self.window_settings.with_title(name);
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.background = background;
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

    pub fn level(mut self, level: WindowLevel) -> Self {
        let level = match level {
            WindowLevel::AlwaysOnBottom => WinitWindowLevel::AlwaysOnBottom,
            WindowLevel::AlwaysOnTop => WinitWindowLevel::AlwaysOnTop,
            WindowLevel::Normal => WinitWindowLevel::Normal,
        };
        self.window_settings = self.window_settings.with_window_level(level);
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

    pub fn vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
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

        match self.vsync {
            true => event_loop.set_control_flow(ControlFlow::Wait),
            false => event_loop.set_control_flow(ControlFlow::Poll),
        };

        let mut window = UserWindow::<App> {
            window_settings: self.window_settings,
            window: None,
            wgpu_ctx: None,
            backend: Some(self.backend),
            user_app: UserApp {
                app: self.app,
                background: self.background,
                layout: None,
                view_fn: Some(self.view_fn),
                position: PhysicalPosition::new(0.0, 0.0),
                font_system: Some(FontSystem::new()),
            },
        };
        match event_loop.run_app(&mut window) {
            Ok(()) => return Ok(()),
            Err(e) => return Err(e),
        }
    }
}

#[derive(Default)]
struct UserApp<App> {
    app: App,
    view_fn: Option<fn(&mut App, &mut Ui<App>)>,
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
                if let (Some(wgpu_ctx), Some(window)) =
                    (self.wgpu_ctx.as_mut(), self.window.as_ref())
                {
                    let size = window.inner_size();

                    clear_counter();

                    let mut layout = LayoutEngine::new();
                    let mut ui = Ui::new();
                    let _view = self.user_app.view_fn.unwrap();
                    _view(&mut self.user_app.app, &mut ui);

                    // Create vstack widget that will contain all widgets
                    // Like if you write at widgets .show() it will be automatic put in vstack
                    //

                    layout.compute(
                        &ui.widgets[0],
                        size.width as f32,
                        size.height as f32,
                        &mut self.user_app.font_system,
                    );
                    if let Some(font_system) = self.user_app.font_system.as_mut() {
                        wgpu_ctx.draw(
                            &&ui.widgets[0],
                            &layout,
                            font_system,
                            self.user_app.background,
                        );
                    }
                    self.user_app.layout = Some(layout);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    if let Some(window) = self.window.as_ref() {
                        if let Some(layout) = &self.user_app.layout {
                            clear_counter();
                            let mut ui = Ui::new();
                            let _view = self.user_app.view_fn.unwrap();
                            _view(&mut self.user_app.app, &mut ui);
                            let mut user_window = Window {
                                window: self.window.as_ref().unwrap().clone(),
                                eventloop: event_loop,
                            };
                            let layout_resolved = layout.layouts.get(&ui.widgets[0].id).unwrap();

                            let clicked = check_clicked(layout_resolved, self.user_app.position);

                            if clicked {
                                if let WidgetElement::VStack { children, .. }
                                | WidgetElement::HStack { children, .. } = &ui.widgets[0].element
                                {
                                    for child in children {
                                        if let WidgetElement::HStack { children, .. }
                                        | WidgetElement::VStack { children, .. } = &child.element
                                        {
                                            for child in children {
                                                // Get widget information (position, width and height)
                                                let layout_resolved =
                                                    layout.layouts.get(&child.id).unwrap();
                                                // Check if the widget was clicked
                                                let clicked = check_clicked(
                                                    layout_resolved,
                                                    self.user_app.position,
                                                );
                                                if clicked {
                                                    if let Some(callback) = &child.on_click {
                                                        let mut cb = callback.borrow_mut();
                                                        cb(
                                                            &mut self.user_app.app,
                                                            &mut user_window,
                                                        );
                                                        window.request_redraw();
                                                    }
                                                }
                                            }
                                        }

                                        // Get widget information (position, width and height)
                                        let layout_resolved =
                                            layout.layouts.get(&child.id).unwrap();
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
                                } else if let WidgetElement::Container { child, .. } =
                                    &ui.widgets[0].element
                                {
                                    // Get widget information (position, width and height)
                                    let layout_resolved =
                                        layout.layouts.get(&ui.widgets[0].id).unwrap();
                                    // Check if the widget was clicked
                                    let clicked =
                                        check_clicked(layout_resolved, self.user_app.position);
                                    if clicked {
                                        if let Some(callback) = &ui.widgets[0].on_click {
                                            let mut cb = callback.borrow_mut();
                                            cb(&mut self.user_app.app, &mut user_window);
                                            window.request_redraw();
                                        }
                                    } else {
                                        // Check the child of container

                                        // Get widget information (position, width and height)
                                        let layout_resolved =
                                            layout.layouts.get(&child.id).unwrap();
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
                        };
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
