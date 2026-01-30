use std::sync::Arc;

use glazeui_core::{WidgetElement, id::clear_counter, window::control::Window};
use glazeui_layout::{LayoutEngine, LayoutNode};
use glazeui_vello::draw;
use vello::{
    AaConfig, RenderParams, Renderer as VelloRenderer, RendererOptions,
    peniko::color::AlphaColor,
    util::{RenderContext, RenderSurface},
    wgpu,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::Program;

impl<App> ApplicationHandler for Program<App> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            // Get window settings
            let win_attr = self.window_attributes.clone();
            // Create window
            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("Creating window error"),
            );
            self.window = Some(window.clone());

            let size = window.inner_size();

            // Set vsync mode
            let mode = if self.renderer.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            };

            let surface_future =
                self.renderer
                    .context
                    .create_surface(window.clone(), size.width, size.height, mode);
            let surface = pollster::block_on(surface_future).expect("Error creating surface");

            // Create a vello Renderer for the surface (using its device id)
            self.renderer
                .renderers
                .resize_with(self.renderer.context.devices.len(), || None);
            self.renderer.renderers[surface.dev_id]
                .get_or_insert_with(|| create_vello_renderer(&self.renderer.context, &surface));

            self.renderer.surface = Some(surface);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let (Some(window), Some(surface)) =
                    (self.window.as_ref(), self.renderer.surface.as_mut())
                {
                    if new_size.width != 0 && new_size.height != 0 {
                        self.renderer.context.resize_surface(
                            surface,
                            new_size.width,
                            new_size.height,
                        );
                    }
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(view), Some(surface), Some(font_cx), Some(layout_cx)) = (
                    self.window.as_ref(),
                    self.application.view_fn.as_ref(),
                    self.renderer.surface.as_ref(),
                    self.renderer.font_context.as_mut(),
                    self.renderer.layout_context.as_mut(),
                ) {
                    // Reset scene
                    self.renderer.scene.reset();

                    // Get window size
                    let size = window.inner_size();

                    // // Remove all id's that was created in the past
                    clear_counter();

                    let mut layout = LayoutEngine::new();
                    let ui = view(&mut self.application.user_struct);

                    // Compute layout
                    layout.compute(
                        &ui,
                        size.width as f32,
                        size.height as f32,
                        font_cx,
                        layout_cx,
                    );

                    if let (Some(font_context), Some(layout_context)) = (
                        self.renderer.font_context.as_mut(),
                        self.renderer.layout_context.as_mut(),
                    ) {
                        draw(
                            &mut self.renderer.scene,
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
                    let device_handle = &self.renderer.context.devices[surface.dev_id];

                    let (r, g, b, a) = (
                        self.application.background.r,
                        self.application.background.g,
                        self.application.background.b,
                        self.application.background.a,
                    );

                    // Render to a texture, which we will later copy into the surface
                    self.renderer.renderers[surface.dev_id]
                        .as_mut()
                        .unwrap()
                        .render_to_texture(
                            &device_handle.device,
                            &device_handle.queue,
                            &self.renderer.scene,
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

                    self.renderer.layout = Some(layout);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    if let (Some(window), Some(view), Some(layout)) = (
                        self.window.as_ref(),
                        self.application.view_fn.as_ref(),
                        self.renderer.layout.as_ref(),
                    ) {
                        // Remove all id's that was created in the past
                        clear_counter();

                        // Get the root widget
                        let ui = view(&mut self.application.user_struct);

                        // Create copy of window and give that to user, with that he can edit the window settings
                        let mut user_window = Window {
                            window: self.window.as_ref().unwrap().clone(),
                            background: &mut self.application.background,
                            eventloop: event_loop,
                        };

                        // Get root widget info
                        let layout_resolved = layout.get(ui.id).unwrap();

                        // Check if was a click inside the root widget
                        let clicked = check_clicked(layout_resolved, self.application.position);

                        if clicked {
                            // If root widget is VStack or HStack
                            if let WidgetElement::VStack { children, .. }
                            | WidgetElement::HStack { children, .. } = &ui.element
                            {
                                // Go to every child in vstack/hstack childrens
                                for child in children {
                                    // Get widget information (position, width and height)
                                    let layout_resolved = layout.get(child.id).unwrap();
                                    // Check if was a click inside the widget
                                    let clicked =
                                        check_clicked(layout_resolved, self.application.position);

                                    if clicked {
                                        // If click was inside the widget and user provided a fn in on_press
                                        if let Some(callback) = &child.on_press {
                                            let mut cb = callback.borrow_mut();
                                            // Call on_press fn
                                            cb(&mut self.application.user_struct, &mut user_window);
                                            // Redraw the window
                                            window.request_redraw();
                                        }
                                    }
                                }
                            } else if let WidgetElement::Container { child, .. } = &ui.element {
                                // Get widget information (position, width and height)
                                let layout_resolved = layout.get(ui.id).unwrap();
                                // Check if was a click inside the widget
                                let clicked =
                                    check_clicked(layout_resolved, self.application.position);
                                if clicked {
                                    // If click was inside the widget and user provided a fn in on_press
                                    if let Some(callback) = &child.on_press {
                                        let mut cb = callback.borrow_mut();
                                        // Call on_press fn
                                        cb(&mut self.application.user_struct, &mut user_window);
                                        // Redraw the window
                                        window.request_redraw();
                                    }
                                }
                            }
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.application.position = position;
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
fn create_vello_renderer(render_cx: &RenderContext, surface: &RenderSurface<'_>) -> VelloRenderer {
    VelloRenderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions::default(),
    )
    .expect("Couldn't create renderer")
}
