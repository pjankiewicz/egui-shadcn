//! DatePicker builder struct — date input with calendar popup.

/// A date picker: button that opens a calendar popup for date selection.
#[must_use]
pub struct DatePicker {
    pub(crate) placeholder: String,
}

impl Default for DatePicker {
    fn default() -> Self {
        Self::new()
    }
}

impl DatePicker {
    pub fn new() -> Self {
        Self {
            placeholder: "Pick a date".to_owned(),
        }
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }
}
