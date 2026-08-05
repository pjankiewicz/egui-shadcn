//! Sidebar builder struct -- app sidebar navigation.

/// A sidebar: `w-64 border-r bg-sidebar h-full`.
#[must_use]
pub struct Sidebar {
    pub(crate) width: f32,
    pub(crate) collapsible: bool,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            width: 256.0,
            collapsible: false,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn collapsible(mut self) -> Self {
        self.collapsible = true;
        self
    }
}
