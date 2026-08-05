//! Calendar builder struct — a month grid for date selection.

/// A calendar grid: month view with day cells.
#[must_use]
pub struct Calendar;

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

impl Calendar {
    pub fn new() -> Self {
        Self
    }
}
