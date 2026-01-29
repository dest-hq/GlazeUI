#[derive(Debug, Clone)]
pub enum Backend {
    /// Vulkan, Metal, DX12 or Browser WebGPU Renderer
    Auto,
    /// CPU Renderer
    CPU,
    /// OpenGL Renderer
    OpenGL,
}
