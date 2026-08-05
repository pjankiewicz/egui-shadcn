//! Show method for Calendar — renders a month grid with selectable days.

impl super::calendar::Calendar {
    /// Shows the calendar with mutable year/month/day. Navigation arrows directly
    /// mutate `year` and `month`. Returns the newly selected day if clicked.
    pub fn show(
        self,
        ui: &mut egui::Ui,
        year: &mut i32,
        month: &mut u32,
        selected_day: &mut u32,
    ) -> Option<u32> {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());
        let mut clicked_day = None;

        *month = (*month).clamp(1, 12);
        let days_in_month = Self::days_in_month(*year, *month);
        let first_weekday = Self::day_of_week(*year, *month, 1);

        let cell_size: f32 = 36.0;
        let font_size: f32 = 13.0;
        let cr = egui::CornerRadius::same(theme.radius.round() as u8);

        ui.vertical(|ui| {
            // Month/year header with navigation arrows
            let nav_icon_size: f32 = 14.0;
            ui.horizontal(|ui| {
                let (prev_rect, prev_resp) = ui.allocate_exact_size(
                    egui::vec2(nav_icon_size, nav_icon_size),
                    egui::Sense::click(),
                );
                if ui.is_rect_visible(prev_rect) {
                    crate::icons::paint_icon::paint_icon(
                        ui.painter(),
                        prev_rect,
                        &crate::icons::lucide_icon::LucideIcon::ChevronLeft,
                        theme.muted_foreground,
                    );
                }
                if prev_resp.clicked() {
                    if *month == 1 {
                        *month = 12;
                        *year -= 1;
                    } else {
                        *month -= 1;
                    }
                    ui.ctx().request_repaint();
                }

                ui.label(
                    egui::RichText::new(format!(
                        "{} {}",
                        Self::month_name(*month),
                        year
                    ))
                    .color(theme.foreground)
                    .size(14.0)
                    .strong(),
                );

                let (next_rect, next_resp) = ui.allocate_exact_size(
                    egui::vec2(nav_icon_size, nav_icon_size),
                    egui::Sense::click(),
                );
                if ui.is_rect_visible(next_rect) {
                    crate::icons::paint_icon::paint_icon(
                        ui.painter(),
                        next_rect,
                        &crate::icons::lucide_icon::LucideIcon::ChevronRight,
                        theme.muted_foreground,
                    );
                }
                if next_resp.clicked() {
                    if *month == 12 {
                        *month = 1;
                        *year += 1;
                    } else {
                        *month += 1;
                    }
                    ui.ctx().request_repaint();
                }
            });

            ui.add_space(8.0);

            // Day-of-week headers
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                for day_name in ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"] {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(cell_size, cell_size * 0.75),
                        egui::Sense::hover(),
                    );
                    if ui.is_rect_visible(rect) {
                        let galley = ui.painter().layout_no_wrap(
                            day_name.to_owned(),
                            egui::FontId::proportional(12.0),
                            theme.muted_foreground,
                        );
                        let pos = egui::pos2(
                            rect.center().x - galley.size().x / 2.0,
                            rect.center().y - galley.size().y / 2.0,
                        );
                        ui.painter().galley(pos, galley, theme.muted_foreground);
                    }
                }
            });

            // Day grid
            let mut day_counter: u32 = 0;
            let total_cells = first_weekday + days_in_month;
            let rows = total_cells.div_ceil(7);

            for row in 0..rows {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;
                    for col in 0..7_u32 {
                        let cell_idx = row * 7 + col;
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(cell_size, cell_size),
                            egui::Sense::click(),
                        );

                        if cell_idx >= first_weekday && day_counter < days_in_month {
                            day_counter += 1;
                            let day = day_counter;
                            let is_selected = day == *selected_day;

                            if ui.is_rect_visible(rect) {
                                let painter = ui.painter();

                                let (bg, fg) = if is_selected {
                                    (theme.primary, theme.primary_foreground)
                                } else if response.hovered() {
                                    (theme.accent, theme.accent_foreground)
                                } else {
                                    (egui::Color32::TRANSPARENT, theme.foreground)
                                };

                                if bg != egui::Color32::TRANSPARENT {
                                    painter.rect_filled(rect, cr, bg);
                                }

                                let galley = painter.layout_no_wrap(
                                    format!("{}", day),
                                    egui::FontId::proportional(font_size),
                                    fg,
                                );
                                let pos = egui::pos2(
                                    rect.center().x - galley.size().x / 2.0,
                                    rect.center().y - galley.size().y / 2.0,
                                );
                                painter.galley(pos, galley, fg);
                            }

                            if response.clicked() {
                                *selected_day = day;
                                clicked_day = Some(day);
                                ui.ctx().request_repaint();
                            }
                        }
                    }
                });
            }
        });

        clicked_day
    }

    /// Returns number of days in the given month.
    fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    /// Returns the day of week for a date (0=Monday, 6=Sunday). Zeller-like.
    fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
        let (y, m) = if month <= 2 {
            (year - 1, month + 12)
        } else {
            (year, month)
        };
        let q = day as i32;
        let k = y % 100;
        let j = y / 100;
        let m = m as i32;
        let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        // Convert from Zeller (0=Sat) to Monday-based (0=Mon)
        (((h + 5) % 7 + 7) % 7) as u32
    }

    fn month_name(month: u32) -> &'static str {
        match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }
}
