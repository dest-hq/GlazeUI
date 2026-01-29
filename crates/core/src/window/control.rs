use crate::{
    color::Color,
    window::{theme::Theme, user_attention::UserAttention},
};
use std::sync::Arc;
use winit::{
    event_loop::ActiveEventLoop, window::Theme as WinitTheme, window::Window as WinitWindow,
};
// Crate for controling the window in runtime
pub struct Window<'window> {
    pub window: Arc<WinitWindow>,
    pub background: &'window mut Color,
    pub eventloop: &'window ActiveEventLoop,
}

impl<'window> Window<'window> {
    /// Window title
    pub fn title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw();
    }

    /// Window background
    pub fn background(&mut self, color: Color) {
        self.background.r = color.r;
        self.background.g = color.g;
        self.background.b = color.b;
        self.background.a = color.a;
    }

    pub fn request_user_attention(&mut self, attention: UserAttention) {
        let attention = match attention {
            UserAttention::Critical => winit::window::UserAttentionType::Critical,
            UserAttention::Informational => winit::window::UserAttentionType::Informational,
        };
        self.window.request_user_attention(Some(attention));
    }

    // Window decorations
    pub fn decorations(&mut self, decorations: bool) {
        self.window.set_decorations(decorations);
    }

    /// If window will be resizable
    pub fn resizable(&mut self, resizable: bool) {
        self.window.set_resizable(resizable);
    }

    /// Theme for title bar
    pub fn theme(&mut self, theme: Theme) {
        let theme = match theme {
            Theme::Dark => WinitTheme::Dark,
            Theme::Light => WinitTheme::Light,
        };
        self.window.set_theme(Some(theme));
    }

    /// Close the window
    pub fn close(&mut self) {
        self.eventloop.exit();
    }

    /// Minimize the window
    pub fn minimize(&mut self) {
        self.window.set_minimized(true);
    }

    /// Maximize the window
    pub fn maximize(&mut self, maximized: bool) {
        self.window.set_maximized(maximized);
    }
}
