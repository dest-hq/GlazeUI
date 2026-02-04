#[derive(Debug, Default, Clone)]
/// Space inside an element
pub struct Padding {
    pub top: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Padding {
    pub fn all(padding: i32) -> Self {
        Padding {
            top: padding,
            left: padding,
            right: padding,
            bottom: padding,
        }
    }

    pub fn new() -> Self {
        Padding {
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
        }
    }

    pub fn left(mut self, padding: i32) -> Self {
        self.left = padding;
        self
    }

    pub fn right(mut self, padding: i32) -> Self {
        self.right = padding;
        self
    }

    pub fn bottom(mut self, padding: i32) -> Self {
        self.bottom = padding;
        self
    }

    pub fn top(mut self, padding: i32) -> Self {
        self.bottom = padding;
        self
    }
}
