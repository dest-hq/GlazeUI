#[derive(Debug, Default, Clone)]
/// Space outside an element
pub struct Margin {
    pub top: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Margin {
    pub fn all(margin: i32) -> Self {
        Margin {
            top: margin,
            left: margin,
            right: margin,
            bottom: margin,
        }
    }

    pub fn new() -> Self {
        Margin {
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
        }
    }

    pub fn left(mut self, margin: i32) -> Self {
        self.left = margin;
        self
    }

    pub fn right(mut self, margin: i32) -> Self {
        self.right = margin;
        self
    }

    pub fn bottom(mut self, margin: i32) -> Self {
        self.bottom = margin;
        self
    }

    pub fn top(mut self, margin: i32) -> Self {
        self.bottom = margin;
        self
    }
}
