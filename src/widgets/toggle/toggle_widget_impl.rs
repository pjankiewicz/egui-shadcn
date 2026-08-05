//! Widget trait implementation for Toggle.

impl egui::Widget for super::toggle::Toggle<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());
        let (height, h_padding, font_size) = self.size.metrics();

        // Convert WidgetText into a Galley preserving RichText attributes (bold, italic, etc.)
        let text_galley = self.text.into_galley(
            ui,
            None,
            font_size,
            egui::TextStyle::Button,
        );


        let desired = egui::vec2(text_galley.size().x + h_padding * 2.0, height);

        let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());

        if response.clicked() {
            *self.pressed = !*self.pressed;
            response.ctx.request_repaint();
        }

        if ui.is_rect_visible(rect) {
            let mut style = super::toggle_style::resolve_toggle_style(
                &theme,
                self.variant,
                *self.pressed,
                response.hovered(),
            );
            if response.is_pointer_button_down_on() {
                style.bg = crate::paint::interpolate_color::interpolate_color(
                    style.bg,
                    theme.accent,
                    0.65,
                );
                style.fg = theme.accent_foreground;
            }
            let painter = ui.painter();
            let cr = egui::CornerRadius::same(style.corner_radius.round() as u8);

            painter.rect_filled(rect, cr, style.bg);

            if let Some(border_color) = style.border {
                painter.rect_stroke(
                    rect,
                    cr,
                    egui::Stroke::new(1.0, border_color),
                    egui::epaint::StrokeKind::Inside,
                );
            }

            let text_pos = egui::pos2(
                rect.center().x - text_galley.size().x / 2.0,
                rect.center().y - text_galley.size().y / 2.0,
            );
            painter.galley(text_pos, text_galley, style.fg);

            if response.has_focus() {
                crate::paint::paint_focus_ring::paint_focus_ring(
                    painter,
                    rect,
                    style.corner_radius,
                    theme.ring,
                );
            }
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        response
    }
}
