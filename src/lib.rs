use ::vello::{
    AaConfig, RenderParams, Renderer, RendererOptions, Scene,
    peniko::color::AlphaColor,
    util::{RenderContext, RenderSurface},
};
use core::{Widget, backend::Backend, color::Color};
use glazeui_core::{WidgetElement, id::clear_counter, window::control::Window};
use glazeui_layout::{LayoutEngine, LayoutNode};
use glazeui_vello::draw;
use parley::{FontContext, LayoutContext};
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
    vsync: bool,
    view_fn: Option<fn(&mut App) -> Widget<App>>,
    background: Color,
    font_context: Option<FontContext>,
    layout_context: Option<LayoutContext>,
    position: PhysicalPosition<f64>,
    layout: Option<LayoutEngine<App>>,
}

struct UserWindow<App> {
    window: Option<Arc<WinitWindow>>,
    context: RenderContext,
    scene: Scene,
    surface: Option<RenderSurface<'static>>,
    renderer: Vec<Option<Renderer>>,
    backend: Option<Backend>,
    window_settings: WindowAttributes,
    user_app: UserApp<App>,
}

impl<App> ApplicationHandler for UserWindow<App> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let win_attr = self.window_settings.clone();
            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("Creating window error"),
            );
            self.window = Some(window.clone());
            // Create a vello Surface
            let size = window.inner_size();

            let mode = if self.user_app.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            };

            let surface_future =
                self.context
                    .create_surface(window.clone(), size.width, size.height, mode);
            let surface = pollster::block_on(surface_future).expect("Error creating surface");

            // Create a vello Renderer for the surface (using its device id)
            self.renderer
                .resize_with(self.context.devices.len(), || None);
            self.renderer[surface.dev_id]
                .get_or_insert_with(|| create_vello_renderer(&self.context, &surface));

            self.surface = Some(surface);

            // let backend = if let Some(backend) = self.backend.clone() {
            //     backend
            // } else {
            //     Backend::Auto
            // };
            // self.backend = Some(backend);
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
                if let (Some(window), Some(surface)) = (self.window.as_ref(), self.surface.as_mut())
                {
                    if new_size.width != 0 && new_size.height != 0 {
                        self.context
                            .resize_surface(surface, new_size.width, new_size.height);
                    }
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(view), Some(surface)) = (
                    self.window.as_ref(),
                    self.user_app.view_fn.as_ref(),
                    self.surface.as_ref(),
                ) {
                    self.scene.reset();
                    let size = window.inner_size();

                    clear_counter();

                    let mut layout = LayoutEngine::new();
                    let ui = view(&mut self.user_app.user_struct);

                    layout.compute(&ui, size.width as f32, size.height as f32);
                    if let (Some(font_context), Some(layout_context)) = (
                        self.user_app.font_context.as_mut(),
                        self.user_app.layout_context.as_mut(),
                    ) {
                        draw(
                            &mut self.scene,
                            font_context,
                            layout_context,
                            &mut layout,
                            1.0,
                            &ui,
                        );
                    }

                    // Get the window size
                    let width = surface.config.width;
                    let height = surface.config.height;

                    // Get a handle to the device
                    let device_handle = &self.context.devices[surface.dev_id];

                    let (r, g, b, a) = (
                        self.user_app.background.r,
                        self.user_app.background.g,
                        self.user_app.background.b,
                        self.user_app.background.a,
                    );

                    // Render to a texture, which we will later copy into the surface
                    self.renderer[surface.dev_id]
                        .as_mut()
                        .unwrap()
                        .render_to_texture(
                            &device_handle.device,
                            &device_handle.queue,
                            &self.scene,
                            &surface.target_view,
                            &RenderParams {
                                base_color: AlphaColor::from_rgba8(r, g, b, a),
                                width,
                                height,
                                antialiasing_method: AaConfig::Msaa8,
                            },
                        )
                        .expect("failed to render to surface");

                    // Get the surface's texture
                    let surface_texture = surface
                        .surface
                        .get_current_texture()
                        .expect("failed to get surface texture");

                    // Perform the copy
                    let mut encoder = device_handle.device.create_command_encoder(
                        &wgpu::CommandEncoderDescriptor {
                            label: Some("Surface Blit"),
                        },
                    );
                    surface.blitter.copy(
                        &device_handle.device,
                        &mut encoder,
                        &surface.target_view,
                        &surface_texture
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    );
                    device_handle.queue.submit([encoder.finish()]);
                    // Queue the texture to be presented on the surface
                    surface_texture.present();

                    device_handle.device.poll(wgpu::PollType::Poll).unwrap();

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
                                                    cb(
                                                        &mut self.user_app.user_struct,
                                                        &mut user_window,
                                                    );
                                                    window.request_redraw();
                                                }
                                            }
                                        }
                                    }

                                    // Get widget information (position, width and height)
                                    let layout_resolved = layout.get(child.id).unwrap();
                                    // Check if the widget was clicked
                                    let clicked =
                                        check_clicked(layout_resolved, self.user_app.position);
                                    if clicked {
                                        if let Some(callback) = &child.on_click {
                                            let mut cb = callback.borrow_mut();
                                            cb(&mut self.user_app.user_struct, &mut user_window);
                                            window.request_redraw();
                                        }
                                    }
                                }
                            } else if let WidgetElement::Container { child, .. } = &ui.element {
                                // Get widget information (position, width and height)
                                let layout_resolved = layout.get(ui.id).unwrap();
                                // Check if the widget was clicked
                                let clicked =
                                    check_clicked(layout_resolved, self.user_app.position);
                                if clicked {
                                    if let Some(callback) = &ui.on_click {
                                        let mut cb = callback.borrow_mut();
                                        cb(&mut self.user_app.user_struct, &mut user_window);
                                        window.request_redraw();
                                    }
                                } else {
                                    // Check the child of container

                                    // Get widget information (position, width and height)
                                    let layout_resolved = layout.get(child.id).unwrap();
                                    // Check if the widget was clicked
                                    let clicked =
                                        check_clicked(layout_resolved, self.user_app.position);
                                    if clicked {
                                        if let Some(callback) = &child.on_click {
                                            let mut cb = callback.borrow_mut();
                                            cb(&mut self.user_app.user_struct, &mut user_window);
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

fn check_clicked(layout: &LayoutNode, click: PhysicalPosition<f64>) -> bool {
    if click.x >= layout.x as f64
        && click.x <= layout.x as f64 + layout.width as f64
        && click.y >= layout.y as f64
        && click.y <= layout.y as f64 + layout.height as f64
    {
        return true;
    }
    false
}

/// Helper function that creates a vello `Renderer` for a given `RenderContext` and `RenderSurface`
fn create_vello_renderer(render_cx: &RenderContext, surface: &RenderSurface<'_>) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions::default(),
    )
    .expect("Couldn't create renderer")
}
