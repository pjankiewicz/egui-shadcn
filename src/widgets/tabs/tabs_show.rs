//! Show method for Tabs — renders tab bar and content.

impl super::tabs::Tabs {
    /// Shows the tab bar and content. `selected` is the currently active tab index.
    /// Calls `content(ui, selected_index)` for the active tab's body.
    pub fn show(
        self,
        ui: &mut egui::Ui,
        selected: &mut usize,
        content: impl FnOnce(&mut egui::Ui, usize),
    ) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());
        let cr = theme.radius.round() as u8;

        

        ui
            .vertical(|ui| {
                // Tab bar
                let tab_frame = egui::Frame::NONE
                    .fill(theme.muted)
                    .inner_margin(egui::Margin::same(2))
                    .corner_radius(egui::CornerRadius::same(cr));

                tab_frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 2.0;
                        for (idx, label) in self.labels.iter().enumerate() {
                            let is_active = idx == *selected;
                            let response = Self::render_tab(ui, &theme, label, is_active, cr);
                            if response.clicked() {
                                *selected = idx;
                                ui.ctx().request_repaint();
                            }
                        }
                    });
                });

                ui.add_space(8.0);

                // Content area
                content(ui, *selected);
            })
            .response
    }

    fn render_tab(
        ui: &mut egui::Ui,
        theme: &crate::theme::shadcn_theme::ShadcnTheme,
        label: &str,
        is_active: bool,
        cr: u8,
    ) -> egui::Response {
        let font_size: f32 = 13.0;
        let h_pad: f32 = 12.0;
        let height: f32 = 28.0;

        let galley = ui.painter().layout_no_wrap(
            label.to_owned(),
            egui::FontId::proportional(font_size),
            theme.foreground,
        );

        let desired = egui::vec2(galley.size().x + h_pad * 2.0, height);
        let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let inner_cr = egui::CornerRadius::same(cr.saturating_sub(1));

            let (bg, fg) = if is_active {
                (theme.background, theme.foreground)
            } else if response.is_pointer_button_down_on() {
                (
                    crate::paint::interpolate_color::interpolate_color(
                        theme.background,
                        theme.accent,
                        0.65,
                    ),
                    theme.foreground,
                )
            } else if response.hovered() {
                (
                    egui::Color32::from_rgba_unmultiplied(
                        theme.background.r(),
                        theme.background.g(),
                        theme.background.b(),
                        128,
                    ),
                    theme.foreground,
                )
            } else {
                (egui::Color32::TRANSPARENT, theme.muted_foreground)
            };

            painter.rect_filled(rect, inner_cr, bg);

            if is_active {
                painter.rect_stroke(
                    rect,
                    inner_cr,
                    egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_unmultiplied(
                            theme.foreground.r(),
                            theme.foreground.g(),
                            theme.foreground.b(),
                            13,
                        ),
                    ),
                    egui::epaint::StrokeKind::Inside,
                );
            }

            let text_pos = egui::pos2(
                rect.center().x - galley.size().x / 2.0,
                rect.center().y - galley.size().y / 2.0,
            );
            painter.galley(text_pos, galley, fg);
        }

        if response.hovered() && !is_active {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        response
    }
}

// ---------------------------------------------------------------------------
// IconTabs — icon+tooltip variant
// ---------------------------------------------------------------------------

impl super::tabs::IconTabs {
    pub fn show(
        self,
        ui: &mut egui::Ui,
        selected: &mut usize,
        content: impl FnOnce(&mut egui::Ui, usize),
    ) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());
        let cr = theme.radius.round() as u8;

        

        ui
            .vertical(|ui| {
                let tab_frame = egui::Frame::NONE
                    .fill(theme.muted)
                    .inner_margin(egui::Margin::same(2))
                    .corner_radius(egui::CornerRadius::same(cr));

                tab_frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 2.0;
                        for (idx, entry) in self.entries.iter().enumerate() {
                            let is_active = idx == *selected;
                            let response = Self::render_icon_tab(ui, &theme, entry, is_active, cr);
                            if response.clicked() {
                                *selected = idx;
                                ui.ctx().request_repaint();
                            }
                        }
                    });
                });

                ui.add_space(8.0);
                content(ui, *selected);
            })
            .response
    }

    fn render_icon_tab(
        ui: &mut egui::Ui,
        theme: &crate::theme::shadcn_theme::ShadcnTheme,
        entry: &super::tabs::TabEntry,
        is_active: bool,
        cr: u8,
    ) -> egui::Response {
        let size: f32 = 28.0;
        let icon_size: f32 = 14.0;

        let desired = egui::vec2(size, size);
        let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let inner_cr = egui::CornerRadius::same(cr.saturating_sub(1));

            let (bg, fg) = if is_active {
                (theme.background, theme.foreground)
            } else if response.is_pointer_button_down_on() {
                (
                    crate::paint::interpolate_color::interpolate_color(
                        theme.background,
                        theme.accent,
                        0.65,
                    ),
                    theme.foreground,
                )
            } else if response.hovered() {
                (
                    egui::Color32::from_rgba_unmultiplied(
                        theme.background.r(),
                        theme.background.g(),
                        theme.background.b(),
                        128,
                    ),
                    theme.foreground,
                )
            } else {
                (egui::Color32::TRANSPARENT, theme.muted_foreground)
            };

            painter.rect_filled(rect, inner_cr, bg);

            if is_active {
                painter.rect_stroke(
                    rect,
                    inner_cr,
                    egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_unmultiplied(
                            theme.foreground.r(),
                            theme.foreground.g(),
                            theme.foreground.b(),
                            13,
                        ),
                    ),
                    egui::epaint::StrokeKind::Inside,
                );
            }

            match entry {
                super::tabs::TabEntry::Text(label) => {
                    let galley =
                        painter.layout_no_wrap(label.clone(), egui::FontId::proportional(12.0), fg);
                    let text_pos = egui::pos2(
                        rect.center().x - galley.size().x / 2.0,
                        rect.center().y - galley.size().y / 2.0,
                    );
                    painter.galley(text_pos, galley, fg);
                }
                super::tabs::TabEntry::Icon { icon, .. } => {
                    let icon_rect = egui::Rect::from_center_size(
                        rect.center(),
                        egui::vec2(icon_size, icon_size),
                    );
                    crate::icons::paint_icon::paint_icon(painter, icon_rect, icon, fg);
                }
            }
        }

        if response.hovered() && !is_active {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        // Show tooltip for icon entries
        if let super::tabs::TabEntry::Icon { tooltip, .. } = entry {
            response.clone().on_hover_text(tooltip);
        }

        response
    }
}
