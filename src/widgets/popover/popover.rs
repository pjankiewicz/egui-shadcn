//! Popover builder struct — a click-triggered popup panel.

/// A popover: `bg-popover border rounded-xl p-4 shadow-md`.
#[must_use]
pub struct Popover;

impl Default for Popover {
    fn default() -> Self {
        Self::new()
    }
}

impl Popover {
    pub fn new() -> Self {
        Self
    }
}
