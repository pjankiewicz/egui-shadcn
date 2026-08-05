//! The frame shadcn inputs hand to `egui::TextEdit`.

/// A borderless [`egui::Frame`] that keeps `TextEdit`'s default text margin.
///
/// The widgets in this crate paint their own background, border, and focus
/// ring, so the `TextEdit` inside must not draw a second one.
///
/// On egui 0.33 that was `TextEdit::frame(false)`, which suppressed *painting*
/// only — the widget's default `margin` of `Margin::symmetric(4, 2)` still
/// applied. Since 0.34 `frame` takes a [`egui::Frame`] that replaces `margin`
/// outright, so a bare [`egui::Frame::NONE`] would silently pull the text 4px
/// left and 2px up. Restoring the margin here keeps the layout unchanged.
pub fn text_edit_frame() -> egui::Frame {
    egui::Frame::NONE.inner_margin(egui::Margin::symmetric(4, 2))
}
