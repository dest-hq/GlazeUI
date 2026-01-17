use std::marker::PhantomData;
use std::sync::Arc;

use crate::core::widget::WidgetElement;
use crate::layout::LayoutEngine;
use crate::renderer::components;
use crate::{core::widget::Widget, widgets::text::TextWeight};
use components::{
    atlas::Atlas, lib::Area, lib::Color as canvasColor, lib::Item, lib::Shape, lib::ShapeType,
    renderer::Renderer,
};

use glyphon::{
    Attrs, Buffer, Cache, Color, FontSystem, Metrics, Resolution, SwashCache, TextArea, TextAtlas,
    TextBounds, TextRenderer, Viewport,
};
use wgpu::{ExperimentalFeatures, MultisampleState};
use winit::window::Window;

pub struct WgpuCtx<'window, Message> {
    surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,

    // Text
    font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: glyphon::Viewport,
    atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,
    text_buffer: Vec<glyphon::Buffer>,
    text_positions: Vec<(f32, f32, f32, f32)>,

    // Shape
    shape_renderer: Renderer,

    _marker: PhantomData<Message>,
}

pub struct UiFrame {
    pub texts: Vec<TextCmd>,
    pub shapes: Vec<ShapeCommand>,
}

pub struct TextCmd {
    pub buffer_index: usize,
    pub left: f32,
    pub top: f32,
    pub bounds: TextBounds,
    pub color: Color,
}

pub struct ShapeCommand {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: (u8, u8, u8, u8),
    pub radius: f32,
}

impl<'window, Message> WgpuCtx<'window, Message> {
    pub async fn new_async(window: Arc<Window>) -> WgpuCtx<'window, Message> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                // Request an adapter which can render to out surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropiate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                experimental_features: ExperimentalFeatures::disabled(),
                trace: wgpu::Trace::Off,
            })
            .await
            .expect("Failed to create device");

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let size = window.inner_size();
        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &surface_config);

        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(&device);
        let viewport = Viewport::new(&device, &cache);
        let mut atlas = TextAtlas::new(&device, &queue, &cache, swapchain_format);

        // Set up text renderer
        let text_renderer =
            TextRenderer::new(&mut atlas, &device, MultisampleState::default(), None);

        // Set up shape renderer
        let shape_renderer = Renderer::new(
            &device,
            &surface_config.format,
            wgpu::MultisampleState::default(),
            None,
        );

        WgpuCtx {
            surface,
            surface_config,
            device,
            queue,
            font_system,
            swash_cache,
            viewport: viewport,
            atlas: atlas,
            text_buffer: Vec::new(),
            text_renderer: text_renderer,
            text_positions: Vec::new(),
            shape_renderer: shape_renderer,
            _marker: PhantomData,
        }
    }

    pub fn new(window: Arc<Window>) -> WgpuCtx<'window, Message> {
        pollster::block_on(WgpuCtx::new_async(window))
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        let (width, height) = new_size;
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn draw(
        &mut self,
        element: &Widget<Message>,
        layout: &LayoutEngine<Message>,
        font_system: &mut FontSystem,
    ) {
        self.text_buffer.clear();
        self.text_positions.clear();
        self.viewport.update(
            &self.queue,
            Resolution {
                width: self.surface_config.width,
                height: self.surface_config.height,
            },
        );

        let mut frame = UiFrame {
            texts: Vec::new(),
            shapes: Vec::new(),
        };

        // Collect Widgets
        self.collect_widgets(element, layout, &mut frame);

        // Render UI
        self.render(frame, font_system);
    }

    fn render(&mut self, frame: UiFrame, font_system: &mut FontSystem) {
        let surface_texture = self.surface.get_current_texture().unwrap();
        let texture_view = surface_texture.texture.create_view(&Default::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());

        // 1. Background
        {
            let mut _background_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                multiview_mask: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        // 2. Shapes

        let mut items = Vec::new();
        let mut atlas = Atlas::default();
        for (
            _idx,
            ShapeCommand {
                x,
                y,
                width,
                height,
                color,
                radius,
            },
        ) in frame.shapes.iter().enumerate()
        {
            let (r, g, b, a) = color;
            items.push((
                Area {
                    offset: (*x, *y),
                    bounds: None,
                },
                Item::Shape(Shape {
                    shape: ShapeType::RoundedRectangle(
                        0.0,
                        (*width, *height),
                        0.0, // This is responsible for setting how much it is flipped
                        radius.min(width * 0.5).min(height * 0.5),
                    ),
                    color: canvasColor(*r, *g, *b, *a),
                }),
            ));
        }

        self.shape_renderer.prepare(
            &self.device,
            &self.queue,
            self.surface_config.width as f32,
            self.surface_config.height as f32,
            &mut atlas,
            items,
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("shape_pass"),
                multiview_mask: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.shape_renderer.render(&mut render_pass);
        }

        // 3. Text

        let mut text_areas = Vec::new();

        for (idx, (x, y, width, height)) in self.text_positions.iter().enumerate() {
            text_areas.push(TextArea {
                buffer: &self.text_buffer[idx],
                left: *x,
                top: *y,
                scale: 1.0,
                bounds: TextBounds {
                    left: *x as i32,
                    top: *y as i32,
                    right: (*x + *width) as i32,
                    bottom: (*y + *height) as i32,
                },
                default_color: Color::rgb(255, 255, 255),
                custom_glyphs: &[],
            });
        }

        if !frame.texts.is_empty() && !text_areas.is_empty() {
            self.text_renderer
                .prepare(
                    &self.device,
                    &self.queue,
                    font_system,
                    &mut self.atlas,
                    &self.viewport,
                    text_areas,
                    &mut self.swash_cache,
                )
                .unwrap();

            let mut text_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("text_pass"),
                multiview_mask: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.text_renderer
                .render(&mut self.atlas, &self.viewport, &mut text_pass)
                .unwrap();
        }

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
        self.atlas.trim();
    }

    fn collect_widgets(
        &mut self,
        widget: &Widget<Message>,
        layout: &LayoutEngine<Message>,
        frame: &mut UiFrame,
    ) {
        if let WidgetElement::Text {
            content,
            font_size,
            line_height,
            weight,
        } = &widget.element
        {
            let weight = match weight {
                TextWeight::THIN => 100,
                TextWeight::EXTRA_LIGHT => 200,
                TextWeight::LIGHT => 300,
                TextWeight::NORMAL => 400,
                TextWeight::MEDIUM => 500,
                TextWeight::SEMIBOLD => 600,
                TextWeight::BOLD => 700,
                TextWeight::EXTRA_BOLD => 800,
                TextWeight::BLACK => 900,
            };
            let mut text_buffer = Buffer::new(
                &mut self.font_system,
                Metrics::new(*font_size as f32, *line_height),
            );
            text_buffer.set_text(
                &mut self.font_system,
                &content,
                &Attrs::new()
                    .family(glyphon::Family::SansSerif)
                    .weight(glyphon::Weight(weight)),
                glyphon::Shaping::Advanced,
                None,
            );
            text_buffer.shape_until_scroll(&mut self.font_system, false);

            // Push text buffer to vec
            self.text_buffer.push(text_buffer);

            let layout_resolved = layout.layouts.get(&widget.id).unwrap();

            self.text_positions.push((
                layout_resolved.x,
                layout_resolved.y,
                layout_resolved.width,
                layout_resolved.height,
            ));

            let bounds = TextBounds {
                left: layout_resolved.x as i32,
                top: layout_resolved.y as i32,
                right: (layout_resolved.x + layout_resolved.width) as i32,
                bottom: (layout_resolved.y + layout_resolved.height) as i32,
            };

            frame.texts.push(TextCmd {
                buffer_index: self.text_buffer.len(),
                left: layout_resolved.x,
                top: layout_resolved.y,
                bounds: bounds,
                color: Color::rgb(255, 255, 255),
            });
        }

        if let WidgetElement::VStack { children, .. } = &widget.element {
            for child in children.iter() {
                self.collect_widgets(child, layout, frame);
            }
        }

        if let WidgetElement::HStack { children, .. } = &widget.element {
            for child in children.iter() {
                self.collect_widgets(child, layout, frame);
            }
        }

        if let WidgetElement::Container {
            child,
            width,
            height,
            color,
            radius,
            ..
        } = &widget.element
        {
            let layout_resolved = layout.layouts.get(&widget.id).unwrap();
            frame.shapes.push(ShapeCommand {
                x: layout_resolved.x,
                y: layout_resolved.y,
                width: *width,
                height: *height,
                color: *color,
                radius: radius.min(width * 0.5).min(height * 0.5),
            });

            self.collect_widgets(child, layout, frame);
        }
    }
}
