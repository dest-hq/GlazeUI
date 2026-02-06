use crate::core::{
    Backend, Color, Widget,
    window::{Theme, WindowLevel},
};
use crate::shell::{Application, Program, Renderer};
use glazeui_core::window::Window;
use parley::{FontContext, LayoutContext};
use vello::{Scene, util::RenderContext};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize, Size},
    event_loop::{ControlFlow, EventLoop},
    window::{Theme as WinitTheme, WindowAttributes, WindowLevel as WinitWindowLevel},
};

// Helper to start app
pub fn start<M: Clone, App>(
    app: App,
    view_fn: fn(&mut App) -> Widget<M, App>,
    update_fn: fn(&mut App, M, &mut Window),
) -> Run<M, App> {
    Run::new(app, view_fn, update_fn)
}

struct WindowSettings {
    attributes: WindowAttributes,
    background: Color,
    vsync: bool,
}

pub struct Run<M: Clone, App: 'static> {
    user_struct: App,
    window_settings: WindowSettings,
    view_fn: fn(&mut App) -> Widget<M, App>,
    update_fn: fn(&mut App, M, &mut Window),
    backend: Backend,
}

impl<M: Clone, App: 'static> Run<M, App> {
    pub fn new(
        user_struct: App,
        view_fn: fn(&mut App) -> Widget<M, App>,
        update_fn: fn(&mut App, M, &mut Window),
    ) -> Self {
        Self {
            user_struct: user_struct,
            window_settings: WindowSettings {
                attributes: WindowAttributes::default().with_title("GlazeUI"),
                background: Color::rgb(0, 0, 0),
                vsync: true,
            },
            view_fn: view_fn,
            update_fn: update_fn,
            backend: Backend::Auto,
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
        let backends = match self.backend {
            Backend::Auto => wgpu::Backends::PRIMARY,
            Backend::Vulkan => wgpu::Backends::VULKAN,
            Backend::DX12 => wgpu::Backends::DX12,
            Backend::Metal => wgpu::Backends::METAL,
            Backend::OpenGL => wgpu::Backends::GL,
        };

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends,
            ..Default::default()
        });
        context.instance = instance;

        #[cfg(target_arch = "wasm32")]
        {
            use std::sync::Arc;
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().expect("could not initialize logger");
            use winit::platform::web::WindowExtWebSys;

            #[allow(deprecated)]
            let window = Arc::new(
                event_loop
                    .create_window(WindowAttributes::default())
                    .unwrap(),
            );

            // append canvas
            let canvas = window.canvas().unwrap();
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| body.append_child(canvas.as_ref()).ok())
                .expect("couldn't append canvas to document body");
            drop(web_sys::HtmlElement::from(canvas).focus());

            wasm_bindgen_futures::spawn_local(async move {
                let (width, height, scale_factor) = web_sys::window()
                    .map(|w| {
                        (
                            w.inner_width().unwrap().as_f64().unwrap(),
                            w.inner_height().unwrap().as_f64().unwrap(),
                            w.device_pixel_ratio(),
                        )
                    })
                    .unwrap();

                let size =
                    winit::dpi::PhysicalSize::from_logical::<_, f64>((width, height), scale_factor);
                _ = window.request_inner_size(size);

                let mode = if self.window_settings.vsync {
                    wgpu::PresentMode::AutoVsync
                } else {
                    wgpu::PresentMode::AutoNoVsync
                };

                let surface = context
                    .create_surface(window.clone(), size.width, size.height, mode)
                    .await;

                if let Ok(surface) = surface {
                    let mut program = Program::<App> {
                        window: Some(window),
                        window_attributes: WindowAttributes::default(),
                        renderer: Renderer {
                            context,
                            scene: Scene::new(),
                            surface: Some(surface),
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

                    if let Err(e) = event_loop.run_app(&mut program) {
                        web_sys::console::error_1(&format!("run_app failed: {:?}", e).into());
                    }
                }
            });
            return Ok(());
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut program = Program::<M, App> {
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
}
