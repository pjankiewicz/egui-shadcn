//! Show method for Resizable — renders a draggable split panel.

impl super::resizable::Resizable {
    /// Shows a horizontal split with draggable divider.
    ///
    /// `fraction` persists the split position across frames — pass
    /// `&mut your_f32_state`. On the first frame for this [`egui::Ui`] it is
    /// overwritten with the `initial_fraction` given to
    /// [`super::resizable::Resizable::new`], so the caller does not have to
    /// duplicate that value in its own state initialiser.
    pub fn show(
        self,
        ui: &mut egui::Ui,
        fraction: &mut f32,
        left: impl FnOnce(&mut egui::Ui),
        right: impl FnOnce(&mut egui::Ui),
    ) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());
        let available_width = ui.available_width();
        let handle_width: f32 = 8.0;
        let panel_height = self.height;

        let seeded_id = ui.id().with("shadcn_resizable_seeded");
        if !ui.data(|d| d.get_temp::<bool>(seeded_id).unwrap_or(false)) {
            *fraction = self.initial_fraction;
            ui.data_mut(|d| d.insert_temp(seeded_id, true));
        }

        let left_width = (available_width - handle_width) * (*fraction);
        let right_width = available_width - left_width - handle_width;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            // Left panel
            ui.allocate_ui(egui::vec2(left_width, panel_height), left);

            // Handle
            let (handle_rect, handle_response) = ui.allocate_exact_size(
                egui::vec2(handle_width, panel_height),
                egui::Sense::drag(),
            );

            if handle_response.dragged() {
                let delta = handle_response.drag_delta().x;
                let total = available_width - handle_width;
                *fraction = (*fraction + delta / total).clamp(0.1, 0.9);
                ui.ctx().request_repaint();
            }

            if handle_response.hovered() || handle_response.dragged() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeColumn);
            }

            // Draw handle
            if ui.is_rect_visible(handle_rect) {
                let painter = ui.painter();
                painter.rect_filled(handle_rect, egui::CornerRadius::ZERO, theme.border);

                // Center dots
                let center = handle_rect.center();
                let dot_color = theme.muted_foreground;
                for dy in [-8.0_f32, 0.0, 8.0] {
                    painter.circle_filled(
                        egui::pos2(center.x, center.y + dy),
                        1.5,
                        dot_color,
                    );
                }
            }

            // Right panel
            ui.allocate_ui(egui::vec2(right_width, panel_height), right);
        })
        .response
    }
}
