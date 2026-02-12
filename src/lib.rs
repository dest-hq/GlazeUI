use crate::error::Error;
pub mod error;

pub mod application;

pub mod layout {
    pub use glazeui_layout::*;
}
pub mod core {
    pub use glazeui_core::*;
}
pub mod shell {
    pub use glazeui_winit::*;
}
pub mod render {
    pub use glazeui_render::*;
}

pub type Result = std::result::Result<(), Error>;
