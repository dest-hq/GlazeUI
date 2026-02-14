#[cfg(feature = "async")]
use glazeui_core::task::Task;

use glazeui_core::{Widget, WidgetElement, id::clear_counter, window::Window as UserWindow};
use glazeui_layout::{LayoutEngine, LayoutNode};
use glazeui_render::{RenderState, Renderer};
use multirender::WindowRenderer;
#[cfg(feature = "skia")]
use multirender_skia::SkiaWindowRenderer;
#[cfg(feature = "vello")]
use multirender_vello::VelloWindowRenderer;
#[cfg(feature = "cpu")]
use multirender_vello_cpu::SoftbufferWindowRenderer;
#[cfg(feature = "hybrid")]
use multirender_vello_hybrid::VelloHybridWindowRenderer;
#[cfg(feature = "async")]
use winit::event_loop::EventLoopProxy;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, WindowEvent as WinitWindowEvent},
    event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::{Program, event::UserEvent};

impl<M: Clone + Send + 'static, App> ApplicationHandler<UserEvent<M>> for Program<M, App> {
    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let RenderState::Active { window, .. } = &self.renderer.render_state {
            self.renderer.render_state = RenderState::Suspended(Some(window.clone()));
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let renderer = match &mut self.renderer.backend {
            #[cfg(feature = "skia")]
            glazeui_core::Backend::Skia => Renderer::from(SkiaWindowRenderer::new()),
            #[cfg(feature = "cpu")]
            glazeui_core::Backend::Cpu => Renderer::from(SoftbufferWindowRenderer::new()),
            #[cfg(feature = "hybrid")]
            glazeui_core::Backend::Hybrid => Renderer::from(VelloHybridWindowRenderer::new()),
            #[cfg(feature = "vello")]
            glazeui_core::Backend::Vello => Renderer::from(VelloWindowRenderer::new()),
        };
        let fallback_backend = self.renderer.fallback_backend.clone();
        self.set_backend(renderer, &fallback_backend, event_loop);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: UserEvent<M>) {
        #[cfg(feature = "async")]
        match _event {
            UserEvent::Message(message) => {
                if let Some(window) = self.window.as_ref() {
                    // Create copy of window and give that to user, with that he can edit the window settings
                    let mut user_window = UserWindow {
                        window: window.clone(),
                        background: &mut self.application.background,
                        eventloop: _event_loop,
                    };

                    let update_fn = self.application.update_fn;
                    update_fn(&mut self.application.user_struct, message, &mut user_window);

                    let window = match &self.renderer.render_state {
                        RenderState::Active { window, renderer } => {
                            if renderer.is_active() {
                                Some(window)
                            } else {
                                None
                            }
                        }
                        RenderState::Suspended(_) => None,
                    };

                    if let Some(window) = window {
                        window.request_redraw();
                    }
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        id: WindowId,
        event: WinitWindowEvent,
    ) {
        let RenderState::Active { window, renderer } = &mut self.renderer.render_state else {
            return;
        };

        if window.id() != id {
            return;
        }

        match event {
            WinitWindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WinitWindowEvent::Resized(physical_size) => {
                self.width = physical_size.width;
                self.height = physical_size.height;
                renderer.set_size(self.width, self.height);
                self.request_redraw();
            }
            WinitWindowEvent::RedrawRequested => {
                // Remove all id's that was created in the past
                clear_counter();

                let mut layout = LayoutEngine::new();
                let view_fn = self.application.view_fn;
                let ui = view_fn(&mut self.application.user_struct);

                let scale = window.scale_factor();

                // Compute layout
                layout.compute(
                    &ui,
                    self.width as f32 / scale as f32,
                    self.height as f32 / scale as f32,
                    &mut self.renderer.font_context,
                    &mut self.renderer.layout_context,
                );

                self.renderer.layout = layout;

                match renderer {
                    #[cfg(feature = "skia")]
                    Renderer::Skia(r) => r.render(|p| {
                        Self::draw_scene(
                            p,
                            &mut self.renderer.font_context,
                            &mut self.renderer.layout_context,
                            &mut self.renderer.layout,
                            1.0,
                            &ui,
                            self.application.background,
                            (self.width, self.height),
                        );
                    }),
                    #[cfg(feature = "cpu")]
                    Renderer::CpuSoftbuffer(r) => r.render(|p| {
                        Self::draw_scene(
                            p,
                            &mut self.renderer.font_context,
                            &mut self.renderer.layout_context,
                            &mut self.renderer.layout,
                            1.0,
                            &ui,
                            self.application.background,
                            (self.width, self.height),
                        );
                    }),
                    #[cfg(feature = "vello")]
                    Renderer::Gpu(r) => r.render(|p| {
                        Self::draw_scene(
                            p,
                            &mut self.renderer.font_context,
                            &mut self.renderer.layout_context,
                            &mut self.renderer.layout,
                            1.0,
                            &ui,
                            self.application.background,
                            (self.width, self.height),
                        );
                    }),
                    #[cfg(feature = "hybrid")]
                    Renderer::Hybrid(r) => r.render(|p| {
                        Self::draw_scene(
                            p,
                            &mut self.renderer.font_context,
                            &mut self.renderer.layout_context,
                            &mut self.renderer.layout,
                            1.0,
                            &ui,
                            self.application.background,
                            (self.width, self.height),
                        );
                    }),
                    Renderer::Null(r) => r.render(|p| {
                        Self::draw_scene(
                            p,
                            &mut self.renderer.font_context,
                            &mut self.renderer.layout_context,
                            &mut self.renderer.layout,
                            1.0,
                            &ui,
                            self.application.background,
                            (self.width, self.height),
                        );
                    }),
                };
            }
            WinitWindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    if let Some(window) = self.window.as_ref() {
                        // Remove all id's that was created in the past
                        clear_counter();

                        // Create copy of window and give that to user, with that he can edit the window settings
                        let mut user_window = UserWindow {
                            window: window.clone(),
                            background: &mut self.application.background,
                            eventloop: event_loop,
                        };

                        // Get the root widget
                        let view_fn = self.application.view_fn;
                        let ui = view_fn(&mut self.application.user_struct);

                        check_click(
                            &mut user_window,
                            &ui,
                            #[cfg(feature = "async")]
                            &self.runtime,
                            &self.renderer.render_state,
                            &self.renderer.layout,
                            &self.application.position,
                            &mut self.application.user_struct,
                            #[cfg(feature = "async")]
                            self.proxy.clone(),
                            &self.application.update_fn,
                        );
                    }
                }
            }
            WinitWindowEvent::CursorMoved { position, .. } => {
                self.application.position = position;
            }
            _ => (),
        }
    }
}

fn check_click<M: Clone + Send + 'static, App>(
    window: &mut UserWindow,
    ui: &Widget<M>,
    #[cfg(feature = "async")] runtime: &tokio::runtime::Runtime,
    render_state: &RenderState,
    layout: &LayoutEngine<M>,
    pos: &PhysicalPosition<f64>,
    user_struct: &mut App,
    #[cfg(feature = "async")] proxy: EventLoopProxy<UserEvent<M>>,
    #[cfg(feature = "async")] user_update: &fn(&mut App, M, &mut UserWindow) -> Task<M>,
    #[cfg(not(feature = "async"))] user_update: &fn(&mut App, M, &mut UserWindow),
) {
    // Get root widget info
    let layout_resolved = layout.get(ui.id).unwrap();

    // Check if was a click inside the root widget
    let clicked = check_click_inside(layout_resolved, *pos);

    if clicked {
        // If root widget is VStack or HStack
        if let WidgetElement::VStack { children, .. } | WidgetElement::HStack { children, .. } =
            &ui.element
        {
            // Go to every child in vstack/hstack childrens
            for child in children {
                check_click(
                    window,
                    child,
                    #[cfg(feature = "async")]
                    runtime,
                    render_state,
                    layout,
                    pos,
                    user_struct,
                    #[cfg(feature = "async")]
                    proxy.clone(),
                    user_update,
                );
            }
        } else if let WidgetElement::Container { child, .. } = &ui.element {
            check_click(
                window,
                child,
                #[cfg(feature = "async")]
                runtime,
                render_state,
                layout,
                pos,
                user_struct,
                #[cfg(feature = "async")]
                proxy.clone(),
                user_update,
            );

            // Get widget information (position, width and height)
            let layout_resolved = layout.get(ui.id).unwrap();
            // Check if was a click inside the widget
            let clicked = check_click_inside(layout_resolved, *pos);
            if clicked {
                // If click was inside the widget and user provided a fn in on_press
                if let Some(callback) = &ui.on_press {
                    // Call update fn
                    let _task = user_update(user_struct, callback.clone(), window);

                    #[cfg(feature = "async")]
                    if let Some(future) = _task.future {
                        runtime.spawn(async move {
                            let message = future.await;
                            proxy.send_event(UserEvent::Message(message)).ok();
                        });
                    }

                    // Redraw the window
                    let window = match render_state {
                        RenderState::Active { window, renderer } => {
                            if renderer.is_active() {
                                Some(window)
                            } else {
                                None
                            }
                        }
                        RenderState::Suspended(_) => None,
                    };

                    if let Some(window) = window {
                        window.request_redraw();
                    }
                }
            }
        } else if let WidgetElement::Text { .. } = ui.element {
            // Get widget information (position, width and height)
            let layout_resolved = layout.get(ui.id).unwrap();
            // Check if was a click inside the widget
            let clicked = check_click_inside(layout_resolved, *pos);
            if clicked {
                // If click was inside the widget and user provided a fn in on_press
                if let Some(callback) = &ui.on_press {
                    // Call update fn
                    let _task = user_update(user_struct, callback.clone(), window);

                    #[cfg(feature = "async")]
                    if let Some(future) = _task.future {
                        runtime.spawn(async move {
                            let message = future.await;
                            proxy.send_event(UserEvent::Message(message)).ok();
                        });
                    }

                    // Redraw the window
                    let window = match render_state {
                        RenderState::Active { window, renderer } => {
                            if renderer.is_active() {
                                Some(window)
                            } else {
                                None
                            }
                        }
                        RenderState::Suspended(_) => None,
                    };

                    if let Some(window) = window {
                        window.request_redraw();
                    }
                }
            }
        }
    }
}

fn check_click_inside(layout: &LayoutNode, click: PhysicalPosition<f64>) -> bool {
    if click.x >= layout.x as f64
        && click.x <= layout.x as f64 + layout.width as f64
        && click.y >= layout.y as f64
        && click.y <= layout.y as f64 + layout.height as f64
    {
        return true;
    }
    false
}
