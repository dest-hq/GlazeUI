#[derive(Debug, Clone, PartialEq)]
pub enum Backend {
    /// Vulkan, Metal, DX12 or Browser WebGPU Renderer
    Auto,
    Vulkan,
    Metal,
    DX12,
    /// OpenGL Renderer
    OpenGL,
}
