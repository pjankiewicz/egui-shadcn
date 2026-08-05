//! Spinner builder struct — an animated rotating arc.

/// An animated spinner: `size-4 animate-spin`.
#[must_use]
pub struct Spinner {
    pub(crate) size: f32,
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Spinner {
    pub fn new() -> Self {
        Self { size: 16.0 } // size-4 = 16px
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(self)
    }
}
