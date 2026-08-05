//! HoverCard builder struct — a hover-triggered popup panel.

/// A hover card: popup that appears on hover over a trigger element.
#[must_use]
pub struct HoverCard {
    pub(crate) width: f32,
}

impl Default for HoverCard {
    fn default() -> Self {
        Self::new()
    }
}

impl HoverCard {
    pub fn new() -> Self {
        Self { width: 280.0 }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}
