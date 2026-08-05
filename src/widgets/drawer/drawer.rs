//! Drawer builder struct — a bottom slide-up panel.

/// A drawer: bottom sheet with handle and content area.
#[must_use]
pub struct Drawer {
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
}

impl Default for Drawer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawer {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}
