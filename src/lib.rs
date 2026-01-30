use crate::error::Error;
pub mod error;

pub mod application;

pub mod vello {
    pub use glazeui_vello::*;
}
pub mod layout {
    pub use glazeui_layout::*;
}
pub mod core {
    pub use glazeui_core::*;
}
pub mod shell {
    pub use glazeui_winit::*;
}

pub type Result = std::result::Result<(), Error>;
