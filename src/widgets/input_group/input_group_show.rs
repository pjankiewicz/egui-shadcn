//! Show method for InputGroup — renders input with prefix/suffix addons.

impl super::input_group::InputGroup {
    /// Renders an input field with optional prefix text and suffix content.
    pub fn show(
        ui: &mut egui::Ui,
        text: &mut String,
        placeholder: &str,
        prefix: Option<&str>,
        suffix: Option<impl FnOnce(&mut egui::Ui)>,
    ) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());
        let cr = egui::CornerRadius::same(theme.radius.round() as u8);
        let height: f32 = 32.0;

        let width = ui.available_width().min(300.0);
        let desired = egui::vec2(width, height);
        let (outer_rect, outer_response) = ui.allocate_exact_size(desired, egui::Sense::hover());
        let outer_hovered = outer_response.hovered() || ui.rect_contains_pointer(outer_rect);

        // Outer border
        let bg = if outer_hovered {
            crate::paint::interpolate_color::interpolate_color(theme.background, theme.accent, 0.35)
        } else {
            theme.background
        };
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

        let mut cursor_x = outer_rect.min.x;
        let h_padding: f32 = 10.0;

        // Prefix text
        if let Some(prefix_text) = prefix {
            let galley = ui.painter().layout_no_wrap(
                prefix_text.to_owned(),
                egui::FontId::proportional(14.0),
                theme.muted_foreground,
            );
            let prefix_w = galley.size().x + h_padding;
            let text_pos = egui::pos2(
                cursor_x + h_padding,
                outer_rect.center().y - galley.size().y / 2.0,
            );
            ui.painter()
                .galley(text_pos, galley, theme.muted_foreground);

            // Divider line
            cursor_x += prefix_w + h_padding;
            ui.painter().vline(
                cursor_x,
                outer_rect.y_range(),
                egui::Stroke::new(1.0, theme.border),
            );
        }

        // Suffix area (reserve space on right)
        let suffix_width: f32 = if suffix.is_some() { 40.0 } else { 0.0 };
        let input_right = outer_rect.max.x - suffix_width;

        // Input area
        let input_rect = egui::Rect::from_min_max(
            egui::pos2(cursor_x + h_padding, outer_rect.min.y + 2.0),
            egui::pos2(input_right - h_padding, outer_rect.max.y - 2.0),
        );
        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(input_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );

        let te = egui::TextEdit::singleline(text)
            .frame(crate::paint::text_edit_frame::text_edit_frame())
            .hint_text(placeholder)
            .text_color(theme.foreground)
            .desired_width(input_rect.width());

        let response = child_ui.add(te);

        // Suffix content
        if let Some(suffix_fn) = suffix {
            let suffix_rect = egui::Rect::from_min_max(
                egui::pos2(input_right, outer_rect.min.y + 2.0),
                egui::pos2(outer_rect.max.x - 4.0, outer_rect.max.y - 2.0),
            );
            // Divider
            ui.painter().vline(
                input_right,
                outer_rect.y_range(),
                egui::Stroke::new(1.0, theme.border),
            );
            let mut suffix_ui = ui.new_child(egui::UiBuilder::new().max_rect(suffix_rect).layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
            ));
            suffix_fn(&mut suffix_ui);
        }

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
                theme.radius,
                theme.ring,
            );
        }

        response
    }
}
