//! PropertyGrid builder struct.

/// A generic inspector-style property grid container.
#[must_use]
pub struct PropertyGrid {
    pub(crate) label_width: f32,
    pub(crate) row_gap: f32,
}

impl Default for PropertyGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyGrid {
    pub fn new() -> Self {
        Self {
            label_width: 112.0,
            row_gap: 8.0,
        }
    }

    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = width;
        self
    }

    pub fn row_gap(mut self, gap: f32) -> Self {
        self.row_gap = gap;
        self
    }
}
