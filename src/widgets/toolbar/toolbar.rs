//! Toolbar builder struct.

/// A generic toolbar container for compact command groups.
#[must_use]
pub struct Toolbar {
    pub(crate) wrap: bool,
    pub(crate) spacing: f32,
    pub(crate) dense: bool,
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            wrap: true,
            spacing: 6.0,
            dense: false,
        }
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn dense(mut self) -> Self {
        self.dense = true;
        self.spacing = 4.0;
        self
    }
}
