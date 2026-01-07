use wgpu::{
    BindGroup, BindGroupLayout, Device, Extent3d, Origin3d, Queue, Sampler, TexelCopyBufferLayout,
    TexelCopyTextureInfo, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages, TextureViewDescriptor,
};

use crate::RgbaImage;

use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default, Debug)]
pub struct Atlas {
    pub(crate) image: ImageAtlas,
}
impl Atlas {
    pub(crate) fn trim(&mut self) {
        self.image.trim();
    }
}

#[derive(Default, Debug)]
pub struct ImageAtlas(Vec<(Arc<RgbaImage>, Arc<BindGroup>)>);

impl ImageAtlas {
    pub fn trim(&mut self) {
        self.0 = self
            .0
            .drain(..)
            .filter(|(i, _)| (Arc::strong_count(i) > 1))
            .collect();
    }

    pub fn get(
        &mut self,
        queue: &Queue,
        device: &Device,
        layout: &BindGroupLayout,
        sampler: &Sampler,
        image: &Arc<RgbaImage>,
    ) -> Arc<BindGroup> {
        match self
            .0
            .iter()
            .find_map(|(i, b)| Arc::ptr_eq(i, image).then_some(b))
        {
            Some(bind_group) => bind_group.clone(),
            None => {
                let size = Extent3d {
                    width: image.width(),
                    height: image.height(),
                    depth_or_array_layers: 1,
                };

                let texture = device.create_texture(&TextureDescriptor {
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Rgba8UnormSrgb,
                    usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                    label: None,
                    view_formats: &[],
                });

                queue.write_texture(
                    TexelCopyTextureInfo {
                        texture: &texture,
                        mip_level: 0,
                        origin: Origin3d::ZERO,
                        aspect: TextureAspect::All,
                    },
                    image,
                    TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * image.width()),
                        rows_per_image: Some(image.height()),
                    },
                    size,
                );

                let texture_view = texture.create_view(&TextureViewDescriptor::default());

                let bind_group = Arc::new(device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&texture_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(sampler),
                        },
                    ],
                    label: None,
                }));

                self.0.push((image.clone(), bind_group.clone()));
                bind_group
            }
        }
    }
}

type ImageMap = HashMap<char, Option<Arc<RgbaImage>>>;
type Offset = (f32, f32);
