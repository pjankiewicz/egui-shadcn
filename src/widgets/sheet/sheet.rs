//! Sheet builder struct — a slide-in panel from screen edge.

/// A slide-in panel: `fixed inset-y-0 bg-background border`.
#[must_use]
pub struct Sheet {
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) side: crate::tokens::sheet_side::SheetSide,
    pub(crate) width: f32,
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}

impl Sheet {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            side: crate::tokens::sheet_side::SheetSide::Right,
            width: 320.0,
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

    pub fn side(mut self, side: crate::tokens::sheet_side::SheetSide) -> Self {
        self.side = side;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}
