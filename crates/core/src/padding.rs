#[derive(Debug)]
pub struct Padding {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Padding {
    pub fn all(padding: f32) -> Self {
        Padding {
            top: padding,
            left: padding,
            right: padding,
            bottom: padding,
        }
    }

    pub fn new() -> Self {
        Padding {
            top: 0.0,
            left: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }

    pub fn left(mut self, padding: f32) -> Self {
        self.left = padding;
        self
    }

    pub fn right(mut self, padding: f32) -> Self {
        self.right = padding;
        self
    }

    pub fn bottom(mut self, padding: f32) -> Self {
        self.bottom = padding;
        self
    }

    pub fn top(mut self, padding: f32) -> Self {
        self.bottom = padding;
        self
    }
}
