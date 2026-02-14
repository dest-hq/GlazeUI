#[cfg(feature = "async")]
use glazeui_core::task::Task;
use glazeui_winit::event::UserEvent;

use crate::core::{
    Backend, Color, Widget,
    window::{Theme, WindowLevel},
};
use crate::shell::{Application, Program, Renderer};
use glazeui_core::window::Window;
use glazeui_layout::LayoutEngine;
use glazeui_render::RenderState;
use parley::{FontContext, LayoutContext, fontique::Blob};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize, Size},
    event_loop::EventLoop,
    window::{Theme as WinitTheme, WindowAttributes, WindowLevel as WinitWindowLevel},
};

// Helper to start app
pub fn start<M: Clone + Send + 'static, App>(
    app: App,
    view_fn: fn(&mut App) -> Widget<M>,
    #[cfg(feature = "async")] update_fn: fn(&mut App, M, &mut Window) -> Task<M>,
    #[cfg(not(feature = "async"))] update_fn: fn(&mut App, M, &mut Window),
) -> Run<M, App> {
    Run::new(app, view_fn, update_fn)
}

struct WindowSettings {
    attributes: WindowAttributes,
    background: Color,
}

pub struct Run<M: Clone + Send + 'static, App: 'static> {
    user_struct: App,
    window_settings: WindowSettings,
    view_fn: fn(&mut App) -> Widget<M>,
    #[cfg(feature = "async")]
    update_fn: fn(&mut App, M, &mut Window) -> Task<M>,
    #[cfg(not(feature = "async"))]
    update_fn: fn(&mut App, M, &mut Window),
    backend: Backend,
    fallback_backend: Backend,
}

#[allow(unused)]
fn get_backend() -> Backend {
    #[cfg(feature = "skia")]
    return Backend::Skia;
    #[cfg(feature = "vello")]
    return Backend::Vello;
    #[cfg(feature = "hybrid")]
    return Backend::Hybrid;
    #[cfg(feature = "cpu")]
    return Backend::Cpu;
}

#[allow(unused)]
fn get_fallback_backend() -> Backend {
    #[cfg(feature = "cpu")]
    return Backend::Cpu;
    #[cfg(feature = "skia")]
    return Backend::Skia;
    #[cfg(feature = "vello")]
    return Backend::Vello;
    #[cfg(feature = "hybrid")]
    return Backend::Hybrid;
}

impl<M: Clone + Send + 'static, App: 'static> Run<M, App> {
    pub fn new(
        user_struct: App,
        view_fn: fn(&mut App) -> Widget<M>,
        #[cfg(feature = "async")] update_fn: fn(&mut App, M, &mut Window) -> Task<M>,
        #[cfg(not(feature = "async"))] update_fn: fn(&mut App, M, &mut Window),
    ) -> Self {
        Self {
            user_struct: user_struct,
            window_settings: WindowSettings {
                attributes: WindowAttributes::default()
                    .with_inner_size(Size::Physical(PhysicalSize {
                        width: 800,
                        height: 600,
                    }))
                    .with_title("GlazeUI"),
                background: Color::rgb(0, 0, 0),
            },
            view_fn: view_fn,
            update_fn: update_fn,
            backend: get_backend(),
            fallback_backend: get_fallback_backend(),
        }
    }

    pub fn backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }

    pub fn fallback(mut self, fallback: Backend) -> Self {
        self.fallback_backend = fallback;
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
        let event_loop = EventLoop::<UserEvent<M>>::with_user_event()
            .build()
            .unwrap();
        #[cfg(feature = "async")]
        let proxy = event_loop.create_proxy();

        let size = self
            .window_settings
            .attributes
            .inner_size
            .unwrap()
            .to_physical(1.0);

        let mut font_context = FontContext::new();
        let registred_fallback_font = if font_context.collection.family_names().next().is_none() {
            let font_blob = Blob::from(include_bytes!("assets/fonts/Inter.ttf").to_vec());
            font_context
                .collection
                .register_fonts(font_blob.clone(), None);
            true
        } else {
            false
        };

        #[cfg(feature = "async")]
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let mut program = Program::<M, App> {
            #[cfg(feature = "async")]
            proxy,
            #[cfg(feature = "async")]
            runtime,
            window: None,
            width: size.width,
            height: size.height,
            window_attributes: self.window_settings.attributes,
            renderer: Renderer {
                render_state: RenderState::Suspended(None),
                backend: self.backend,
                fallback_backend: self.fallback_backend,
                font_context: font_context,
                registred_fallback_font,
                layout_context: LayoutContext::new(),
                layout: LayoutEngine::new(),
            },
            application: Application {
                user_struct: self.user_struct,
                view_fn: self.view_fn,
                update_fn: self.update_fn,
                background: self.window_settings.background,
                position: PhysicalPosition::new(0.0, 0.0),
            },
        };

        match event_loop.run_app(&mut program) {
            Ok(()) => return Ok(()),
            Err(e) => return Err(e),
        }
    }
}
