use std::{borrow::Cow, sync::Arc};

use glazeui_canvas::{Area, Atlas, Canvas, Color as canvasColor, Item, Renderer, Shape, ShapeType};
use glazeui_core::{Node, NodeElement, node::TextWeight};
use glazeui_layout::LayoutEngine;
use glyphon::{
    Attrs, Buffer, Cache, Color, FontSystem, Metrics, Resolution, SwashCache, TextArea, TextAtlas,
    TextBounds, TextRenderer, Viewport,
};
use wgpu::{ExperimentalFeatures, MultisampleState};
use winit::window::Window;

pub struct WgpuCtx<'window> {
    surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    // render_pipeline: wgpu::RenderPipeline,
    font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: glyphon::Viewport,
    atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,
    text_buffer: Vec<glyphon::Buffer>,
    text_positions: Vec<(f32, f32, f32, f32)>,
}

impl<'window> WgpuCtx<'window> {
    pub async fn new_async(window: Arc<Window>) -> WgpuCtx<'window> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        // let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        //     label: None,
        //     layout: Some(&pipeline_layout),
        //     vertex: wgpu::VertexState {
        //         module: &shader,
        //         entry_point: Some("vs_main"),
        //         compilation_options: Default::default(),
        //         buffers: &[],
        //     },
        //     fragment: Some(wgpu::FragmentState {
        //         module: &shader,
        //         entry_point: Some("fs_main"),
        //         compilation_options: Default::default(),
        //         targets: &[Some(swapchain_format.into())],
        //     }),
        //     primitive: wgpu::PrimitiveState::default(),
        //     depth_stencil: None,
        //     cache: None,
        //     multiview_mask: None,
        //     multisample: wgpu::MultisampleState::default(),
        // });

        let size = window.inner_size();
        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &surface_config);

        // Set up text renderer
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(&device);
        let viewport = Viewport::new(&device, &cache);
        let mut atlas = TextAtlas::new(&device, &queue, &cache, swapchain_format);
        let text_renderer =
            TextRenderer::new(&mut atlas, &device, MultisampleState::default(), None);

        WgpuCtx {
            surface,
            surface_config,
            adapter,
            device,
            queue,
            // render_pipeline,
            font_system,
            swash_cache,
            viewport: viewport,
            atlas: atlas,
            text_buffer: Vec::new(),
            text_renderer: text_renderer,
            text_positions: Vec::new(),
        }
    }

    pub fn new(window: Arc<Window>) -> WgpuCtx<'window> {
        pollster::block_on(WgpuCtx::new_async(window))
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        let (width, height) = new_size;
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn draw(&mut self, element: &Node, layout: &LayoutEngine) {
        self.text_buffer.clear();
        self.text_positions.clear();
        self.viewport.update(
            &self.queue,
            Resolution {
                width: self.surface_config.width,
                height: self.surface_config.height,
            },
        );
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut _r_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                multiview_mask: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        // Render UI
        self.draw_node(element, layout, &mut encoder, &texture_view);

        let mut text_areas: Vec<TextArea> = Vec::new();
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

        if !text_areas.is_empty() {
            self.text_renderer
                .prepare(
                    &self.device,
                    &self.queue,
                    &mut self.font_system,
                    &mut self.atlas,
                    &self.viewport,
                    text_areas,
                    &mut self.swash_cache,
                )
                .unwrap();

            {
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
        }

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();

        self.atlas.trim();
    }

    fn draw_node(
        &mut self,
        node: &Node,
        layout: &LayoutEngine,
        encoder: &mut wgpu::CommandEncoder,
        texture_view: &wgpu::TextureView,
    ) {
        if let NodeElement::Text {
            content,
            font_size,
            line_height,
            weight,
        } = &node.element
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
                Metrics::new(*font_size, *line_height),
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
            let layout = layout.layouts.get(&node.id).unwrap();
            self.text_positions
                .push((layout.x, layout.y, layout.width, layout.height));
        }

        if let NodeElement::VStack { children, .. } = &node.element {
            for child in children.iter() {
                self.draw_node(child, layout, encoder, texture_view);
            }
        }

        if let NodeElement::HStack { children, .. } = &node.element {
            for child in children.iter() {
                self.draw_node(child, layout, encoder, texture_view);
            }
        }

        if let NodeElement::Container {
            child,
            width,
            height,
            color,
            radius,
        } = &node.element
        {
            let mut renderer = Renderer::new(
                &self.device,
                &self.surface_config.format,
                wgpu::MultisampleState::default(),
                None,
            );
            let layout = layout.layouts.get(&node.id).unwrap();
            let mut items = Vec::new();
            let mut atlas = Atlas::default();
            let (r, g, b, a) = color;
            let radius = radius.min(width * 0.5).min(height * 0.5);
            items.push((
                Area {
                    offset: (layout.x, layout.y),
                    bounds: None,
                },
                Item::Shape(Shape {
                    shape: ShapeType::RoundedRectangle(
                        0.0,
                        (*width, *height),
                        0.0, // This is responsible for setting how much it is flipped
                        radius,
                    ),
                    color: canvasColor(*r, *g, *b, *a),
                }),
            ));
            renderer.prepare(
                &self.device,
                &self.queue,
                self.surface_config.width as f32,
                self.surface_config.height as f32,
                &mut atlas,
                items,
            );

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                renderer.render(&mut render_pass);
            }
        }
    }
}
