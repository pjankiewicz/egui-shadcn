//! StatusBar builder struct.

/// A compact status bar container for metadata and transient state.
#[must_use]
pub struct StatusBar {
    pub(crate) dense: bool,
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl StatusBar {
    pub fn new() -> Self {
        Self { dense: false }
    }

    pub fn dense(mut self) -> Self {
        self.dense = true;
        self
    }
}
