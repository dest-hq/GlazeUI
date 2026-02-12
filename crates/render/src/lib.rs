use std::sync::Arc;

use multirender::{NullWindowRenderer, WindowRenderer};
#[cfg(feature = "skia")]
use multirender_skia::SkiaWindowRenderer;
#[cfg(feature = "vello")]
use multirender_vello::VelloWindowRenderer;
#[cfg(feature = "cpu")]
use multirender_vello_cpu::VelloCpuImageRenderer;
#[cfg(feature = "hybrid")]
use multirender_vello_hybrid::VelloHybridWindowRenderer;
use winit::window::Window;

pub mod draw;
pub mod widgets;

pub enum Renderer {
    #[cfg(feature = "vello")]
    Gpu(Box<VelloWindowRenderer>),
    #[cfg(feature = "hybrid")]
    Hybrid(Box<VelloHybridWindowRenderer>),
    #[cfg(feature = "cpu")]
    CpuSoftbuffer(Box<multirender_vello_cpu::SoftbufferWindowRenderer<VelloCpuImageRenderer>>),
    #[cfg(feature = "skia")]
    Skia(Box<SkiaWindowRenderer>),
    Null(multirender::NullWindowRenderer),
}
#[cfg(feature = "vello")]
impl From<VelloWindowRenderer> for Renderer {
    fn from(renderer: VelloWindowRenderer) -> Self {
        Self::Gpu(Box::new(renderer))
    }
}
#[cfg(feature = "hybrid")]
impl From<VelloHybridWindowRenderer> for Renderer {
    fn from(renderer: VelloHybridWindowRenderer) -> Self {
        Self::Hybrid(Box::new(renderer))
    }
}
#[cfg(feature = "cpu")]
impl From<multirender_vello_cpu::SoftbufferWindowRenderer<VelloCpuImageRenderer>> for Renderer {
    fn from(
        renderer: multirender_vello_cpu::SoftbufferWindowRenderer<VelloCpuImageRenderer>,
    ) -> Self {
        Self::CpuSoftbuffer(Box::new(renderer))
    }
}
#[cfg(feature = "skia")]
impl From<SkiaWindowRenderer> for Renderer {
    fn from(renderer: SkiaWindowRenderer) -> Self {
        Self::Skia(Box::new(renderer))
    }
}
impl From<NullWindowRenderer> for Renderer {
    fn from(renderer: NullWindowRenderer) -> Self {
        Self::Null(renderer)
    }
}

impl Renderer {
    pub fn is_active(&self) -> bool {
        match self {
            #[cfg(feature = "vello")]
            Renderer::Gpu(r) => r.is_active(),
            #[cfg(feature = "hybrid")]
            Renderer::Hybrid(r) => r.is_active(),
            #[cfg(feature = "cpu")]
            Renderer::CpuSoftbuffer(r) => r.is_active(),
            Renderer::Null(r) => r.is_active(),
            #[cfg(feature = "skia")]
            Renderer::Skia(r) => r.is_active(),
        }
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        match self {
            #[cfg(feature = "vello")]
            Renderer::Gpu(r) => r.set_size(w, h),
            #[cfg(feature = "hybrid")]
            Renderer::Hybrid(r) => r.set_size(w, h),
            #[cfg(feature = "cpu")]
            Renderer::CpuSoftbuffer(r) => r.set_size(w, h),
            Renderer::Null(r) => r.set_size(w, h),
            #[cfg(feature = "skia")]
            Renderer::Skia(r) => r.set_size(w, h),
        }
    }
}

pub enum RenderState {
    Active {
        window: Arc<Window>,
        renderer: Renderer,
    },
    Suspended(Option<Arc<Window>>),
}
