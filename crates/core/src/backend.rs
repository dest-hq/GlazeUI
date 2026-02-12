#[derive(Debug, Clone, PartialEq)]
pub enum Backend {
    // OpenGL for Windows, Linux; Metal for MacOS
    #[cfg(feature = "skia")]
    Skia,
    // CPU Render
    #[cfg(feature = "cpu")]
    Cpu,
    // Vulkan, Metal, DX12
    #[cfg(feature = "vello")]
    Vello,
    // Vello + CPU
    #[cfg(feature = "hybrid")]
    Hybrid,
}
