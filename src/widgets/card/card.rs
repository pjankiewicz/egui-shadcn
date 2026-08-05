//! Card builder struct — a bordered container styled after shadcn/ui Card.

/// A card container: `ring-foreground/10 bg-card rounded-xl py-4 ring-1`.
///
/// Not a `Widget` — uses `show(ui, closure)` pattern like `egui::Frame`.
#[must_use]
pub struct Card;

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Card {
    pub fn new() -> Self {
        Self
    }
}
