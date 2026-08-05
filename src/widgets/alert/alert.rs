//! Alert builder struct — a status message container.

/// An alert container: `rounded-lg border px-4 py-3 text-sm`.
#[must_use]
pub struct Alert {
    pub(crate) title: Option<String>,
    pub(crate) variant: crate::tokens::alert_variant::AlertVariant,
}

impl Default for Alert {
    fn default() -> Self {
        Self::new()
    }
}

impl Alert {
    pub fn new() -> Self {
        Self {
            title: None,
            variant: crate::tokens::alert_variant::AlertVariant::Default,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn variant(mut self, variant: crate::tokens::alert_variant::AlertVariant) -> Self {
        self.variant = variant;
        self
    }
}
