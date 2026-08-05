//! Show method for Command — renders a command palette overlay.

/// Which filtered row the keyboard is on, and the query it was chosen under.
///
/// Kept in `Context` temp data so it survives between frames without the
/// caller having to own it — matching how the rest of this crate persists
/// per-widget state.
#[derive(Clone, Default)]
struct NavState {
    active: usize,
    /// Last query this `active` was valid for. When the filter changes the
    /// row under the cursor becomes meaningless, so selection returns to the
    /// top rather than landing on whatever happens to sit at that offset.
    query: String,
}

impl super::command::Command {
    /// Shows the command palette when `open` is true.
    /// `search` holds the filter text. Returns the index of selected command if any.
    pub fn show(
        self,
        ctx: &egui::Context,
        open: &mut bool,
        search: &mut String,
    ) -> Option<usize> {
        if !*open {
            return None;
        }

        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ctx);
        let mut selected = None;

        // Backdrop
        let screen = ctx.input(|i| i.viewport_rect());
        let backdrop_layer = egui::LayerId::new(
            egui::Order::Middle,
            egui::Id::new("command_backdrop"),
        );
        ctx.layer_painter(backdrop_layer).rect_filled(
            screen,
            egui::CornerRadius::ZERO,
            egui::Color32::from_black_alpha(60),
        );

        // Backdrop click to close
        let backdrop_resp = egui::Area::new(egui::Id::new("command_backdrop_sense"))
            .order(egui::Order::Middle)
            .anchor(egui::Align2::LEFT_TOP, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                let (_, response) =
                    ui.allocate_exact_size(screen.size(), egui::Sense::click());
                response
            });

        if backdrop_resp.inner.clicked() {
            *open = false;
            search.clear();
            ctx.request_repaint();
            return None;
        }

        // Escape to close
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            *open = false;
            search.clear();
            ctx.request_repaint();
            return None;
        }

        // Which rows the current filter leaves, as indices into `self.items`.
        let query = search.to_lowercase();
        let visible: Vec<usize> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, (group, label))| {
                query.is_empty()
                    || label.to_lowercase().contains(&query)
                    || group.to_lowercase().contains(&query)
            })
            .map(|(idx, _)| idx)
            .collect();

        // Keyboard navigation.
        //
        // These keys are *consumed* rather than merely read, and that happens
        // here — before the search `Input` below requests focus and renders.
        // A focused `TextEdit` claims Up/Down/Home/End/Enter for cursor
        // movement, so reading them later would mean the arrows moved the
        // caret instead of the selection.
        let nav_id = egui::Id::new("command_palette_nav");
        let mut nav: NavState = ctx.data(|d| d.get_temp(nav_id)).unwrap_or_default();
        if nav.query != query {
            nav.query = query.clone();
            nav.active = 0;
        }

        let mut activate = false;
        if !visible.is_empty() {
            nav.active = nav.active.min(visible.len() - 1);
            let last = visible.len() - 1;

            ctx.input_mut(|input| {
                // Wrapping: a palette is a short list, and stopping dead at
                // the ends makes reaching the bottom entry needlessly slow.
                if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                    nav.active = if nav.active >= last { 0 } else { nav.active + 1 };
                }
                if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                    nav.active = if nav.active == 0 { last } else { nav.active - 1 };
                }
                if input.consume_key(egui::Modifiers::NONE, egui::Key::Home) {
                    nav.active = 0;
                }
                if input.consume_key(egui::Modifiers::NONE, egui::Key::End) {
                    nav.active = last;
                }
                if input.consume_key(egui::Modifiers::NONE, egui::Key::Enter) {
                    activate = true;
                }
            });
        }

        let active_item = visible.get(nav.active).copied();
        ctx.data_mut(|d| d.insert_temp(nav_id, nav));

        if activate && let Some(idx) = active_item {
            *open = false;
            search.clear();
            ctx.data_mut(|d| d.remove_temp::<NavState>(nav_id));
            ctx.request_repaint();
            return Some(idx);
        }

        let cr = (theme.radius + 2.0).round() as u8;

        egui::Area::new(egui::Id::new("command_palette"))
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, -60.0))
            .show(ctx, |ui| {
                let frame = egui::Frame::NONE
                    .fill(theme.popover)
                    .inner_margin(egui::Margin::same(0))
                    .corner_radius(egui::CornerRadius::same(cr))
                    .stroke(egui::Stroke::new(1.0, theme.border))
                    .shadow(egui::Shadow {
                        offset: [0, 8],
                        blur: 24,
                        spread: 0,
                        color: egui::Color32::from_black_alpha(12),
                    });

                frame.show(ui, |ui| {
                    ui.set_min_width(480.0);
                    ui.set_max_width(480.0);

                    // Search input
                    let input_frame = egui::Frame::NONE.inner_margin(egui::Margin {
                        left: 12,
                        right: 12,
                        top: 12,
                        bottom: 12,
                    });

                    input_frame.show(ui, |ui| {
                        let input_resp =
                            crate::widgets::input::input::Input::new(search)
                                .placeholder(&self.placeholder)
                                .desired_width(ui.available_width())
                                .show(ui);
                        input_resp.request_focus();
                    });

                    // Divider
                    let avail = ui.available_rect_before_wrap();
                    ui.painter().hline(
                        avail.min.x..=avail.max.x,
                        avail.min.y,
                        egui::Stroke::new(1.0, theme.border),
                    );
                    ui.add_space(1.0);

                    // Command list
                    let results_frame =
                        egui::Frame::NONE.inner_margin(egui::Margin::same(8));

                    results_frame.show(ui, |ui| {
                        let mut current_group = String::new();
                        let mut any_shown = false;

                        // Bounded so a long list cannot grow the palette past
                        // the viewport; the active row is scrolled into view
                        // below, which is what makes arrowing past the fold work.
                        egui::ScrollArea::vertical()
                        .max_height(320.0)
                        .show(ui, |ui| {
                        for (idx, (group, label)) in self.items.iter().enumerate() {
                            if !visible.contains(&idx) {
                                continue;
                            }

                            any_shown = true;

                            if *group != current_group {
                                if !current_group.is_empty() {
                                    ui.add_space(4.0);
                                }
                                ui.label(
                                    egui::RichText::new(group)
                                        .color(theme.muted_foreground)
                                        .size(12.0)
                                        .strong(),
                                );
                                ui.add_space(2.0);
                                current_group = group.clone();
                            }

                            let galley = ui.painter().layout_no_wrap(
                                label.clone(),
                                egui::FontId::proportional(14.0),
                                theme.popover_foreground,
                            );
                            let desired = egui::vec2(
                                ui.available_width(),
                                galley.size().y + 8.0,
                            );
                            let (rect, r) =
                                ui.allocate_exact_size(desired, egui::Sense::click());

                            // Keyboard and mouse share one highlight, so the
                            // row Enter would run always looks selected.
                            let is_active = active_item == Some(idx);
                            if r.hovered() || is_active {
                                ui.painter().rect_filled(
                                    rect,
                                    egui::CornerRadius::same(4),
                                    theme.accent,
                                );
                            }
                            if is_active {
                                ui.scroll_to_rect(rect, None);
                            }

                            if ui.is_rect_visible(rect) {
                                ui.painter().galley(
                                    egui::pos2(
                                        rect.min.x + 8.0,
                                        rect.center().y - galley.size().y / 2.0,
                                    ),
                                    galley,
                                    theme.popover_foreground,
                                );
                            }

                            if r.clicked() {
                                selected = Some(idx);
                                *open = false;
                                search.clear();
                                ctx.request_repaint();
                            }
                        }
                        });

                        if !any_shown {
                            ui.label(
                                egui::RichText::new("No results found.")
                                    .color(theme.muted_foreground)
                                    .size(14.0),
                            );
                        }
                    });
                });
            });

        selected
    }
}

#[cfg(test)]
mod tests {
    //! Keyboard navigation. A palette you must reach for the mouse to use is
    //! worse than the menu it replaces, so these cover the paths that make it
    //! keyboard-only-usable.

    fn items() -> Vec<(String, String)> {
        [
            ("File", "New Project"),
            ("File", "Open Project"),
            ("Run", "Playtest Debug"),
            ("Run", "Smoke Test"),
        ]
        .into_iter()
        .map(|(g, l)| (g.to_owned(), l.to_owned()))
        .collect()
    }

    fn key(key: egui::Key) -> egui::RawInput {
        let mut input = egui::RawInput::default();
        for pressed in [true, false] {
            input.events.push(egui::Event::Key {
                key,
                physical_key: None,
                pressed,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            });
        }
        input
    }

    /// Runs one frame per entry in `keys` (`None` meaning "no input"), and
    /// returns whatever the palette selected along the way.
    fn drive(search: &mut String, keys: &[Option<egui::Key>]) -> Option<usize> {
        let ctx = egui::Context::default();
        ctx.set_fonts(egui::FontDefinitions::empty());
        let mut open = true;
        let mut picked = None;

        for pressed in keys {
            let input = pressed.map_or_else(egui::RawInput::default, key);
            let _ = ctx.run_ui(input, |ui| {
                let got = super::super::command::Command::new(items())
                    .show(ui.ctx(), &mut open, search);
                if got.is_some() {
                    picked = got;
                }
            });
        }

        picked
    }

    #[test]
    fn enter_runs_the_first_row_by_default() {
        let mut search = String::new();
        assert_eq!(drive(&mut search, &[None, Some(egui::Key::Enter)]), Some(0));
    }

    #[test]
    fn arrow_down_moves_the_selection() {
        let mut search = String::new();
        let picked = drive(
            &mut search,
            &[
                None,
                Some(egui::Key::ArrowDown),
                Some(egui::Key::ArrowDown),
                Some(egui::Key::Enter),
            ],
        );
        assert_eq!(picked, Some(2));
    }

    /// Arrowing up from the top lands on the last row rather than stalling.
    #[test]
    fn selection_wraps_at_both_ends() {
        let mut search = String::new();
        assert_eq!(
            drive(&mut search, &[None, Some(egui::Key::ArrowUp), Some(egui::Key::Enter)]),
            Some(3),
        );

        let mut search = String::new();
        assert_eq!(
            drive(
                &mut search,
                &[
                    None,
                    Some(egui::Key::End),
                    Some(egui::Key::ArrowDown),
                    Some(egui::Key::Enter),
                ],
            ),
            Some(0),
        );
    }

    #[test]
    fn home_and_end_jump_to_the_edges() {
        let mut search = String::new();
        assert_eq!(
            drive(&mut search, &[None, Some(egui::Key::End), Some(egui::Key::Enter)]),
            Some(3),
        );

        let mut search = String::new();
        assert_eq!(
            drive(
                &mut search,
                &[
                    None,
                    Some(egui::Key::ArrowDown),
                    Some(egui::Key::Home),
                    Some(egui::Key::Enter),
                ],
            ),
            Some(0),
        );
    }

    /// Enter must select from the *filtered* list, not the unfiltered one —
    /// the bug that makes a palette run the wrong command.
    #[test]
    fn enter_selects_within_the_filtered_results() {
        let mut search = "playtest".to_owned();
        assert_eq!(drive(&mut search, &[None, Some(egui::Key::Enter)]), Some(2));
    }

    /// After typing, the highlight must return to the top; leaving it at an
    /// offset would run whatever coincidentally sits there.
    #[test]
    fn changing_the_filter_resets_the_selection() {
        let ctx = egui::Context::default();
        ctx.set_fonts(egui::FontDefinitions::empty());
        let mut open = true;
        let mut search = String::new();
        let mut picked = None;

        // Move down twice with no filter…
        for input in [
            egui::RawInput::default(),
            key(egui::Key::ArrowDown),
            key(egui::Key::ArrowDown),
        ] {
            let _ = ctx.run_ui(input, |ui| {
                super::super::command::Command::new(items()).show(ui.ctx(), &mut open, &mut search);
            });
        }

        // …then narrow the list and hit Enter.
        search.push_str("run");
        for input in [egui::RawInput::default(), key(egui::Key::Enter)] {
            let _ = ctx.run_ui(input, |ui| {
                let got = super::super::command::Command::new(items())
                    .show(ui.ctx(), &mut open, &mut search);
                if got.is_some() {
                    picked = got;
                }
            });
        }

        // "Playtest Debug" is the first Run entry, not the third overall.
        assert_eq!(picked, Some(2));
    }

    #[test]
    fn escape_closes_without_selecting() {
        let ctx = egui::Context::default();
        ctx.set_fonts(egui::FontDefinitions::empty());
        let mut open = true;
        let mut search = String::new();

        for input in [egui::RawInput::default(), key(egui::Key::Escape)] {
            let _ = ctx.run_ui(input, |ui| {
                let got = super::super::command::Command::new(items())
                    .show(ui.ctx(), &mut open, &mut search);
                assert!(got.is_none());
            });
        }

        assert!(!open);
    }
}
