//! Dialog builder struct — a modal overlay panel.

/// A modal dialog: centered panel with backdrop overlay.
#[must_use]
pub struct Dialog {
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) width: f32,
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

impl Dialog {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            width: 420.0,
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

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}
