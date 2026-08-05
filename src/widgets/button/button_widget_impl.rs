//! Widget trait implementation for Button.

impl egui::Widget for super::button::Button<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());

        // Check if inside a ButtonGroup and claim an index
        let group_key =
            crate::widgets::button_group::button_group::ButtonGroup::context_key();
        let group_info = ui.ctx().data_mut(|d| {
            d.get_temp::<crate::widgets::button_group::button_group_context::ButtonGroupContext>(
                group_key,
            )
            .and_then(|mut ctx| {
                if !ctx.active {
                    return None;
                }
                let index = ctx.current_index;
                let cached_count = ctx.cached_count;
                let cr = ctx.corner_radius;
                ctx.current_index += 1;
                d.insert_temp(group_key, ctx);
                Some((index, cached_count, cr))
            })
        });
        let in_group = group_info.is_some();

        let mut style = super::button_variant_style::resolve_button_style(
            &theme,
            self.variant,
            self.size,
            false,
            false,
            !ui.is_enabled(),
        );

        // When full_width, adopt the UI's own sizing so the button matches
        // native egui menu items (same height, font, padding).
        if self.full_width {
            style.height = ui.spacing().interact_size.y;
            if let Some(font_id) = ui.style().text_styles.get(&egui::TextStyle::Button) {
                style.font_size = font_id.size;
            }
            style.h_padding = ui.spacing().button_padding.x;
        }

        let text_string = self.text.text().to_owned();
        let is_icon_only = text_string.is_empty() && self.icon.is_some();
        let has_icon = self.icon.is_some();
        let has_text = !text_string.is_empty();
        let has_shortcut = self.shortcut_text.is_some();

        let text_galley = ui.painter().layout_no_wrap(
            text_string.clone(),
            egui::FontId::proportional(style.font_size),
            style.fg,
        );

        let shortcut_galley = self.shortcut_text.as_ref().map(|st| {
            ui.painter().layout_no_wrap(
                st.clone(),
                egui::FontId::proportional(style.font_size - 1.0),
                theme.muted_foreground,
            )
        });

        let icon_size = style.height * 0.5;
        let icon_gap = 6.0;
        let shortcut_gap = 16.0;

        let shortcut_width = shortcut_galley
            .as_ref()
            .map(|g| shortcut_gap + g.size().x)
            .unwrap_or(0.0);

        let content_width = if is_icon_only {
            style.height
        } else if has_icon && has_text {
            style.h_padding + icon_size + icon_gap + text_galley.size().x + shortcut_width + style.h_padding
        } else {
            text_galley.size().x + shortcut_width + style.h_padding * 2.0
        };

        let desired = egui::vec2(content_width, style.height);

        // full_width: use allocate_at_least so the button stretches to fill
        // the menu/list width without inflating the menu's own desired size.
        let (rect, response) = if self.full_width {
            ui.allocate_at_least(desired, egui::Sense::click())
        } else {
            ui.allocate_exact_size(desired, egui::Sense::click())
        };

        // Record boundary and rect in group context
        if in_group {
            ui.ctx().data_mut(|d| {
                if let Some(mut ctx) = d.get_temp::<crate::widgets::button_group::button_group_context::ButtonGroupContext>(group_key)
                    && ctx.active
                {
                    ctx.boundaries.push(rect.max.x);
                    ctx.group_rect = Some(match ctx.group_rect {
                        Some(r) => r.union(rect),
                        None => rect,
                    });
                    d.insert_temp(group_key, ctx);
                }
            });
        }

        // Re-resolve with actual interaction state
        let mut style = super::button_variant_style::resolve_button_style(
            &theme,
            self.variant,
            self.size,
            response.hovered(),
            response.is_pointer_button_down_on(),
            !ui.is_enabled(),
        );

        // Apply full_width overrides again (height/font/padding from UI context)
        if self.full_width {
            style.height = ui.spacing().interact_size.y;
            if let Some(font_id) = ui.style().text_styles.get(&egui::TextStyle::Button) {
                style.font_size = font_id.size;
            }
            style.h_padding = ui.spacing().button_padding.x;
        }

        // Selected override: use accent background
        if self.selected {
            style.bg = theme.accent;
            style.fg = theme.accent_foreground;
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let cr = group_corner_radius(&group_info, style.corner_radius);

            // Background
            painter.rect_filled(rect, cr, style.bg);

            // Border (skip when inside a button group — the group draws borders)
            if !in_group
                && let Some(border_color) = style.border
            {
                painter.rect_stroke(
                    rect,
                    cr,
                    egui::Stroke::new(1.0, border_color),
                    egui::epaint::StrokeKind::Inside,
                );
            }

            if is_icon_only {
                // Icon centered
                if let Some(ref icon) = self.icon {
                    let icon_rect = egui::Rect::from_center_size(
                        rect.center(),
                        egui::vec2(icon_size, icon_size),
                    );
                    crate::icons::paint_icon::paint_icon(painter, icon_rect, icon, style.fg);
                }
            } else if has_icon && has_text {
                // Icon + text (left-aligned when shortcut present)
                if has_shortcut {
                    let x = rect.min.x + style.h_padding;
                    if let Some(ref icon) = self.icon {
                        let icon_rect = egui::Rect::from_min_size(
                            egui::pos2(x, rect.center().y - icon_size / 2.0),
                            egui::vec2(icon_size, icon_size),
                        );
                        crate::icons::paint_icon::paint_icon(painter, icon_rect, icon, style.fg);
                    }
                    let text_pos = egui::pos2(
                        x + icon_size + icon_gap,
                        rect.center().y - text_galley.size().y / 2.0,
                    );
                    painter.galley(text_pos, text_galley, style.fg);
                } else {
                    let total_w = icon_size + icon_gap + text_galley.size().x;
                    let start_x = rect.center().x - total_w / 2.0;
                    if let Some(ref icon) = self.icon {
                        let icon_rect = egui::Rect::from_min_size(
                            egui::pos2(start_x, rect.center().y - icon_size / 2.0),
                            egui::vec2(icon_size, icon_size),
                        );
                        crate::icons::paint_icon::paint_icon(painter, icon_rect, icon, style.fg);
                    }
                    let text_pos = egui::pos2(
                        start_x + icon_size + icon_gap,
                        rect.center().y - text_galley.size().y / 2.0,
                    );
                    painter.galley(text_pos, text_galley, style.fg);
                }
            } else {
                // Text only (left-aligned when shortcut present or full_width)
                if has_shortcut || self.full_width {
                    let text_pos = egui::pos2(
                        rect.min.x + style.h_padding,
                        rect.center().y - text_galley.size().y / 2.0,
                    );
                    painter.galley(text_pos, text_galley, style.fg);
                } else {
                    let text_pos = egui::pos2(
                        rect.center().x - text_galley.size().x / 2.0,
                        rect.center().y - text_galley.size().y / 2.0,
                    );
                    painter.galley(text_pos, text_galley, style.fg);
                }
            }

            // Shortcut text (right-aligned, muted)
            if let Some(shortcut_galley) = shortcut_galley {
                let shortcut_pos = egui::pos2(
                    rect.max.x - style.h_padding - shortcut_galley.size().x,
                    rect.center().y - shortcut_galley.size().y / 2.0,
                );
                painter.galley(shortcut_pos, shortcut_galley, theme.muted_foreground);
            }

            // Underline for Link variant
            if style.underline && response.hovered() {
                let galley2 = painter.layout_no_wrap(
                    text_string,
                    egui::FontId::proportional(style.font_size),
                    style.fg,
                );
                let text_pos = egui::pos2(
                    rect.center().x - galley2.size().x / 2.0,
                    rect.center().y - galley2.size().y / 2.0,
                );
                let underline_y = text_pos.y + galley2.size().y;
                painter.hline(
                    text_pos.x..=text_pos.x + galley2.size().x,
                    underline_y,
                    egui::Stroke::new(1.0, style.fg),
                );
            }

            // Focus ring
            if response.has_focus() {
                crate::paint::paint_focus_ring::paint_focus_ring(
                    painter,
                    rect,
                    style.corner_radius,
                    theme.ring,
                );
            }
        }

        response
    }
}

/// Computes per-corner radius for a button based on its group position.
/// First button gets left rounding, last gets right rounding, middle gets none.
fn group_corner_radius(
    group_info: &Option<(usize, usize, f32)>,
    default_cr: f32,
) -> egui::CornerRadius {
    match group_info {
        Some((index, cached_count, group_cr)) => {
            let r = group_cr.round() as u8;
            let is_first = *index == 0;
            let is_last = *cached_count > 0 && *index == *cached_count - 1;
            egui::CornerRadius {
                nw: if is_first { r } else { 0 },
                sw: if is_first { r } else { 0 },
                ne: if is_last { r } else { 0 },
                se: if is_last { r } else { 0 },
            }
        }
        None => egui::CornerRadius::same(default_cr.round() as u8),
    }
}
