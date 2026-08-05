//! Menubar builder struct — a horizontal menu bar.

/// A horizontal menu bar: `flex h-9 items-center border-b px-4`.
#[must_use]
pub struct Menubar;

impl Default for Menubar {
    fn default() -> Self {
        Self::new()
    }
}

impl Menubar {
    pub fn new() -> Self {
        Self
    }
}
