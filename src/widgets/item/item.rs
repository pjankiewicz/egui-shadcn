//! Item builder struct — a list item container.

/// A list item: `rounded-lg border text-sm gap-2.5 px-3 py-2.5`.
pub struct Item {
    pub(crate) variant: crate::tokens::item_variant::ItemVariant,
}

impl Default for Item {
    fn default() -> Self {
        Self::new()
    }
}

impl Item {
    pub fn new() -> Self {
        Self {
            variant: crate::tokens::item_variant::ItemVariant::Default,
        }
    }

    pub fn variant(mut self, variant: crate::tokens::item_variant::ItemVariant) -> Self {
        self.variant = variant;
        self
    }
}
