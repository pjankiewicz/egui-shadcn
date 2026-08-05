//! Widget trait implementation for Textarea.

impl egui::Widget for super::textarea::Textarea<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());

        let h_padding: f32 = 10.0; // px-2.5
        let v_padding: f32 = 8.0; // py-2
        let width = self
            .desired_width
            .unwrap_or(ui.available_width().min(240.0));
        let corner_radius = theme.radius;
        let cr = egui::CornerRadius::same(corner_radius.round() as u8);

        let desired = egui::vec2(width, self.min_height);
        let (outer_rect, outer_response) = ui.allocate_exact_size(desired, egui::Sense::hover());
        let outer_hovered = outer_response.hovered() || ui.rect_contains_pointer(outer_rect);

        // Background and border
        let mut bg =
            crate::paint::interpolate_color::interpolate_color(theme.background, theme.muted, 0.4);
        if outer_hovered {
            bg = crate::paint::interpolate_color::interpolate_color(bg, theme.accent, 0.35);
        }
        ui.painter().rect_filled(outer_rect, cr, bg);
        ui.painter().rect_stroke(
            outer_rect,
            cr,
            egui::Stroke::new(
                1.0,
                if outer_hovered {
                    theme.input
                } else {
                    theme.border
                },
            ),
            egui::epaint::StrokeKind::Inside,
        );

        // Inner area with scroll for overflow
        let inner_rect = outer_rect.shrink2(egui::vec2(h_padding, v_padding));
        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(inner_rect)
                .layout(egui::Layout::top_down(egui::Align::LEFT)),
        );

        let scroll_resp = egui::ScrollArea::vertical()
            .max_height(inner_rect.height())
            .show(&mut child_ui, |ui| {
                let text_edit = egui::TextEdit::multiline(self.text)
                    .frame(crate::paint::text_edit_frame::text_edit_frame())
                    .hint_text(&self.placeholder)
                    .text_color(theme.foreground)
                    .desired_width(inner_rect.width())
                    .desired_rows(3);

                ui.add(text_edit)
            });

        let response = scroll_resp.inner;

        // Focus ring
        if response.has_focus() {
            ui.painter().rect_stroke(
                outer_rect,
                cr,
                egui::Stroke::new(1.0, theme.ring),
                egui::epaint::StrokeKind::Inside,
            );
            crate::paint::paint_focus_ring::paint_focus_ring(
                ui.painter(),
                outer_rect,
                corner_radius,
                theme.ring,
            );
        }

        response
    }
}
