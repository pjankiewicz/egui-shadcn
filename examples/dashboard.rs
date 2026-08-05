//! Dashboard example — recreation of the shadcn/ui dashboard with egui-shadcn widgets.

// ---------------------------------------------------------------------------
// Chart data
// ---------------------------------------------------------------------------

struct ChartPoint {
    date: &'static str,
    desktop: f32,
    mobile: f32,
}

const CHART_DATA: &[ChartPoint] = &[
    ChartPoint { date: "Jun 1", desktop: 178.0, mobile: 200.0 },
    ChartPoint { date: "Jun 2", desktop: 470.0, mobile: 410.0 },
    ChartPoint { date: "Jun 3", desktop: 103.0, mobile: 160.0 },
    ChartPoint { date: "Jun 4", desktop: 439.0, mobile: 380.0 },
    ChartPoint { date: "Jun 5", desktop: 88.0, mobile: 140.0 },
    ChartPoint { date: "Jun 6", desktop: 294.0, mobile: 250.0 },
    ChartPoint { date: "Jun 7", desktop: 323.0, mobile: 370.0 },
    ChartPoint { date: "Jun 8", desktop: 385.0, mobile: 320.0 },
    ChartPoint { date: "Jun 9", desktop: 438.0, mobile: 480.0 },
    ChartPoint { date: "Jun 10", desktop: 155.0, mobile: 200.0 },
    ChartPoint { date: "Jun 11", desktop: 92.0, mobile: 150.0 },
    ChartPoint { date: "Jun 12", desktop: 492.0, mobile: 420.0 },
    ChartPoint { date: "Jun 13", desktop: 81.0, mobile: 130.0 },
    ChartPoint { date: "Jun 14", desktop: 426.0, mobile: 380.0 },
    ChartPoint { date: "Jun 15", desktop: 307.0, mobile: 350.0 },
    ChartPoint { date: "Jun 16", desktop: 371.0, mobile: 310.0 },
    ChartPoint { date: "Jun 17", desktop: 475.0, mobile: 520.0 },
    ChartPoint { date: "Jun 18", desktop: 107.0, mobile: 170.0 },
    ChartPoint { date: "Jun 19", desktop: 341.0, mobile: 290.0 },
    ChartPoint { date: "Jun 20", desktop: 408.0, mobile: 450.0 },
    ChartPoint { date: "Jun 21", desktop: 169.0, mobile: 210.0 },
    ChartPoint { date: "Jun 22", desktop: 317.0, mobile: 270.0 },
    ChartPoint { date: "Jun 23", desktop: 480.0, mobile: 530.0 },
    ChartPoint { date: "Jun 24", desktop: 132.0, mobile: 180.0 },
    ChartPoint { date: "Jun 25", desktop: 141.0, mobile: 190.0 },
    ChartPoint { date: "Jun 26", desktop: 434.0, mobile: 380.0 },
    ChartPoint { date: "Jun 27", desktop: 448.0, mobile: 490.0 },
    ChartPoint { date: "Jun 28", desktop: 149.0, mobile: 200.0 },
    ChartPoint { date: "Jun 29", desktop: 103.0, mobile: 160.0 },
    ChartPoint { date: "Jun 30", desktop: 446.0, mobile: 400.0 },
];

// ---------------------------------------------------------------------------
// Table data
// ---------------------------------------------------------------------------

struct TableRow {
    header: &'static str,
    section_type: &'static str,
    status: &'static str,
    target: &'static str,
    limit: &'static str,
    reviewer: &'static str,
}

const TABLE_DATA: &[TableRow] = &[
    TableRow { header: "Cover page", section_type: "Cover page", status: "In Process", target: "18", limit: "5", reviewer: "Eddie Lake" },
    TableRow { header: "Table of contents", section_type: "Table of contents", status: "Done", target: "29", limit: "24", reviewer: "Eddie Lake" },
    TableRow { header: "Executive summary", section_type: "Narrative", status: "Done", target: "10", limit: "13", reviewer: "Eddie Lake" },
    TableRow { header: "Technical approach", section_type: "Narrative", status: "Done", target: "27", limit: "23", reviewer: "Jamik Tashpulatov" },
    TableRow { header: "Design", section_type: "Narrative", status: "In Process", target: "2", limit: "16", reviewer: "Jamik Tashpulatov" },
    TableRow { header: "Capabilities", section_type: "Narrative", status: "In Process", target: "20", limit: "8", reviewer: "Jamik Tashpulatov" },
    TableRow { header: "Integration with existing systems", section_type: "Narrative", status: "In Process", target: "19", limit: "21", reviewer: "Jamik Tashpulatov" },
    TableRow { header: "Innovation and Advantages", section_type: "Narrative", status: "Done", target: "25", limit: "26", reviewer: "Assign reviewer" },
    TableRow { header: "Overview of EMR's Innovative Solutions", section_type: "Technical content", status: "Done", target: "7", limit: "23", reviewer: "Assign reviewer" },
    TableRow { header: "Advanced Algorithms and Machine Learning", section_type: "Narrative", status: "Done", target: "30", limit: "28", reviewer: "Assign reviewer" },
    TableRow { header: "Adaptive Communication Protocols", section_type: "Narrative", status: "Done", target: "9", limit: "31", reviewer: "Assign reviewer" },
    TableRow { header: "Advantages Over Current Technologies", section_type: "Narrative", status: "Done", target: "12", limit: "0", reviewer: "Assign reviewer" },
    TableRow { header: "Past Performance", section_type: "Narrative", status: "Done", target: "22", limit: "33", reviewer: "Assign reviewer" },
    TableRow { header: "Customer Feedback and Satisfaction Levels", section_type: "Narrative", status: "Done", target: "15", limit: "34", reviewer: "Assign reviewer" },
    TableRow { header: "Implementation Challenges and Solutions", section_type: "Narrative", status: "Done", target: "3", limit: "35", reviewer: "Assign reviewer" },
    TableRow { header: "Security Measures and Data Protection", section_type: "Narrative", status: "In Process", target: "6", limit: "36", reviewer: "Assign reviewer" },
    TableRow { header: "Scalability and Future Proofing", section_type: "Narrative", status: "Done", target: "4", limit: "37", reviewer: "Assign reviewer" },
    TableRow { header: "Cost-Benefit Analysis", section_type: "Plain language", status: "Done", target: "14", limit: "38", reviewer: "Assign reviewer" },
    TableRow { header: "User Training and Onboarding Experience", section_type: "Narrative", status: "Done", target: "17", limit: "39", reviewer: "Assign reviewer" },
    TableRow { header: "Future Development Roadmap", section_type: "Narrative", status: "Done", target: "11", limit: "40", reviewer: "Assign reviewer" },
];

const ROWS_PER_PAGE: usize = 10;

// ---------------------------------------------------------------------------
// App state
// ---------------------------------------------------------------------------

struct DashboardApp {
    dark_mode: bool,
    sidebar_collapsed: bool,
    selected_nav: usize,
    chart_time_range: usize,
    table_tab: usize,
    table_page: usize,
}

impl Default for DashboardApp {
    fn default() -> Self {
        Self {
            dark_mode: false,
            sidebar_collapsed: false,
            selected_nav: 0,
            chart_time_range: 0,
            table_tab: 0,
            table_page: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Sidebar navigation items
// ---------------------------------------------------------------------------

struct NavItem {
    label: &'static str,
    icon: egui_shadcn::LucideIcon,
}

const NAV_MAIN: &[NavItem] = &[
    NavItem { label: "Dashboard", icon: egui_shadcn::LucideIcon::LayoutDashboard },
    NavItem { label: "Lifecycle", icon: egui_shadcn::LucideIcon::ListTree },
    NavItem { label: "Analytics", icon: egui_shadcn::LucideIcon::ChartBar },
    NavItem { label: "Projects", icon: egui_shadcn::LucideIcon::Folder },
    NavItem { label: "Team", icon: egui_shadcn::LucideIcon::Users },
];

const NAV_DOCS: &[NavItem] = &[
    NavItem { label: "Data Library", icon: egui_shadcn::LucideIcon::Database },
    NavItem { label: "Reports", icon: egui_shadcn::LucideIcon::FileText },
    NavItem { label: "Word Assistant", icon: egui_shadcn::LucideIcon::FileText },
];

const NAV_SECONDARY: &[NavItem] = &[
    NavItem { label: "Settings", icon: egui_shadcn::LucideIcon::Settings },
    NavItem { label: "Get Help", icon: egui_shadcn::LucideIcon::LifeBuoy },
    NavItem { label: "Search", icon: egui_shadcn::LucideIcon::Search },
];

// ---------------------------------------------------------------------------
// Stat card data
// ---------------------------------------------------------------------------

struct StatCard {
    label: &'static str,
    value: &'static str,
    badge_text: &'static str,
    trending_up: bool,
    footer_text: &'static str,
    footer_detail: &'static str,
}

const STAT_CARDS: &[StatCard] = &[
    StatCard {
        label: "Total Revenue",
        value: "$1,250.00",
        badge_text: "+12.5%",
        trending_up: true,
        footer_text: "Trending up this month",
        footer_detail: "Visitors for the last 6 months",
    },
    StatCard {
        label: "New Customers",
        value: "1,234",
        badge_text: "-20%",
        trending_up: false,
        footer_text: "Down 20% this period",
        footer_detail: "Acquisition needs attention",
    },
    StatCard {
        label: "Active Accounts",
        value: "45,678",
        badge_text: "+12.5%",
        trending_up: true,
        footer_text: "Strong user retention",
        footer_detail: "Engagement exceed targets",
    },
    StatCard {
        label: "Growth Rate",
        value: "4.5%",
        badge_text: "+4.5%",
        trending_up: true,
        footer_text: "Steady performance",
        footer_detail: "Meets growth projections",
    },
];

// ---------------------------------------------------------------------------
// eframe App impl
// ---------------------------------------------------------------------------

impl eframe::App for DashboardApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let theme = if self.dark_mode {
            egui_shadcn::theme::shadcn_theme_dark::dark()
        } else {
            egui_shadcn::theme::shadcn_theme_light::light()
        };
        egui_shadcn::ShadcnThemeExt::set_shadcn_theme(ui.ctx(), theme.clone());

        let sidebar_width = if self.sidebar_collapsed { 72.0 } else { 264.0 };

        egui::Panel::left("sidebar")
            .exact_size(sidebar_width)
            .frame(
                egui::Frame::NONE
                    .fill(theme.card)
                    .inner_margin(egui::Margin::symmetric(12, 12))
                    .stroke(egui::Stroke::new(1.0, theme.border)),
            )
            .show(ui, |ui| {
                self.draw_sidebar_content(ui, &theme);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme.background))
            .show(ui, |ui| {
                self.draw_header(ui, &theme);
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.draw_content(ui, &theme);
                    });
            });
    }
}

// ---------------------------------------------------------------------------
// Drawing methods
// ---------------------------------------------------------------------------

impl DashboardApp {
    fn draw_sidebar_content(&mut self, ui: &mut egui::Ui, theme: &egui_shadcn::ShadcnTheme) {
        ui.vertical(|ui| {
            // Logo
            if !self.sidebar_collapsed {
                ui.horizontal(|ui| {
                    let icon_size = 20.0;
                    let (icon_rect, _) = ui.allocate_exact_size(
                        egui::vec2(icon_size, icon_size),
                        egui::Sense::hover(),
                    );
                    if ui.is_rect_visible(icon_rect) {
                        egui_shadcn::icons::paint_icon::paint_icon(
                            ui.painter(),
                            icon_rect,
                            &egui_shadcn::LucideIcon::Hexagon,
                            theme.foreground,
                        );
                    }
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("Acme Inc.")
                            .size(15.0)
                            .strong()
                            .color(theme.foreground),
                    );
                });
            }

            ui.add_space(12.0);

            // Quick Create button
            if !self.sidebar_collapsed {
                ui.horizontal(|ui| {
                    egui_shadcn::Button::new("Quick Create")
                        .icon(egui_shadcn::LucideIcon::CirclePlus)
                        .show(ui);
                    egui_shadcn::Button::icon_only(egui_shadcn::LucideIcon::Mail)
                        .variant(egui_shadcn::ButtonVariant::Outline)
                        .show(ui);
                });
            }

            ui.add_space(8.0);

            // Main nav
            for (idx, item) in NAV_MAIN.iter().enumerate() {
                let resp = self.draw_nav_item(ui, theme, item, idx == self.selected_nav);
                if resp.clicked() {
                    self.selected_nav = idx;
                }
            }

            ui.add_space(16.0);

            // Documents section
            if !self.sidebar_collapsed {
                ui.label(
                    egui::RichText::new("Documents")
                        .size(11.0)
                        .color(theme.muted_foreground),
                );
                ui.add_space(4.0);
                for item in NAV_DOCS {
                    self.draw_nav_item(ui, theme, item, false);
                }

                // "More" item
                let (more_rect, more_resp) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 32.0),
                    egui::Sense::click(),
                );
                if ui.is_rect_visible(more_rect) {
                    let painter = ui.painter();
                    if more_resp.hovered() {
                        painter.rect_filled(
                            more_rect,
                            egui::CornerRadius::same(6),
                            theme.accent,
                        );
                    }
                    painter.text(
                        egui::pos2(more_rect.min.x + 32.0, more_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        "More",
                        egui::FontId::proportional(13.0),
                        theme.muted_foreground,
                    );
                }
            }

            // Spacer to push secondary nav to bottom
            let remaining = ui.available_height() - 140.0;
            if remaining > 0.0 {
                ui.add_space(remaining);
            }

            // Secondary nav
            if !self.sidebar_collapsed {
                for item in NAV_SECONDARY {
                    self.draw_nav_item(ui, theme, item, false);
                }

                // Dark mode toggle
                let (toggle_rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 32.0),
                    egui::Sense::hover(),
                );
                if ui.is_rect_visible(toggle_rect) {
                    let icon_size = 16.0;
                    let icon_rect = egui::Rect::from_min_size(
                        egui::pos2(
                            toggle_rect.min.x + 8.0,
                            toggle_rect.center().y - icon_size / 2.0,
                        ),
                        egui::vec2(icon_size, icon_size),
                    );
                    egui_shadcn::icons::paint_icon::paint_icon(
                        ui.painter(),
                        icon_rect,
                        &egui_shadcn::LucideIcon::Sun,
                        theme.muted_foreground,
                    );
                    ui.painter().text(
                        egui::pos2(toggle_rect.min.x + 32.0, toggle_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        "Dark Mode",
                        egui::FontId::proportional(13.0),
                        theme.foreground,
                    );
                }
                // Place a switch on top of that row
                let switch_rect = egui::Rect::from_min_size(
                    egui::pos2(toggle_rect.max.x - 48.0, toggle_rect.center().y - 10.0),
                    egui::vec2(40.0, 20.0),
                );
                let mut child_ui =
                    ui.new_child(egui::UiBuilder::new().max_rect(switch_rect));
                child_ui.add(egui_shadcn::Switch::new(&mut self.dark_mode));

                ui.add_space(8.0);
                ui.add(egui_shadcn::Separator::horizontal());
                ui.add_space(8.0);

                // User
                ui.horizontal(|ui| {
                    ui.add(egui_shadcn::Avatar::new("SC").size(32.0));
                    ui.add_space(4.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("shadcn")
                                .size(13.0)
                                .strong()
                                .color(theme.foreground),
                        );
                        ui.label(
                            egui::RichText::new("m@example.com")
                                .size(11.0)
                                .color(theme.muted_foreground),
                        );
                    });
                });
            }
        });
    }

    fn draw_nav_item(
        &self,
        ui: &mut egui::Ui,
        theme: &egui_shadcn::ShadcnTheme,
        item: &NavItem,
        active: bool,
    ) -> egui::Response {
        let width = ui.available_width();
        let height = 32.0;
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(width, height),
            egui::Sense::click(),
        );

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Background
            let bg = if active {
                theme.accent
            } else if response.hovered() {
                egui::Color32::from_rgba_unmultiplied(
                    theme.accent.r(),
                    theme.accent.g(),
                    theme.accent.b(),
                    128,
                )
            } else {
                egui::Color32::TRANSPARENT
            };
            painter.rect_filled(rect, egui::CornerRadius::same(6), bg);

            // Icon
            let icon_size = 16.0;
            let icon_rect = egui::Rect::from_min_size(
                egui::pos2(rect.min.x + 8.0, rect.center().y - icon_size / 2.0),
                egui::vec2(icon_size, icon_size),
            );
            egui_shadcn::icons::paint_icon::paint_icon(
                painter,
                icon_rect,
                &item.icon,
                if active { theme.accent_foreground } else { theme.muted_foreground },
            );

            // Label
            if !self.sidebar_collapsed {
                let fg = if active { theme.accent_foreground } else { theme.foreground };
                painter.text(
                    egui::pos2(rect.min.x + 32.0, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    item.label,
                    egui::FontId::proportional(13.0),
                    fg,
                );
            }
        }

        response
    }

    fn draw_header(&mut self, ui: &mut egui::Ui, theme: &egui_shadcn::ShadcnTheme) {
        let frame = egui::Frame::NONE
            .fill(theme.background)
            .inner_margin(egui::Margin::symmetric(16, 0));

        let header_resp = frame.show(ui, |ui| {
            ui.set_min_height(48.0);

            egui_shadcn::Flex::row()
                .w_full()
                .align_center()
                .show(ui, |f| {
                    // Sidebar toggle
                    f.ui(|ui| {
                        let icon = if self.sidebar_collapsed {
                            egui_shadcn::LucideIcon::PanelLeftOpen
                        } else {
                            egui_shadcn::LucideIcon::PanelLeftClose
                        };
                        if egui_shadcn::Button::icon_only(icon)
                            .variant(egui_shadcn::ButtonVariant::Ghost)
                            .size(egui_shadcn::ComponentSize::Sm)
                            .show(ui)
                            .clicked()
                        {
                            self.sidebar_collapsed = !self.sidebar_collapsed;
                        }
                    });

                    // Separator
                    f.ui(|ui| {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(1.0, 16.0),
                            egui::Sense::hover(),
                        );
                        if ui.is_rect_visible(rect) {
                            ui.painter().vline(
                                rect.center().x,
                                rect.y_range(),
                                egui::Stroke::new(1.0, theme.border),
                            );
                        }
                    });

                    // Title
                    f.ui(|ui| {
                        ui.label(
                            egui::RichText::new("Documents")
                                .size(15.0)
                                .strong()
                                .color(theme.foreground),
                        );
                    });

                    f.spacer();

                    // Dark mode toggle (right side)
                    f.ui(|ui| {
                        let icon = if self.dark_mode {
                            egui_shadcn::LucideIcon::Moon
                        } else {
                            egui_shadcn::LucideIcon::Sun
                        };
                        if egui_shadcn::Button::icon_only(icon)
                            .variant(egui_shadcn::ButtonVariant::Ghost)
                            .size(egui_shadcn::ComponentSize::Sm)
                            .show(ui)
                            .clicked()
                        {
                            self.dark_mode = !self.dark_mode;
                        }
                    });
                });
        });

        // Bottom border only
        let rect = header_resp.response.rect;
        ui.painter().hline(
            rect.x_range(),
            rect.max.y,
            egui::Stroke::new(1.0, theme.border),
        );
    }

    fn draw_content(&mut self, ui: &mut egui::Ui, theme: &egui_shadcn::ShadcnTheme) {
        ui.add_space(16.0);

        // Stat cards
        self.draw_section_cards(ui, theme);

        ui.add_space(16.0);

        // Chart area
        let margin = egui::Margin::symmetric(16, 0);
        egui::Frame::NONE.inner_margin(margin).show(ui, |ui| {
            self.draw_chart_card(ui, theme);
        });

        ui.add_space(16.0);

        // Data table
        self.draw_data_table_section(ui, theme);

        ui.add_space(24.0);
    }

    fn draw_section_cards(&self, ui: &mut egui::Ui, theme: &egui_shadcn::ShadcnTheme) {
        let margin = egui::Margin::symmetric(16, 0);
        egui::Frame::NONE.inner_margin(margin).show(ui, |ui| {
            ui.columns(STAT_CARDS.len(), |cols| {
                for (i, col) in cols.iter_mut().enumerate() {
                    draw_stat_card(col, theme, &STAT_CARDS[i]);
                }
            });
        });
    }

    fn draw_chart_card(&mut self, ui: &mut egui::Ui, theme: &egui_shadcn::ShadcnTheme) {
        egui_shadcn::Card::new().show(ui, |ui| {
            // Header
            egui_shadcn::Flex::row()
                .w_full()
                .align_center()
                .show(ui, |f| {
                    f.ui(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Total Visitors")
                                    .size(16.0)
                                    .strong()
                                    .color(theme.foreground),
                            );
                            ui.label(
                                egui::RichText::new("Total for the last 3 months")
                                    .size(13.0)
                                    .color(theme.muted_foreground),
                            );
                        });
                    });

                    f.spacer();

                    f.ui(|ui| {
                        egui_shadcn::ToggleGroup::new(vec![
                            "Last 3 months".to_owned(),
                            "Last 30 days".to_owned(),
                            "Last 7 days".to_owned(),
                        ])
                        .show(ui, &mut self.chart_time_range);
                    });
                });

            ui.add_space(16.0);

            // Chart
            let data = match self.chart_time_range {
                1 => &CHART_DATA[CHART_DATA.len().saturating_sub(15)..],
                2 => &CHART_DATA[CHART_DATA.len().saturating_sub(7)..],
                _ => CHART_DATA,
            };

            let labels: Vec<String> = data.iter().map(|p| p.date.to_owned()).collect();
            let mobile_vals: Vec<f32> = data.iter().map(|p| p.mobile).collect();
            let desktop_vals: Vec<f32> = data.iter().map(|p| p.desktop).collect();

            egui_shadcn::AreaChart::new(labels)
                .series(egui_shadcn::AreaSeries { values: mobile_vals, color: theme.primary })
                .series(egui_shadcn::AreaSeries { values: desktop_vals, color: theme.primary })
                .stacked()
                .height(250.0)
                .show(ui);
        });
    }

    fn draw_data_table_section(&mut self, ui: &mut egui::Ui, theme: &egui_shadcn::ShadcnTheme) {
        let margin = egui::Margin::symmetric(16, 0);
        egui::Frame::NONE.inner_margin(margin).show(ui, |ui| {
            // Tabs bar + action buttons
            egui_shadcn::Flex::row()
                .w_full()
                .align_center()
                .show(ui, |f| {
                    f.ui(|ui| {
                        egui_shadcn::Tabs::new(vec![
                            "Outline".to_owned(),
                            "Past Performance".to_owned(),
                            "Key Personnel".to_owned(),
                            "Focus Documents".to_owned(),
                        ])
                        .show(ui, &mut self.table_tab, |_, _| {});
                    });

                    f.spacer();

                    f.ui(|ui| {
                        ui.horizontal(|ui| {
                            egui_shadcn::Button::new("Customize Columns")
                                .variant(egui_shadcn::ButtonVariant::Outline)
                                .size(egui_shadcn::ComponentSize::Sm)
                                .icon(egui_shadcn::LucideIcon::Columns3)
                                .show(ui);
                            egui_shadcn::Button::new("Add Section")
                                .variant(egui_shadcn::ButtonVariant::Outline)
                                .size(egui_shadcn::ComponentSize::Sm)
                                .icon(egui_shadcn::LucideIcon::Plus)
                                .show(ui);
                        });
                    });
                });

            ui.add_space(8.0);

            // Table content based on active tab
            if self.table_tab == 0 {
                self.draw_outline_table(ui, theme);
            } else {
                // Placeholder for other tabs
                let frame = egui::Frame::NONE
                    .stroke(egui::Stroke::new(
                        1.0,
                        egui::Stroke::new(1.0, theme.border).color,
                    ))
                    .corner_radius(egui::CornerRadius::same(8))
                    .inner_margin(egui::Margin::same(32));
                frame.show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.set_min_height(120.0);
                    egui_shadcn::center(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Content coming soon")
                                .color(theme.muted_foreground),
                        );
                    });
                });
            }
        });
    }

    fn draw_outline_table(&mut self, ui: &mut egui::Ui, theme: &egui_shadcn::ShadcnTheme) {
        let total_pages = (TABLE_DATA.len() + ROWS_PER_PAGE - 1) / ROWS_PER_PAGE;
        self.table_page = self.table_page.min(total_pages.saturating_sub(1));

        let start = self.table_page * ROWS_PER_PAGE;
        let end = (start + ROWS_PER_PAGE).min(TABLE_DATA.len());
        let page_data = &TABLE_DATA[start..end];

        let headers = vec![
            "Header".to_owned(),
            "Section Type".to_owned(),
            "Status".to_owned(),
            "Target".to_owned(),
            "Limit".to_owned(),
            "Reviewer".to_owned(),
        ];

        let rows: Vec<Vec<String>> = page_data
            .iter()
            .map(|r| {
                vec![
                    r.header.to_owned(),
                    r.section_type.to_owned(),
                    r.status.to_owned(),
                    r.target.to_owned(),
                    r.limit.to_owned(),
                    r.reviewer.to_owned(),
                ]
            })
            .collect();

        egui_shadcn::Table::new(headers)
            .rows(rows)
            .col_weights(vec![3.0, 2.0, 1.5, 1.0, 1.0, 2.0])
            .show(ui);

        ui.add_space(8.0);

        // Pagination footer
        egui_shadcn::Flex::row()
            .w_full()
            .align_center()
            .show(ui, |f| {
                f.ui(|ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "Showing {}-{} of {} rows",
                            start + 1,
                            end,
                            TABLE_DATA.len()
                        ))
                        .size(13.0)
                        .color(theme.muted_foreground),
                    );
                });

                f.spacer();

                f.ui(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!(
                                "Page {} of {}",
                                self.table_page + 1,
                                total_pages
                            ))
                            .size(13.0)
                            .color(theme.foreground),
                        );
                        ui.add_space(8.0);

                        if egui_shadcn::Button::icon_only(
                            egui_shadcn::LucideIcon::ChevronsLeft,
                        )
                        .variant(egui_shadcn::ButtonVariant::Outline)
                        .size(egui_shadcn::ComponentSize::Sm)
                        .enabled(self.table_page > 0)
                        .show(ui)
                        .clicked()
                        {
                            self.table_page = 0;
                        }

                        if egui_shadcn::Button::icon_only(
                            egui_shadcn::LucideIcon::ChevronLeft,
                        )
                        .variant(egui_shadcn::ButtonVariant::Outline)
                        .size(egui_shadcn::ComponentSize::Sm)
                        .enabled(self.table_page > 0)
                        .show(ui)
                        .clicked()
                        {
                            self.table_page = self.table_page.saturating_sub(1);
                        }

                        if egui_shadcn::Button::icon_only(
                            egui_shadcn::LucideIcon::ChevronRight,
                        )
                        .variant(egui_shadcn::ButtonVariant::Outline)
                        .size(egui_shadcn::ComponentSize::Sm)
                        .enabled(self.table_page + 1 < total_pages)
                        .show(ui)
                        .clicked()
                        {
                            self.table_page += 1;
                        }

                        if egui_shadcn::Button::icon_only(
                            egui_shadcn::LucideIcon::ChevronsRight,
                        )
                        .variant(egui_shadcn::ButtonVariant::Outline)
                        .size(egui_shadcn::ComponentSize::Sm)
                        .enabled(self.table_page + 1 < total_pages)
                        .show(ui)
                        .clicked()
                        {
                            self.table_page = total_pages.saturating_sub(1);
                        }
                    });
                });
            });
    }
}

// ---------------------------------------------------------------------------
// Standalone drawing functions
// ---------------------------------------------------------------------------

fn draw_stat_card(
    ui: &mut egui::Ui,
    theme: &egui_shadcn::ShadcnTheme,
    stat: &StatCard,
) {
    egui_shadcn::Card::new().show(ui, |ui| {
        // Header row: description + badge
        egui_shadcn::Flex::row()
            .w_full()
            .align_center()
            .show(ui, |f| {
                f.ui(|ui| {
                    ui.label(
                        egui::RichText::new(stat.label)
                            .size(13.0)
                            .color(theme.muted_foreground),
                    );
                });
                f.spacer();
                f.ui(|ui| {
                    let icon = if stat.trending_up {
                        egui_shadcn::LucideIcon::TrendingUp
                    } else {
                        egui_shadcn::LucideIcon::TrendingDown
                    };
                    ui.horizontal(|ui| {
                        let icon_size = 12.0;
                        let (icon_rect, _) = ui.allocate_exact_size(
                            egui::vec2(icon_size, icon_size),
                            egui::Sense::hover(),
                        );
                        if ui.is_rect_visible(icon_rect) {
                            egui_shadcn::icons::paint_icon::paint_icon(
                                ui.painter(),
                                icon_rect,
                                &icon,
                                theme.muted_foreground,
                            );
                        }
                        ui.add(
                            egui_shadcn::Badge::new(stat.badge_text)
                                .variant(egui_shadcn::BadgeVariant::Outline),
                        );
                    });
                });
            });

        ui.add_space(4.0);

        // Value
        ui.label(
            egui::RichText::new(stat.value)
                .size(28.0)
                .strong()
                .color(theme.foreground),
        );

        ui.add_space(8.0);

        // Footer
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(stat.footer_text)
                    .size(13.0)
                    .strong()
                    .color(theme.foreground),
            );
            let icon = if stat.trending_up {
                egui_shadcn::LucideIcon::TrendingUp
            } else {
                egui_shadcn::LucideIcon::TrendingDown
            };
            let icon_size = 14.0;
            let (icon_rect, _) = ui.allocate_exact_size(
                egui::vec2(icon_size, icon_size),
                egui::Sense::hover(),
            );
            if ui.is_rect_visible(icon_rect) {
                egui_shadcn::icons::paint_icon::paint_icon(
                    ui.painter(),
                    icon_rect,
                    &icon,
                    theme.foreground,
                );
            }
        });

        ui.label(
            egui::RichText::new(stat.footer_detail)
                .size(12.0)
                .color(theme.muted_foreground),
        );
    });
}


// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "shadcn Dashboard — egui-shadcn",
        options,
        Box::new(|cc| {
            egui_shadcn::setup_fonts(&cc.egui_ctx);
            Ok(Box::new(DashboardApp::default()))
        }),
    )
}
