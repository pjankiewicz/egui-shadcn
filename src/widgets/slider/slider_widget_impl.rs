//! Widget trait implementation for Slider.

impl egui::Widget for super::slider::Slider<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());
        let style = super::slider_style::resolve_slider_style(&theme);

        let track_height: f32 = 4.0;
        let handle_radius = 6.0;
        let total_height = handle_radius * 2.0 + 4.0;
        let slider_width = self.width.unwrap_or(ui.available_width().min(200.0));

        // Layout prefix/suffix labels
        let prefix_galley = self.prefix.as_ref().map(|p| {
            ui.painter().layout_no_wrap(
                p.clone(),
                egui::FontId::proportional(12.0),
                theme.muted_foreground,
            )
        });
        let suffix_galley = self.suffix.as_ref().map(|s| {
            ui.painter().layout_no_wrap(
                s.clone(),
                egui::FontId::proportional(12.0),
                theme.muted_foreground,
            )
        });

        let prefix_w = prefix_galley.as_ref().map(|g| g.size().x + 6.0).unwrap_or(0.0);
        let suffix_w = suffix_galley.as_ref().map(|g| g.size().x + 6.0).unwrap_or(0.0);
        let total_width = prefix_w + slider_width + suffix_w;

        let desired = egui::vec2(total_width, total_height);
        let (full_rect, response) = ui.allocate_exact_size(desired, egui::Sense::click_and_drag());

        // Get current value as f64
        let current_val = match &self.value {
            super::slider::SliderValue::F64(v) => **v,
            super::slider::SliderValue::F32(v) => **v as f64,
        };

        let range_start = *self.range.start();
        let range_end = *self.range.end();
        let range_span = range_end - range_start;

        // Slider track rect (between prefix and suffix)
        let track_rect = egui::Rect::from_min_max(
            egui::pos2(full_rect.min.x + prefix_w, full_rect.min.y),
            egui::pos2(full_rect.max.x - suffix_w, full_rect.max.y),
        );

        // Handle drag
        let mut new_val = current_val;
        if (response.dragged() || response.clicked())
            && let Some(pos) = response.interact_pointer_pos()
        {
            let usable_min = track_rect.min.x + handle_radius;
            let usable_max = track_rect.max.x - handle_radius;
            let t = ((pos.x - usable_min) / (usable_max - usable_min)).clamp(0.0, 1.0);
            new_val = range_start + t as f64 * range_span;

            if let Some(step) = self.step {
                new_val = (new_val / step).round() * step;
            }

            new_val = new_val.clamp(range_start, range_end);
        }

        // Write back
        match self.value {
            super::slider::SliderValue::F64(v) => *v = new_val,
            super::slider::SliderValue::F32(v) => *v = new_val as f32,
        }

        if ui.is_rect_visible(full_rect) {
            let painter = ui.painter();

            // Paint prefix
            if let Some(galley) = prefix_galley {
                painter.galley(
                    egui::pos2(
                        full_rect.min.x,
                        full_rect.center().y - galley.size().y / 2.0,
                    ),
                    galley,
                    theme.muted_foreground,
                );
            }

            // Paint suffix
            if let Some(galley) = suffix_galley {
                painter.galley(
                    egui::pos2(
                        full_rect.max.x - galley.size().x,
                        full_rect.center().y - galley.size().y / 2.0,
                    ),
                    galley,
                    theme.muted_foreground,
                );
            }

            let track_y = track_rect.center().y;
            let usable_min = track_rect.min.x + handle_radius;
            let usable_max = track_rect.max.x - handle_radius;

            let t = if range_span > 0.0 {
                ((new_val - range_start) / range_span) as f32
            } else {
                0.0
            };
            let handle_x = usable_min + (usable_max - usable_min) * t;

            let track_cr = (track_height / 2.0).round().min(255.0) as u8;

            // Track background
            let track_bg_rect = egui::Rect::from_min_max(
                egui::pos2(usable_min, track_y - track_height / 2.0),
                egui::pos2(usable_max, track_y + track_height / 2.0),
            );
            painter.rect_filled(
                track_bg_rect,
                egui::CornerRadius::same(track_cr),
                style.track_color,
            );

            // Fill
            let fill_rect = egui::Rect::from_min_max(
                egui::pos2(usable_min, track_y - track_height / 2.0),
                egui::pos2(handle_x, track_y + track_height / 2.0),
            );
            painter.rect_filled(
                fill_rect,
                egui::CornerRadius::same(track_cr),
                style.fill_color,
            );

            // Handle
            let handle_center = egui::pos2(handle_x, track_y);
            painter.circle_filled(handle_center, handle_radius, style.handle_fill);
            painter.circle_stroke(
                handle_center,
                handle_radius,
                egui::Stroke::new(2.0, style.handle_border),
            );

            // Focus ring
            if response.has_focus() {
                let handle_rect = egui::Rect::from_center_size(
                    handle_center,
                    egui::vec2(handle_radius * 2.0, handle_radius * 2.0),
                );
                crate::paint::paint_focus_ring::paint_focus_ring(
                    painter,
                    handle_rect,
                    handle_radius,
                    theme.ring,
                );
            }
        }

        response
    }
}
