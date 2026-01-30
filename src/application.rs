use std::marker::PhantomData;

use crate::core::{
    Widget,
    backend::Backend,
    color::Color,
    window::{level::WindowLevel, theme::Theme},
};
use crate::shell::{Application, Program, Renderer};
use parley::{FontContext, LayoutContext};
use vello::{Scene, util::RenderContext};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize, Size},
    event_loop::{ControlFlow, EventLoop},
    window::{Theme as WinitTheme, WindowAttributes, WindowLevel as WinitWindowLevel},
};

// Helper to start app
pub fn start<App>(app: App, view_fn: fn(&mut App) -> Widget<App>) -> Run<App> {
    Run::new(app, view_fn)
}

struct WindowSettings {
    attributes: WindowAttributes,
    background: Color,
    vsync: bool,
}

pub struct Run<App: 'static> {
    user_struct: App,
    window_settings: WindowSettings,
    view_fn: fn(&mut App) -> Widget<App>,
    backend: Backend,
    _marker: PhantomData<App>,
}

impl<App: 'static> Run<App> {
    pub fn new(user_struct: App, view_fn: fn(&mut App) -> Widget<App>) -> Self {
        Self {
            user_struct: user_struct,
            window_settings: WindowSettings {
                attributes: WindowAttributes::default().with_title("GlazeUI"),
                background: Color::rgb(0, 0, 0),
                vsync: true,
            },
            view_fn: view_fn,
            backend: Backend::Auto,
            _marker: PhantomData,
        }
    }

    pub fn backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }

    pub fn title(mut self, name: &str) -> Self {
        self.window_settings.attributes = self.window_settings.attributes.with_title(name);
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.window_settings.background = background;
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.window_settings.attributes =
            self.window_settings
                .attributes
                .with_inner_size(Size::Physical(PhysicalSize {
                    width: width,
                    height: height,
                }));
        self
    }

    pub fn max_size(mut self, width: u32, height: u32) -> Self {
        self.window_settings.attributes =
            self.window_settings
                .attributes
                .with_max_inner_size(Size::Physical(PhysicalSize {
                    width: width,
                    height: height,
                }));
        self
    }

    pub fn min_size(mut self, width: u32, height: u32) -> Self {
        self.window_settings.attributes =
            self.window_settings
                .attributes
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
        self.window_settings.attributes = self.window_settings.attributes.with_window_level(level);
        self
    }

    pub fn blur(mut self, blur: bool) -> Self {
        self.window_settings.attributes = self.window_settings.attributes.with_blur(blur);
        self
    }

    pub fn transparent(mut self, transparent: bool) -> Self {
        self.window_settings.attributes = self
            .window_settings
            .attributes
            .with_transparent(transparent);
        self
    }

    pub fn decorations(mut self, decorations: bool) -> Self {
        self.window_settings.attributes = self
            .window_settings
            .attributes
            .with_decorations(decorations);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.window_settings.attributes = self.window_settings.attributes.with_resizable(resizable);
        self
    }

    pub fn vsync(mut self, vsync: bool) -> Self {
        self.window_settings.vsync = vsync;
        self
    }

    // The theme of titlebar
    pub fn theme(mut self, theme: Theme) -> Self {
        let theme = match theme {
            Theme::Dark => WinitTheme::Dark,
            Theme::Light => WinitTheme::Light,
        };
        self.window_settings.attributes = self.window_settings.attributes.with_theme(Some(theme));
        self
    }

    // Function to run the app
    pub fn run(self) -> crate::Result {
        let event_loop = EventLoop::new().unwrap();

        match self.window_settings.vsync {
            true => event_loop.set_control_flow(ControlFlow::Wait),
            false => event_loop.set_control_flow(ControlFlow::Poll),
        };

        let mut context = RenderContext::new();
        let backends = if self.backend == Backend::OpenGL {
            wgpu::Backends::GL
        } else {
            wgpu::Backends::PRIMARY
        };
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends,
            ..Default::default()
        });
        context.instance = instance;

        let mut window = Program::<App> {
            window: None,
            window_attributes: self.window_settings.attributes,
            renderer: Renderer {
                context,
                scene: Scene::new(),
                surface: None,
                renderers: vec![],
                backend: Some(self.backend),
                font_context: Some(FontContext::new()),
                layout_context: Some(LayoutContext::new()),
                layout: None,
                vsync: self.window_settings.vsync,
            },
            application: Application {
                user_struct: self.user_struct,
                view_fn: Some(self.view_fn),
                background: self.window_settings.background,
                position: PhysicalPosition::new(0.0, 0.0),
            },
        };

        match event_loop.run_app(&mut window) {
            Ok(()) => return Ok(()),
            Err(e) => return Err(e),
        }
    }
}
