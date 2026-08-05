// Component dashboard — comprehensive demo of every egui-shadcn widget.

// ---------------------------------------------------------------------------
// Component metadata
// ---------------------------------------------------------------------------

const CATEGORIES: &[(&str, &[&str])] = &[
    ("Layout", &["Flex", "Toolbar", "StatusBar"]),
    (
        "Typography & Display",
        &[
            "Typography",
            "Icons",
            "Badge",
            "ColorSwatch",
            "Kbd",
            "Separator",
            "Avatar",
            "Skeleton",
            "Spinner",
            "Progress",
            "Empty",
        ],
    ),
    (
        "Inputs",
        &[
            "Button",
            "Checkbox",
            "Switch",
            "Radio",
            "RadioGroup",
            "Toggle",
            "ToggleGroup",
            "Slider",
            "Input",
            "NumberInput",
            "InputGroup",
            "Textarea",
            "Select",
            "Combobox",
            "InputOtp",
        ],
    ),
    (
        "Containers",
        &[
            "Card",
            "PropertyGrid",
            "Alert",
            "Accordion",
            "Collapsible",
            "Tabs",
            "IconTabs",
            "Table",
            "Item",
            "AspectRatio",
            "ScrollArea",
            "Resizable",
        ],
    ),
    (
        "Navigation",
        &[
            "Breadcrumb",
            "Pagination",
            "NavigationMenu",
            "Menubar",
            "Sidebar",
        ],
    ),
    (
        "Overlays",
        &[
            "Dialog",
            "AlertDialog",
            "Sheet",
            "Drawer",
            "Popover",
            "HoverCard",
            "Tooltip",
            "ContextMenu",
            "DropdownMenu",
            "Command",
        ],
    ),
    (
        "Feedback",
        &["Toast", "Carousel", "Calendar", "DatePicker"],
    ),
    (
        "Data Visualization",
        &["AreaChart"],
    ),
    (
        "Form Layout",
        &["FieldGroup"],
    ),
];

fn flat_index(cat: usize, item: usize) -> usize {
    CATEGORIES
        .iter()
        .take(cat)
        .map(|(_, items)| items.len())
        .sum::<usize>()
        + item
}

fn component_name(flat: usize) -> &'static str {
    let mut remaining = flat;
    for (_, items) in CATEGORIES {
        if remaining < items.len() {
            return items[remaining];
        }
        remaining -= items.len();
    }
    ""
}

#[cfg(target_arch = "wasm32")]
fn component_index(name: &str) -> Option<usize> {
    let needle = name.trim().to_ascii_lowercase();
    CATEGORIES
        .iter()
        .flat_map(|(_, items)| items.iter())
        .position(|item| item.to_ascii_lowercase() == needle)
}

#[cfg(target_arch = "wasm32")]
fn initial_selected_component() -> usize {
    selected_component_from_query().unwrap_or(0)
}

#[cfg(not(target_arch = "wasm32"))]
fn initial_selected_component() -> usize {
    0
}

#[cfg(target_arch = "wasm32")]
fn selected_component_from_query() -> Option<usize> {
    let search = web_sys::window()?.location().search().ok()?;
    let query = search.strip_prefix('?').unwrap_or(&search);

    query.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        if key == "component" {
            component_index(value)
        } else {
            None
        }
    })
}

// ---------------------------------------------------------------------------
// App state
// ---------------------------------------------------------------------------

struct DashboardApp {
    dark_mode: bool,
    selected: usize,

    // Per-component demo state
    checkbox_val: bool,
    switch_val: bool,
    radio_a: bool,
    radio_b: bool,
    radio_c: bool,
    radio_group_val: String,
    toggle_bold: bool,
    toggle_italic: bool,
    toggle_group_idx: usize,
    slider_val: f64,
    input_text: String,
    textarea_text: String,
    select_val: Option<String>,
    combobox_selected: Option<usize>,
    combobox_search: String,
    otp_value: String,
    accordion_open: Vec<usize>,
    collapsible_open: bool,
    tabs_idx: usize,
    pagination_page: usize,
    nav_menu_idx: usize,
    sidebar_collapsed: bool,
    resizable_fraction: f32,
    dialog_open: bool,
    alert_dialog_open: bool,
    sheet_open: bool,
    drawer_open: bool,
    command_open: bool,
    command_search: String,
    toast_state: egui_shadcn::ToastState,
    carousel_idx: usize,
    calendar_year: i32,
    calendar_month: u32,
    calendar_day: u32,
    date_picker_state: egui_shadcn::DatePickerState,
    progress_val: f32,
    number_f64: f64,
    number_f32: f32,
    number_i32: i32,
    input_group_text: String,
    icon_tabs_idx: usize,
    button_selected: bool,
    icon_search: String,
    flex_input_text: String,
    flex_first_name: String,
    flex_last_name: String,
    flex_email: String,
    flex_phone: String,
    toolbar_tool_idx: usize,
    toolbar_snap: bool,
    color_swatch_idx: usize,
    prop_x: f64,
    prop_y: f64,
    prop_width: f64,
    prop_height: f64,
    prop_rotation: f64,
    prop_opacity: f64,
    prop_blend_mode: String,
}

impl Default for DashboardApp {
    fn default() -> Self {
        Self {
            dark_mode: true,
            selected: initial_selected_component(),
            checkbox_val: false,
            switch_val: false,
            radio_a: true,
            radio_b: false,
            radio_c: false,
            radio_group_val: "Option A".to_owned(),
            toggle_bold: false,
            toggle_italic: false,
            toggle_group_idx: 0,
            slider_val: 50.0,
            input_text: String::new(),
            textarea_text: String::new(),
            select_val: None,
            combobox_selected: None,
            combobox_search: String::new(),
            otp_value: String::new(),
            accordion_open: vec![0],
            collapsible_open: true,
            tabs_idx: 0,
            pagination_page: 0,
            nav_menu_idx: 0,
            sidebar_collapsed: false,
            resizable_fraction: 0.5,
            dialog_open: false,
            alert_dialog_open: false,
            sheet_open: false,
            drawer_open: false,
            command_open: false,
            command_search: String::new(),
            toast_state: egui_shadcn::ToastState::new(),
            carousel_idx: 0,
            calendar_year: 2026,
            calendar_month: 2,
            calendar_day: 14,
            date_picker_state: egui_shadcn::DatePickerState::default(),
            progress_val: 0.66,
            number_f64: 42.0,
            number_f32: 3.14,
            number_i32: 10,
            input_group_text: String::new(),
            icon_tabs_idx: 0,
            button_selected: false,
            icon_search: String::new(),
            flex_input_text: String::new(),
            flex_first_name: String::new(),
            flex_last_name: String::new(),
            flex_email: String::new(),
            flex_phone: String::new(),
            toolbar_tool_idx: 1,
            toolbar_snap: true,
            color_swatch_idx: 0,
            prop_x: 124.0,
            prop_y: 88.0,
            prop_width: 320.0,
            prop_height: 180.0,
            prop_rotation: -8.0,
            prop_opacity: 92.0,
            prop_blend_mode: "Normal".to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// eframe::App
// ---------------------------------------------------------------------------

impl eframe::App for DashboardApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Panels take the `Ui` on egui 0.35; the overlay widgets below are
        // still `Context`-driven, so keep a handle to both.
        let ctx = ui.ctx().clone();

        let theme = if self.dark_mode {
            egui_shadcn::theme::shadcn_theme_dark::dark()
        } else {
            egui_shadcn::theme::shadcn_theme_light::light()
        };
        egui_shadcn::ShadcnThemeExt::set_shadcn_theme(&ctx, theme.clone());

        // Top bar
        egui::Panel::top("top_bar")
            .frame(
                egui::Frame::NONE
                    .fill(theme.card)
                    .inner_margin(egui::Margin {
                        left: 16,
                        right: 16,
                        top: 8,
                        bottom: 8,
                    })
                    .stroke(egui::Stroke::new(1.0, theme.border)),
            )
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    egui_shadcn::Typography::h4("egui-shadcn").show(ui);
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.add(egui_shadcn::Switch::new(&mut self.dark_mode));
                            ui.label(
                                egui::RichText::new("Dark")
                                    .color(theme.muted_foreground)
                                    .size(13.0),
                            );
                        },
                    );
                });
            });

        // Left sidebar
        egui::Panel::left("sidebar_panel")
            .default_size(200.0)
            .frame(
                egui::Frame::NONE
                    .fill(theme.card)
                    .inner_margin(egui::Margin {
                        left: 12,
                        right: 12,
                        top: 12,
                        bottom: 12,
                    })
                    .stroke(egui::Stroke::new(1.0, theme.border)),
            )
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (cat_idx, (cat_name, items)) in CATEGORIES.iter().enumerate() {
                        ui.label(
                            egui::RichText::new(*cat_name)
                                .color(theme.muted_foreground)
                                .size(11.0)
                                .strong(),
                        );
                        ui.add_space(4.0);

                        for (item_idx, item_name) in items.iter().enumerate() {
                            let idx = flat_index(cat_idx, item_idx);
                            let is_selected = idx == self.selected;

                            let fg = if is_selected {
                                theme.primary_foreground
                            } else {
                                theme.foreground
                            };

                            let galley = ui.painter().layout_no_wrap(
                                item_name.to_string(),
                                egui::FontId::proportional(13.0),
                                fg,
                            );

                            let desired =
                                egui::vec2(ui.available_width(), galley.size().y + 8.0);
                            let (rect, response) =
                                ui.allocate_exact_size(desired, egui::Sense::click());

                            if ui.is_rect_visible(rect) {
                                let cr = egui::CornerRadius::same(
                                    theme.radius.round() as u8,
                                );
                                if is_selected {
                                    ui.painter().rect_filled(rect, cr, theme.primary);
                                } else if response.hovered() {
                                    ui.painter().rect_filled(rect, cr, theme.accent);
                                }
                                let pos = egui::pos2(
                                    rect.min.x + 8.0,
                                    rect.center().y - galley.size().y / 2.0,
                                );
                                ui.painter().galley(pos, galley, fg);
                            }

                            if response.clicked() {
                                self.selected = idx;
                                ui.ctx().request_repaint();
                            }
                        }

                        ui.add_space(12.0);
                    }
                });
            });

        // Content area
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme.background))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.add_space(24.0);
                        ui.horizontal(|ui| {
                            ui.add_space(24.0);
                            ui.vertical(|ui| {
                                ui.set_max_width(700.0);
                                self.render_component(ui, &theme);
                            });
                        });
                        ui.add_space(24.0);
                    });
            });

        // Overlay widgets that need ctx
        if self.dialog_open {
            let mut close = false;
            egui_shadcn::Dialog::new()
                .title("Edit Profile")
                .description("Make changes to your profile here.")
                .show(&ctx, &mut self.dialog_open, |ui| {
                    ui.label("Dialog content goes here.");
                    if egui_shadcn::Button::new("Save Changes").show(ui).clicked() {
                        close = true;
                    }
                });
            if close {
                self.dialog_open = false;
            }
        }

        if self.alert_dialog_open {
            egui_shadcn::AlertDialog::new(
                "Are you absolutely sure?",
                "This action cannot be undone. This will permanently delete your account.",
            )
            .destructive()
            .show(&ctx, &mut self.alert_dialog_open);
        }

        if self.sheet_open {
            egui_shadcn::Sheet::new()
                .title("Sheet Panel")
                .description("This is a side sheet overlay.")
                .side(egui_shadcn::SheetSide::Right)
                .show(&ctx, &mut self.sheet_open, |ui| {
                    ui.label("Sheet content here.");
                });
        }

        if self.drawer_open {
            egui_shadcn::Drawer::new()
                .title("Drawer")
                .description("A bottom drawer panel.")
                .show(&ctx, &mut self.drawer_open, |ui| {
                    ui.label("Drawer content here.");
                });
        }

        if self.command_open {
            egui_shadcn::Command::new(vec![
                ("Suggestions".to_owned(), "Calendar".to_owned()),
                ("Suggestions".to_owned(), "Search Emoji".to_owned()),
                ("Suggestions".to_owned(), "Calculator".to_owned()),
                ("Settings".to_owned(), "Profile".to_owned()),
                ("Settings".to_owned(), "Billing".to_owned()),
                ("Settings".to_owned(), "Appearance".to_owned()),
            ])
            .placeholder("Type a command or search...")
            .show(&ctx, &mut self.command_open, &mut self.command_search);
        }

        self.toast_state.show(&ctx);
    }
}

// ---------------------------------------------------------------------------
// Component sections
// ---------------------------------------------------------------------------

impl DashboardApp {
    fn render_component(
        &mut self,
        ui: &mut egui::Ui,
        theme: &egui_shadcn::ShadcnTheme,
    ) {
        let name = component_name(self.selected);
        egui_shadcn::Typography::h3(name).show(ui);
        ui.add_space(4.0);

        match name {
            "Flex" => self.demo_flex(ui),
            "Toolbar" => self.demo_toolbar(ui),
            "StatusBar" => self.demo_status_bar(ui),
            "Typography" => self.demo_typography(ui),
            "Icons" => self.demo_icons(ui, theme),
            "Badge" => self.demo_badge(ui),
            "ColorSwatch" => self.demo_color_swatch(ui),
            "Kbd" => self.demo_kbd(ui),
            "Separator" => self.demo_separator(ui),
            "Avatar" => self.demo_avatar(ui),
            "Skeleton" => self.demo_skeleton(ui),
            "Spinner" => self.demo_spinner(ui),
            "Progress" => self.demo_progress(ui),
            "Empty" => self.demo_empty(ui),
            "Button" => self.demo_button(ui),
            "Checkbox" => self.demo_checkbox(ui),
            "Switch" => self.demo_switch(ui),
            "Radio" => self.demo_radio(ui),
            "RadioGroup" => self.demo_radio_group(ui),
            "Toggle" => self.demo_toggle(ui),
            "ToggleGroup" => self.demo_toggle_group(ui),
            "Slider" => self.demo_slider(ui),
            "Input" => self.demo_input(ui),
            "NumberInput" => self.demo_number_input(ui),
            "InputGroup" => self.demo_input_group(ui),
            "Textarea" => self.demo_textarea(ui),
            "Select" => self.demo_select(ui),
            "Combobox" => self.demo_combobox(ui),
            "InputOtp" => self.demo_input_otp(ui),
            "Card" => self.demo_card(ui),
            "PropertyGrid" => self.demo_property_grid(ui),
            "Alert" => self.demo_alert(ui),
            "Accordion" => self.demo_accordion(ui),
            "Collapsible" => self.demo_collapsible(ui),
            "Tabs" => self.demo_tabs(ui),
            "IconTabs" => self.demo_icon_tabs(ui),
            "Table" => self.demo_table(ui),
            "Item" => self.demo_item(ui),
            "AspectRatio" => self.demo_aspect_ratio(ui, theme),
            "ScrollArea" => self.demo_scroll_area(ui),
            "Resizable" => self.demo_resizable(ui),
            "Breadcrumb" => self.demo_breadcrumb(ui),
            "Pagination" => self.demo_pagination(ui),
            "NavigationMenu" => self.demo_navigation_menu(ui),
            "Menubar" => self.demo_menubar(ui),
            "Sidebar" => self.demo_sidebar(ui),
            "Dialog" => self.demo_dialog(ui),
            "AlertDialog" => self.demo_alert_dialog(ui),
            "Sheet" => self.demo_sheet(ui),
            "Drawer" => self.demo_drawer(ui),
            "Popover" => self.demo_popover(ui),
            "HoverCard" => self.demo_hover_card(ui),
            "Tooltip" => self.demo_tooltip(ui),
            "ContextMenu" => self.demo_context_menu(ui),
            "DropdownMenu" => self.demo_dropdown_menu(ui),
            "Command" => self.demo_command(ui),
            "Toast" => self.demo_toast(ui),
            "Carousel" => self.demo_carousel(ui),
            "Calendar" => self.demo_calendar(ui),
            "DatePicker" => self.demo_date_picker(ui),
            "AreaChart" => self.demo_area_chart(ui),
            "FieldGroup" => self.demo_field_group(ui),
            _ => {
                ui.label("No demo available.");
            }
        }
    }

    // ── Layout ────────────────────────────────────────────────────────

    fn demo_flex(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Flexbox-like layout with gap, grow, justify, and wrap.")
            .show(ui);
        ui.add_space(12.0);

        // Row with gap
        egui_shadcn::Typography::small("Row with gap").show(ui);
        ui.add_space(4.0);
        egui_shadcn::Flex::row().gap(8.0).show(ui, |f| {
            f.add(egui_shadcn::Button::new("Cancel")
                .variant(egui_shadcn::ButtonVariant::Outline));
            f.add(egui_shadcn::Button::new("Save"));
        });

        ui.add_space(16.0);

        // Column with gap
        egui_shadcn::Typography::small("Column with gap").show(ui);
        ui.add_space(4.0);
        egui_shadcn::Flex::column().gap(8.0).show(ui, |f| {
            f.add(egui_shadcn::Badge::new("First"));
            f.add(egui_shadcn::Badge::new("Second"));
            f.add(egui_shadcn::Badge::new("Third"));
        });

        ui.add_space(16.0);

        // Grow — input fills, button stays natural
        egui_shadcn::Typography::small("Grow (input fills, button stays natural)").show(ui);
        ui.add_space(4.0);
        egui_shadcn::Flex::row().gap(8.0).w_full().show(ui, |f| {
            f.grow(1.0, egui_shadcn::Input::new(&mut self.flex_input_text)
                .placeholder("Type a message..."));
            f.add(egui_shadcn::Button::new("Send"));
        });

        ui.add_space(16.0);

        // Justify end — buttons pushed to the right
        egui_shadcn::Typography::small("Justify end").show(ui);
        ui.add_space(4.0);
        egui_shadcn::Flex::row().justify_end().gap(8.0).w_full().show(ui, |f| {
            f.add(egui_shadcn::Button::new("Cancel")
                .variant(egui_shadcn::ButtonVariant::Outline));
            f.add(egui_shadcn::Button::new("Confirm"));
        });

        ui.add_space(16.0);

        // Justify between — spacer pushes items apart
        egui_shadcn::Typography::small("Justify between").show(ui);
        ui.add_space(4.0);
        egui_shadcn::Flex::row().justify_between().w_full().show(ui, |f| {
            f.add(egui_shadcn::Button::new("Previous")
                .variant(egui_shadcn::ButtonVariant::Outline));
            f.add(egui_shadcn::Button::new("Next"));
        });

        ui.add_space(16.0);

        // Justify center
        egui_shadcn::Typography::small("Justify center").show(ui);
        ui.add_space(4.0);
        egui_shadcn::Flex::row().justify_center().gap(8.0).w_full().show(ui, |f| {
            f.add(egui_shadcn::Spinner::new().size(20.0));
            f.ui(|ui| { ui.label("Loading..."); });
        });

        ui.add_space(16.0);

        // Wrapping tags
        egui_shadcn::Typography::small("Wrap (overflowing items wrap to next line)").show(ui);
        ui.add_space(4.0);
        let tags = [
            "Rust", "egui", "shadcn", "flexbox", "layout", "widgets",
            "responsive", "wrap", "gap", "grow", "animation", "theming",
            "buttons", "inputs", "cards", "dialogs", "toasts", "badges",
        ];
        egui_shadcn::Flex::row().gap(4.0).wrap().w_full().show(ui, |f| {
            for tag in &tags {
                f.add(egui_shadcn::Badge::new(*tag));
            }
        });

        ui.add_space(16.0);

        // Spacer — push items apart
        egui_shadcn::Typography::small("Spacer (pushes items apart)").show(ui);
        ui.add_space(4.0);
        egui_shadcn::Flex::row().gap(8.0).w_full().show(ui, |f| {
            f.add(egui_shadcn::Badge::new("Left"));
            f.spacer();
            f.add(egui_shadcn::Badge::new("Right"));
        });

        ui.add_space(16.0);

        // Nested flex — two-column form
        egui_shadcn::Typography::small("Nested flex (two-column form)").show(ui);
        ui.add_space(4.0);
        egui_shadcn::Flex::row().gap(16.0).w_full().show(ui, |f| {
            f.grow_nested(1.0, egui_shadcn::Flex::column().gap(8.0), |f| {
                f.ui(|ui| { egui_shadcn::Label::new("First Name").show(ui); });
                f.add(egui_shadcn::Input::new(&mut self.flex_first_name)
                    .placeholder("John"));
                f.ui(|ui| { egui_shadcn::Label::new("Last Name").show(ui); });
                f.add(egui_shadcn::Input::new(&mut self.flex_last_name)
                    .placeholder("Doe"));
            });
            f.grow_nested(1.0, egui_shadcn::Flex::column().gap(8.0), |f| {
                f.ui(|ui| { egui_shadcn::Label::new("Email").show(ui); });
                f.add(egui_shadcn::Input::new(&mut self.flex_email)
                    .placeholder("john@example.com"));
                f.ui(|ui| { egui_shadcn::Label::new("Phone").show(ui); });
                f.add(egui_shadcn::Input::new(&mut self.flex_phone)
                    .placeholder("+1 555-1234"));
            });
        });

        ui.add_space(16.0);

        // Center utility
        egui_shadcn::Typography::small("Center utility").show(ui);
        ui.add_space(4.0);
        let frame = egui::Frame::NONE
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(80)))
            .inner_margin(egui::Margin::same(0));
        frame.show(ui, |ui| {
            ui.set_min_height(80.0);
            egui_shadcn::center(ui, |ui| {
                ui.horizontal(|ui| {
                    egui_shadcn::Spinner::new().size(16.0).show(ui);
                    ui.add_space(8.0);
                    ui.label("Centered content");
                });
            });
        });
    }

    fn demo_toolbar(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Compact command container for editor and app controls.")
            .show(ui);
        ui.add_space(12.0);

        let tools = [
            egui_shadcn::LucideIcon::MousePointer2,
            egui_shadcn::LucideIcon::PenTool,
            egui_shadcn::LucideIcon::Spline,
            egui_shadcn::LucideIcon::Frame,
            egui_shadcn::LucideIcon::Type,
        ];

        egui_shadcn::Toolbar::new().show(ui, |ui| {
            egui_shadcn::ButtonGroup::show(ui, |ui| {
                for (idx, icon) in tools.iter().enumerate() {
                    let response = egui_shadcn::Button::icon_only(*icon)
                        .variant(egui_shadcn::ButtonVariant::Ghost)
                        .selected(self.toolbar_tool_idx == idx)
                        .show(ui);
                    if response.clicked() {
                        self.toolbar_tool_idx = idx;
                    }
                }
            });

            egui_shadcn::Separator::vertical().show(ui);

            egui_shadcn::ButtonGroup::show(ui, |ui| {
                egui_shadcn::Button::icon_only(egui_shadcn::LucideIcon::Undo2)
                    .variant(egui_shadcn::ButtonVariant::Ghost)
                    .show(ui);
                egui_shadcn::Button::icon_only(egui_shadcn::LucideIcon::Redo2)
                    .variant(egui_shadcn::ButtonVariant::Ghost)
                    .show(ui);
            });

            egui_shadcn::Separator::vertical().show(ui);

            if egui_shadcn::Button::new("Snap")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .selected(self.toolbar_snap)
                .shortcut_text("S")
                .show(ui)
                .clicked()
            {
                self.toolbar_snap = !self.toolbar_snap;
            }

            egui_shadcn::Button::new("Preview")
                .icon(egui_shadcn::LucideIcon::Play)
                .show(ui);
        });

        ui.add_space(14.0);
        egui_shadcn::Typography::small("Dense toolbar").show(ui);
        ui.add_space(4.0);
        egui_shadcn::Toolbar::new().dense().wrap(false).show(ui, |ui| {
            egui_shadcn::Button::icon_only(egui_shadcn::LucideIcon::ZoomOut)
                .variant(egui_shadcn::ButtonVariant::Ghost)
                .size(egui_shadcn::ComponentSize::Sm)
                .show(ui);
            egui_shadcn::Badge::new("100%")
                .variant(egui_shadcn::BadgeVariant::Secondary)
                .show(ui);
            egui_shadcn::Button::icon_only(egui_shadcn::LucideIcon::ZoomIn)
                .variant(egui_shadcn::ButtonVariant::Ghost)
                .size(egui_shadcn::ComponentSize::Sm)
                .show(ui);
        });
    }

    fn demo_status_bar(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Compact container for workspace state and metadata.")
            .show(ui);
        ui.add_space(12.0);

        egui_shadcn::StatusBar::new().show(ui, |ui| {
            egui_shadcn::Badge::new("Saved")
                .variant(egui_shadcn::BadgeVariant::Secondary)
                .show(ui);
            egui_shadcn::Separator::vertical().show(ui);
            egui_shadcn::Typography::small("Canvas 1920 x 1080").show(ui);
            egui_shadcn::Separator::vertical().show(ui);
            egui_shadcn::Typography::small("2 objects selected").show(ui);
            egui_shadcn::Separator::vertical().show(ui);
            egui_shadcn::Kbd::new("Cmd").show(ui);
            ui.label("+");
            egui_shadcn::Kbd::new("S").show(ui);
        });

        ui.add_space(12.0);
        egui_shadcn::StatusBar::new().dense().show(ui, |ui| {
            egui_shadcn::Typography::small("x: 124").show(ui);
            egui_shadcn::Typography::small("y: 88").show(ui);
            egui_shadcn::Typography::small("rotation: -8deg").show(ui);
            egui_shadcn::Badge::new("Snapping")
                .variant(egui_shadcn::BadgeVariant::Outline)
                .show(ui);
        });
    }

    // ── Typography & Display ──────────────────────────────────────────

    fn demo_typography(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::new("Displays styled text at various heading levels.")
            .variant(egui_shadcn::TypographyVariant::Muted)
            .show(ui);
        ui.add_space(12.0);

        egui_shadcn::Typography::h1("Heading 1").show(ui);
        egui_shadcn::Typography::h2("Heading 2").show(ui);
        egui_shadcn::Typography::h3("Heading 3").show(ui);
        egui_shadcn::Typography::h4("Heading 4").show(ui);
        ui.add_space(8.0);
        egui_shadcn::Typography::lead("Lead paragraph text — larger and lighter.").show(ui);
        egui_shadcn::Typography::new("Default paragraph text.").show(ui);
        egui_shadcn::Typography::new("Large variant")
            .variant(egui_shadcn::TypographyVariant::Large)
            .show(ui);
        egui_shadcn::Typography::small("Small text for fine print.").show(ui);
        egui_shadcn::Typography::muted("Muted text for secondary information.").show(ui);
    }

    fn demo_icons(
        &mut self,
        ui: &mut egui::Ui,
        theme: &egui_shadcn::ShadcnTheme,
    ) {
        egui_shadcn::Typography::muted("1671 Lucide icons rendered as vector strokes.").show(ui);
        ui.add_space(12.0);

        // Featured icons at different sizes
        egui_shadcn::Typography::small("Sizes").show(ui);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            for size in [14.0, 18.0, 24.0, 32.0, 48.0] {
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
                egui_shadcn::paint_icon(
                    ui.painter(),
                    rect,
                    &egui_shadcn::LucideIcon::Heart,
                    theme.foreground,
                );
                ui.add_space(8.0);
            }
        });

        ui.add_space(16.0);

        // Search
        egui_shadcn::Input::new(&mut self.icon_search)
            .placeholder("Search icons...")
            .desired_width(300.0)
            .show(ui);
        ui.add_space(12.0);

        // Icon grid
        let icon_size = 24.0;
        let cell = 56.0;
        let available_width = ui.available_width();
        let cols = ((available_width / cell) as usize).max(1);
        let query = self.icon_search.to_lowercase();

        let filtered: Vec<_> = egui_shadcn::icons::lucide_icon::ALL
            .iter()
            .filter(|icon| query.is_empty() || icon.name().contains(&query))
            .collect();

        egui_shadcn::Typography::small(&format!("{} icons", filtered.len())).show(ui);
        ui.add_space(4.0);

        egui::ScrollArea::vertical()
            .max_height(400.0)
            .min_scrolled_height(400.0)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for row in filtered.chunks(cols) {
                    ui.horizontal(|ui| {
                        for icon in row {
                            let (rect, response) = ui.allocate_exact_size(
                                egui::vec2(cell, cell),
                                egui::Sense::hover(),
                            );

                            if ui.is_rect_visible(rect) {
                                let color = if response.hovered() {
                                    theme.primary
                                } else {
                                    theme.foreground
                                };

                                if response.hovered() {
                                    ui.painter().rect_filled(
                                        rect,
                                        egui::CornerRadius::same(theme.radius.round() as u8),
                                        theme.accent,
                                    );
                                }

                                let icon_rect = egui::Rect::from_center_size(
                                    rect.center(),
                                    egui::vec2(icon_size, icon_size),
                                );
                                egui_shadcn::paint_icon(ui.painter(), icon_rect, icon, color);
                            }

                            if response.hovered() {
                                egui_shadcn::Tooltip::new(icon.name()).show(&response);
                            }
                        }
                    });
                }
            });
    }

    fn demo_badge(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Small status indicators with color variants.").show(ui);
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            egui_shadcn::Badge::new("Default")
                .variant(egui_shadcn::BadgeVariant::Default)
                .show(ui);
            egui_shadcn::Badge::new("Secondary")
                .variant(egui_shadcn::BadgeVariant::Secondary)
                .show(ui);
            egui_shadcn::Badge::new("Outline")
                .variant(egui_shadcn::BadgeVariant::Outline)
                .show(ui);
            egui_shadcn::Badge::new("Destructive")
                .variant(egui_shadcn::BadgeVariant::Destructive)
                .show(ui);
        });
    }

    fn demo_color_swatch(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Clickable color swatches for palettes and style controls.")
            .show(ui);
        ui.add_space(12.0);

        let palette = [
            ("Signal", egui::Color32::from_rgb(25, 113, 194)),
            ("Mint", egui::Color32::from_rgb(18, 184, 134)),
            ("Amber", egui::Color32::from_rgb(245, 159, 0)),
            ("Rose", egui::Color32::from_rgb(224, 49, 49)),
            ("Ink", egui::Color32::from_rgb(33, 37, 41)),
        ];

        egui_shadcn::Typography::small("Palette").show(ui);
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            for (idx, (label, color)) in palette.iter().enumerate() {
                let response = egui_shadcn::ColorSwatch::new(*color)
                    .label(*label)
                    .selected(self.color_swatch_idx == idx)
                    .show_hex()
                    .show(ui);
                if response.clicked() {
                    self.color_swatch_idx = idx;
                }
                ui.add_space(6.0);
            }
        });

        ui.add_space(16.0);
        egui_shadcn::Typography::small("Compact states").show(ui);
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            egui_shadcn::ColorSwatch::new(egui::Color32::from_rgb(25, 113, 194))
                .selected(true)
                .show(ui);
            egui_shadcn::ColorSwatch::new(egui::Color32::from_rgba_unmultiplied(
                25, 113, 194, 120,
            ))
            .show(ui);
            egui_shadcn::ColorSwatch::new(egui::Color32::TRANSPARENT)
                .label("Transparent")
                .show_hex()
                .show(ui);
        });
    }

    fn demo_kbd(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Keyboard key indicators.").show(ui);
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            egui_shadcn::Kbd::new("Ctrl").show(ui);
            ui.label("+");
            egui_shadcn::Kbd::new("C").show(ui);
            ui.add_space(16.0);
            egui_shadcn::Kbd::new("Cmd").show(ui);
            ui.label("+");
            egui_shadcn::Kbd::new("K").show(ui);
            ui.add_space(16.0);
            egui_shadcn::Kbd::new("Shift").show(ui);
            ui.label("+");
            egui_shadcn::Kbd::new("Enter").show(ui);
        });
    }

    fn demo_separator(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Visual divider between content sections.").show(ui);
        ui.add_space(12.0);

        ui.label("Content above");
        egui_shadcn::Separator::horizontal().show(ui);
        ui.label("Content below");
        ui.add_space(12.0);
        egui_shadcn::Separator::horizontal().text("With Label").show(ui);
        ui.label("Content after labeled separator");
    }

    fn demo_avatar(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("User avatar circles with initials.").show(ui);
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            egui_shadcn::Avatar::new("CN").show(ui);
            egui_shadcn::Avatar::new("LR").show(ui);
            egui_shadcn::Avatar::new("JD").show(ui);
            egui_shadcn::Avatar::new("AB").show(ui);
        });
    }

    fn demo_skeleton(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Placeholder loading shapes.").show(ui);
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            egui_shadcn::Skeleton::new(40.0, 40.0).circle().show(ui);
            ui.vertical(|ui| {
                egui_shadcn::Skeleton::new(200.0, 16.0).show(ui);
                ui.add_space(4.0);
                egui_shadcn::Skeleton::new(150.0, 16.0).show(ui);
            });
        });
        ui.add_space(8.0);
        egui_shadcn::Skeleton::new(ui.available_width().min(400.0), 120.0).show(ui);
    }

    fn demo_spinner(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Animated loading indicator.").show(ui);
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            egui_shadcn::Spinner::new().size(16.0).show(ui);
            ui.add_space(12.0);
            egui_shadcn::Spinner::new().size(24.0).show(ui);
            ui.add_space(12.0);
            egui_shadcn::Spinner::new().size(36.0).show(ui);
        });
    }

    fn demo_progress(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Progress bar indicating completion.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Progress::new(self.progress_val).show(ui);
        ui.add_space(8.0);
        ui.add(egui_shadcn::Slider::new(
            &mut { self.progress_val as f64 },
            0.0..=1.0,
        ));
        ui.label(format!("{:.0}%", self.progress_val * 100.0));

        ui.add_space(12.0);
        egui_shadcn::Typography::small("Different values:").show(ui);
        ui.add_space(4.0);
        egui_shadcn::Progress::new(0.25).show(ui);
        ui.add_space(4.0);
        egui_shadcn::Progress::new(0.50).show(ui);
        ui.add_space(4.0);
        egui_shadcn::Progress::new(0.75).show(ui);
        ui.add_space(4.0);
        egui_shadcn::Progress::new(1.0).show(ui);
    }

    fn demo_empty(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Empty state placeholder with call to action.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Empty::show(ui, |ui| {
            egui_shadcn::Avatar::new("?").show(ui);
            ui.add_space(8.0);
            egui_shadcn::Label::new("No results found").show(ui);
            egui_shadcn::FieldDescription::show(
                ui,
                "Try adjusting your search or filter criteria.",
            );
            ui.add_space(8.0);
            egui_shadcn::Button::new("Reset Filters")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .size(egui_shadcn::ComponentSize::Sm)
                .show(ui);
        });
    }

    // ── Inputs ────────────────────────────────────────────────────────

    fn demo_button(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Clickable buttons with variant styles and sizes.")
            .show(ui);
        ui.add_space(12.0);

        egui_shadcn::Typography::small("Variants").show(ui);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            egui_shadcn::Button::new("Default").show(ui);
            egui_shadcn::Button::new("Destructive")
                .variant(egui_shadcn::ButtonVariant::Destructive)
                .show(ui);
            egui_shadcn::Button::new("Outline")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui);
            egui_shadcn::Button::new("Secondary")
                .variant(egui_shadcn::ButtonVariant::Secondary)
                .show(ui);
            egui_shadcn::Button::new("Ghost")
                .variant(egui_shadcn::ButtonVariant::Ghost)
                .show(ui);
            egui_shadcn::Button::new("Link")
                .variant(egui_shadcn::ButtonVariant::Link)
                .show(ui);
        });

        ui.add_space(12.0);
        egui_shadcn::Typography::small("Sizes").show(ui);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            egui_shadcn::Button::new("XS")
                .size(egui_shadcn::ComponentSize::Xs)
                .show(ui);
            egui_shadcn::Button::new("Small")
                .size(egui_shadcn::ComponentSize::Sm)
                .show(ui);
            egui_shadcn::Button::new("Default").show(ui);
            egui_shadcn::Button::new("Large")
                .size(egui_shadcn::ComponentSize::Lg)
                .show(ui);
        });

        ui.add_space(12.0);
        egui_shadcn::Typography::small("Icon Only").show(ui);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            egui_shadcn::Button::icon_only(egui_shadcn::LucideIcon::Plus)
                .show(ui);
            egui_shadcn::Button::icon_only(egui_shadcn::LucideIcon::Settings)
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui);
            egui_shadcn::Button::icon_only(egui_shadcn::LucideIcon::Trash)
                .variant(egui_shadcn::ButtonVariant::Destructive)
                .show(ui);
            egui_shadcn::Button::icon_only(egui_shadcn::LucideIcon::Heart)
                .variant(egui_shadcn::ButtonVariant::Ghost)
                .show(ui);
            egui_shadcn::Button::icon_only(egui_shadcn::LucideIcon::Search)
                .variant(egui_shadcn::ButtonVariant::Secondary)
                .size(egui_shadcn::ComponentSize::Sm)
                .show(ui);
            egui_shadcn::Button::icon_only(egui_shadcn::LucideIcon::Star)
                .variant(egui_shadcn::ButtonVariant::Outline)
                .size(egui_shadcn::ComponentSize::Lg)
                .show(ui);
        });

        ui.add_space(12.0);
        egui_shadcn::Typography::small("Icon + Text").show(ui);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            egui_shadcn::Button::new("Download")
                .icon(egui_shadcn::LucideIcon::Download)
                .show(ui);
            egui_shadcn::Button::new("Upload")
                .icon(egui_shadcn::LucideIcon::Upload)
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui);
            egui_shadcn::Button::new("Mail")
                .icon(egui_shadcn::LucideIcon::Mail)
                .variant(egui_shadcn::ButtonVariant::Secondary)
                .show(ui);
            egui_shadcn::Button::new("Copy")
                .icon(egui_shadcn::LucideIcon::Copy)
                .variant(egui_shadcn::ButtonVariant::Ghost)
                .size(egui_shadcn::ComponentSize::Sm)
                .show(ui);
        });

        ui.add_space(12.0);
        egui_shadcn::Typography::small("Shortcut Text").show(ui);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            egui_shadcn::Button::new("Save")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .shortcut_text("Cmd+S")
                .show(ui);
            egui_shadcn::Button::new("Open")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .shortcut_text("Cmd+O")
                .show(ui);
        });

        ui.add_space(12.0);
        egui_shadcn::Typography::small("Selected (toggle)").show(ui);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if egui_shadcn::Button::new("Toggle Me")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .selected(self.button_selected)
                .show(ui)
                .clicked()
            {
                self.button_selected = !self.button_selected;
            }
        });

        ui.add_space(12.0);
        egui_shadcn::Typography::small("Disabled").show(ui);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            egui_shadcn::Button::new("Disabled")
                .enabled(false)
                .show(ui);
            egui_shadcn::Button::new("Disabled Outline")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .enabled(false)
                .show(ui);
        });

        ui.add_space(12.0);
        egui_shadcn::Typography::small("Button Group").show(ui);
        ui.add_space(4.0);
        egui_shadcn::ButtonGroup::show(ui, |ui| {
            egui_shadcn::Button::new("Left")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui);
            egui_shadcn::Button::new("Center")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui);
            egui_shadcn::Button::new("Right")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui);
        });
    }

    fn demo_checkbox(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Toggle boolean values with a checkbox.").show(ui);
        ui.add_space(12.0);

        ui.add(
            egui_shadcn::Checkbox::new(&mut self.checkbox_val)
                .label("Accept terms and conditions"),
        );
        ui.add_space(4.0);
        ui.label(format!("Checked: {}", self.checkbox_val));
    }

    fn demo_switch(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("A toggle switch for on/off states.").show(ui);
        ui.add_space(12.0);

        ui.add(egui_shadcn::Switch::new(&mut self.switch_val).label("Airplane mode"));
        ui.add_space(4.0);
        ui.label(format!("Enabled: {}", self.switch_val));
    }

    fn demo_radio(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Individual radio buttons for exclusive selection.")
            .show(ui);
        ui.add_space(12.0);

        if ui
            .add(egui_shadcn::Radio::new(&mut self.radio_a).label("Option A"))
            .clicked()
        {
            self.radio_b = false;
            self.radio_c = false;
        }
        if ui
            .add(egui_shadcn::Radio::new(&mut self.radio_b).label("Option B"))
            .clicked()
        {
            self.radio_a = false;
            self.radio_c = false;
        }
        if ui
            .add(egui_shadcn::Radio::new(&mut self.radio_c).label("Option C"))
            .clicked()
        {
            self.radio_a = false;
            self.radio_b = false;
        }
    }

    fn demo_radio_group(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("A group of radio buttons managed together.").show(ui);
        ui.add_space(12.0);

        let options = [
            "Option A".to_owned(),
            "Option B".to_owned(),
            "Option C".to_owned(),
        ];
        ui.add(egui_shadcn::RadioGroup::new(
            &mut self.radio_group_val,
            &options,
        ));
        ui.add_space(4.0);
        ui.label(format!("Selected: {}", self.radio_group_val));
    }

    fn demo_toggle(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Toggle buttons with default and outline variants.")
            .show(ui);
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.add(egui_shadcn::Toggle::new(&mut self.toggle_bold, "B"));
            ui.add(
                egui_shadcn::Toggle::new(&mut self.toggle_italic, "I")
                    .variant(egui_shadcn::ToggleVariant::Outline),
            );
        });
        ui.add_space(4.0);
        ui.label(format!(
            "Bold: {}, Italic: {}",
            self.toggle_bold, self.toggle_italic
        ));
    }

    fn demo_toggle_group(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Exclusive toggle group — only one active at a time.")
            .show(ui);
        ui.add_space(12.0);

        egui_shadcn::ToggleGroup::new(vec![
            "Left".to_owned(),
            "Center".to_owned(),
            "Right".to_owned(),
        ])
        .show(ui, &mut self.toggle_group_idx);
        ui.add_space(4.0);
        ui.label(format!("Selected index: {}", self.toggle_group_idx));
    }

    fn demo_slider(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Drag to select a numeric value within a range.")
            .show(ui);
        ui.add_space(12.0);

        egui_shadcn::Slider::new(&mut self.slider_val, 0.0..=100.0)
            .step(1.0)
            .width(ui.available_width().min(400.0))
            .show(ui);
        ui.add_space(4.0);
        ui.label(format!("Value: {:.0}", self.slider_val));
    }

    fn demo_input(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Single-line text input field.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Input::new(&mut self.input_text)
            .placeholder("Type something...")
            .desired_width(300.0)
            .show(ui);
        ui.add_space(4.0);
        ui.label(format!("Value: \"{}\"", self.input_text));
    }

    fn demo_textarea(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Multi-line text area.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Textarea::new(&mut self.textarea_text)
            .placeholder("Write a message...")
            .desired_width(400.0)
            .min_height(80.0)
            .show(ui);
    }

    fn demo_select(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Dropdown selection from a list.").show(ui);
        ui.add_space(12.0);

        let fruits = vec![
            "Apple".to_owned(),
            "Banana".to_owned(),
            "Cherry".to_owned(),
            "Grape".to_owned(),
            "Mango".to_owned(),
        ];
        ui.add(
            egui_shadcn::Select::new(&mut self.select_val, &fruits)
                .placeholder("Pick a fruit..."),
        );
        ui.add_space(4.0);
        ui.label(format!("Selected: {:?}", self.select_val));
    }

    fn demo_combobox(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Searchable dropdown with type-ahead filtering.")
            .show(ui);
        ui.add_space(12.0);

        let frameworks = vec![
            "React".to_owned(),
            "Vue".to_owned(),
            "Angular".to_owned(),
            "Svelte".to_owned(),
            "Solid".to_owned(),
        ];
        egui_shadcn::Combobox::new(frameworks)
            .placeholder("Select framework...")
            .show(ui, &mut self.combobox_selected, &mut self.combobox_search);
        ui.add_space(4.0);
        ui.label(format!("Selected index: {:?}", self.combobox_selected));
    }

    fn demo_input_otp(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("One-time passcode digit input boxes.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::InputOtp::new(6).show(ui, &mut self.otp_value);
        ui.add_space(4.0);
        ui.label(format!("OTP: \"{}\"", self.otp_value));
    }

    // ── Containers ────────────────────────────────────────────────────

    fn demo_card(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Bordered container for grouping content.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Card::new().show(ui, |ui| {
            egui_shadcn::Typography::h4("Card Title").show(ui);
            ui.add_space(4.0);
            ui.label("This is a card with some descriptive content inside.");
            ui.add_space(8.0);
            egui_shadcn::Button::new("Action")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .size(egui_shadcn::ComponentSize::Sm)
                .show(ui);
        });
    }

    fn demo_property_grid(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Inspector-style rows for labels and editable controls.")
            .show(ui);
        ui.add_space(12.0);

        let blend_modes = vec![
            "Normal".to_owned(),
            "Multiply".to_owned(),
            "Screen".to_owned(),
            "Overlay".to_owned(),
        ];

        ui.set_max_width(460.0);
        egui_shadcn::PropertyGrid::new()
            .label_width(92.0)
            .show(ui, |ui| {
                egui_shadcn::PropertyRow::new("Position").show(ui, |ui| {
                    egui_shadcn::NumberInput::new(&mut self.prop_x)
                        .suffix("px")
                        .width(76.0)
                        .show(ui);
                    egui_shadcn::NumberInput::new(&mut self.prop_y)
                        .suffix("px")
                        .width(76.0)
                        .show(ui);
                });

                egui_shadcn::PropertyRow::new("Size").show(ui, |ui| {
                    egui_shadcn::NumberInput::new(&mut self.prop_width)
                        .range(1.0..=4096.0)
                        .suffix("w")
                        .width(76.0)
                        .show(ui);
                    egui_shadcn::NumberInput::new(&mut self.prop_height)
                        .range(1.0..=4096.0)
                        .suffix("h")
                        .width(76.0)
                        .show(ui);
                });

                egui_shadcn::PropertyRow::new("Rotation").show(ui, |ui| {
                    egui_shadcn::Slider::new(&mut self.prop_rotation, -180.0..=180.0)
                        .step(1.0)
                        .width(180.0)
                        .suffix("deg")
                        .show(ui);
                });

                egui_shadcn::PropertyRow::new("Opacity").show(ui, |ui| {
                    egui_shadcn::Slider::new(&mut self.prop_opacity, 0.0..=100.0)
                        .step(1.0)
                        .width(180.0)
                        .suffix("%")
                        .show(ui);
                });

                egui_shadcn::PropertyRow::new("Blend").show(ui, |ui| {
                    egui_shadcn::SelectValue::new(&mut self.prop_blend_mode, &blend_modes)
                        .width(112.0)
                        .show(ui);
                });

                egui_shadcn::PropertyRow::new("Fill").show(ui, |ui| {
                    egui_shadcn::ColorSwatch::new(egui::Color32::from_rgb(25, 113, 194))
                        .label("Primary")
                        .show_hex()
                        .show(ui);
                });
            });
    }

    fn demo_alert(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Status messages with default and destructive variants.")
            .show(ui);
        ui.add_space(12.0);

        egui_shadcn::Alert::new()
            .title("Heads up!")
            .show(ui, |ui| {
                ui.label("You can add components to your app using the CLI.");
            });

        ui.add_space(8.0);

        egui_shadcn::Alert::new()
            .title("Error")
            .variant(egui_shadcn::AlertVariant::Destructive)
            .show(ui, |ui| {
                ui.label("Your session has expired. Please log in again.");
            });
    }

    fn demo_accordion(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Expandable sections — click to toggle.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Accordion::new(vec![
            (
                "Is it accessible?".to_owned(),
                "Yes. It adheres to the WAI-ARIA design pattern.".to_owned(),
            ),
            (
                "Is it styled?".to_owned(),
                "Yes. It comes with default styles matching shadcn/ui.".to_owned(),
            ),
            (
                "Is it animated?".to_owned(),
                "Yes. It has smooth open/close transitions.".to_owned(),
            ),
        ])
        .multiple()
        .show(ui, &mut self.accordion_open);
    }

    fn demo_collapsible(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("A section that can be toggled open or closed.")
            .show(ui);
        ui.add_space(12.0);

        egui_shadcn::Collapsible::new("Click to toggle").show(
            ui,
            &mut self.collapsible_open,
            |ui| {
                ui.label("This content is hidden when the collapsible is closed.");
                ui.label("You can put any widgets inside here.");
            },
        );
    }

    fn demo_tabs(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Tabbed content panels.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Tabs::new(vec![
            "Account".to_owned(),
            "Password".to_owned(),
            "Settings".to_owned(),
        ])
        .show(ui, &mut self.tabs_idx, |ui, idx| match idx {
            0 => {
                ui.label("Manage your account settings and preferences.");
            }
            1 => {
                ui.label("Change your password and security settings.");
            }
            _ => {
                ui.label("Configure application settings.");
            }
        });
    }

    fn demo_table(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Data table with headers and striped rows.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Table::new(vec![
            "Name".to_owned(),
            "Status".to_owned(),
            "Role".to_owned(),
        ])
        .rows(vec![
            vec![
                "Alice".to_owned(),
                "Active".to_owned(),
                "Admin".to_owned(),
            ],
            vec![
                "Bob".to_owned(),
                "Inactive".to_owned(),
                "Editor".to_owned(),
            ],
            vec![
                "Charlie".to_owned(),
                "Active".to_owned(),
                "Viewer".to_owned(),
            ],
            vec![
                "Diana".to_owned(),
                "Active".to_owned(),
                "Editor".to_owned(),
            ],
        ])
        .striped()
        .show(ui);
    }

    fn demo_aspect_ratio(
        &self,
        ui: &mut egui::Ui,
        theme: &egui_shadcn::ShadcnTheme,
    ) {
        egui_shadcn::Typography::muted("Maintains a fixed width-to-height ratio.").show(ui);
        ui.add_space(12.0);

        ui.set_max_width(300.0);
        egui_shadcn::AspectRatio::new(16.0 / 9.0).show(ui, |ui| {
            let rect = ui.available_rect_before_wrap();
            ui.painter().rect_filled(
                rect,
                egui::CornerRadius::same(theme.radius.round() as u8),
                theme.muted,
            );
            let galley = ui.painter().layout_no_wrap(
                "16:9".to_owned(),
                egui::FontId::proportional(20.0),
                theme.muted_foreground,
            );
            let center = rect.center();
            ui.painter().galley(
                egui::pos2(
                    center.x - galley.size().x / 2.0,
                    center.y - galley.size().y / 2.0,
                ),
                galley,
                theme.muted_foreground,
            );
        });
    }

    fn demo_scroll_area(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Themed scrollable region with max height.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::ScrollArea::new(150.0).show(ui, |ui| {
            for i in 1..=20 {
                ui.label(format!("Scrollable item {}", i));
            }
        });
    }

    fn demo_resizable(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Draggable split pane with adjustable divider.")
            .show(ui);
        ui.add_space(12.0);

        egui_shadcn::Resizable::new(self.resizable_fraction).height(150.0).show(
            ui,
            &mut self.resizable_fraction,
            |ui| {
                egui_shadcn::Card::new().show(ui, |ui| {
                    ui.label("Left Panel");
                });
            },
            |ui| {
                egui_shadcn::Card::new().show(ui, |ui| {
                    ui.label("Right Panel");
                });
            },
        );
    }

    // ── Navigation ────────────────────────────────────────────────────

    fn demo_breadcrumb(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Navigation path indicator.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Breadcrumb::new(vec![
            "Home".to_owned(),
            "Components".to_owned(),
            "Breadcrumb".to_owned(),
        ])
        .show(ui);

        ui.add_space(8.0);

        egui_shadcn::Breadcrumb::new(vec![
            "Dashboard".to_owned(),
            "Settings".to_owned(),
            "Profile".to_owned(),
        ])
        .separator(">")
        .show(ui);
    }

    fn demo_pagination(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Page navigation controls.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Pagination::new(10)
            .max_visible(5)
            .show(ui, &mut self.pagination_page);
        ui.add_space(4.0);
        ui.label(format!("Page: {}", self.pagination_page + 1));
    }

    fn demo_navigation_menu(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Horizontal navigation bar with active indicator.")
            .show(ui);
        ui.add_space(12.0);

        egui_shadcn::NavigationMenu::new(vec![
            "Getting Started".to_owned(),
            "Components".to_owned(),
            "Documentation".to_owned(),
            "Examples".to_owned(),
        ])
        .show(ui, &mut self.nav_menu_idx);
    }

    fn demo_menubar(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Horizontal menu bar with clickable items.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Menubar::new().show(ui, |ui| {
            egui_shadcn::Menubar::menu(
                ui, "File",
                &["New", "Open", "Save", "Exit"],
                |_idx| {},
            );
            egui_shadcn::Menubar::menu(
                ui, "Edit",
                &["Undo", "Redo", "Cut", "Copy", "Paste"],
                |_idx| {},
            );
            egui_shadcn::Menubar::menu(
                ui, "View",
                &["Zoom In", "Zoom Out", "Full Screen"],
                |_idx| {},
            );
            egui_shadcn::Menubar::menu(
                ui, "Help",
                &["Documentation", "About"],
                |_idx| {},
            );
        });
    }

    fn demo_sidebar(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Collapsible sidebar navigation panel.").show(ui);
        ui.add_space(12.0);

        ui.set_min_height(200.0);
        ui.horizontal(|ui| {
            egui_shadcn::Sidebar::new()
                .width(180.0)
                .collapsible()
                .show(ui, &mut self.sidebar_collapsed, |ui| {
                    for item in ["Dashboard", "Projects", "Team", "Settings"] {
                        ui.label(item);
                        ui.add_space(4.0);
                    }
                });

            egui_shadcn::Card::new().show(ui, |ui| {
                ui.set_min_width(200.0);
                ui.label("Main content area");
            });
        });
    }

    // ── Overlays ──────────────────────────────────────────────────────

    fn demo_dialog(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Modal dialog that overlays the screen.").show(ui);
        ui.add_space(12.0);

        if egui_shadcn::Button::new("Open Dialog").show(ui).clicked() {
            self.dialog_open = true;
        }
    }

    fn demo_alert_dialog(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Confirmation dialog with cancel/confirm actions.")
            .show(ui);
        ui.add_space(12.0);

        if egui_shadcn::Button::new("Open Alert Dialog")
            .variant(egui_shadcn::ButtonVariant::Destructive)
            .show(ui)
            .clicked()
        {
            self.alert_dialog_open = true;
        }
    }

    fn demo_sheet(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Side panel overlay that slides in.").show(ui);
        ui.add_space(12.0);

        if egui_shadcn::Button::new("Open Sheet").show(ui).clicked() {
            self.sheet_open = true;
        }
    }

    fn demo_drawer(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Bottom drawer panel overlay.").show(ui);
        ui.add_space(12.0);

        if egui_shadcn::Button::new("Open Drawer").show(ui).clicked() {
            self.drawer_open = true;
        }
    }

    fn demo_popover(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Floating content panel anchored to a trigger.")
            .show(ui);
        ui.add_space(12.0);

        let trigger = egui_shadcn::Button::new("Open Popover")
            .variant(egui_shadcn::ButtonVariant::Outline)
            .show(ui);
        egui_shadcn::Popover::new().show(ui, &trigger, |ui| {
            ui.label("Popover content");
            ui.add_space(4.0);
            ui.label("Place any widgets here.");
        });
    }

    fn demo_hover_card(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Card that appears on hover.").show(ui);
        ui.add_space(12.0);

        let trigger = ui.add(
            egui::Label::new(egui::RichText::new("@nextjs").size(14.0).underline())
                .sense(egui::Sense::hover()),
        );
        egui_shadcn::HoverCard::new().show(&trigger, |ui| {
            egui_shadcn::Avatar::new("NJ").show(ui);
            ui.add_space(4.0);
            ui.label("The React Framework");
            ui.label("Created by @vercel");
        });
    }

    fn demo_tooltip(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Small hint text that appears on hover.").show(ui);
        ui.add_space(12.0);

        let response = egui_shadcn::Button::new("Hover me")
            .variant(egui_shadcn::ButtonVariant::Outline)
            .show(ui);
        egui_shadcn::Tooltip::new("This is a tooltip").show(&response);
    }

    fn demo_context_menu(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Right-click menu on any element.").show(ui);
        ui.add_space(12.0);

        let response = ui.add(
            egui::Label::new(
                egui::RichText::new("Right-click this text")
                    .size(14.0)
                    .strong(),
            )
            .sense(egui::Sense::click()),
        );
        egui_shadcn::ContextMenu::show(
            &response,
            &["Cut", "Copy", "Paste", "Delete"],
            |_idx| {},
        );
    }

    fn demo_dropdown_menu(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Dropdown menu triggered by a button click.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Typography::small("Simple").show(ui);
        ui.add_space(4.0);
        let trigger = egui_shadcn::Button::new("Open Menu")
            .variant(egui_shadcn::ButtonVariant::Outline)
            .show(ui);
        egui_shadcn::DropdownMenu::show(
            ui,
            &trigger,
            &["Profile", "Settings", "Billing", "Sign Out"],
            |_idx| {},
        );

        ui.add_space(12.0);
        egui_shadcn::Typography::small("Rich (shortcuts, separators, disabled)").show(ui);
        ui.add_space(4.0);
        let trigger2 = egui_shadcn::Button::new("File Menu")
            .variant(egui_shadcn::ButtonVariant::Outline)
            .show(ui);
        egui_shadcn::DropdownMenu::show_rich(
            ui,
            &trigger2,
            &[
                egui_shadcn::MenuItem::with_shortcut("New File", "Cmd+N"),
                egui_shadcn::MenuItem::with_shortcut("Open...", "Cmd+O"),
                egui_shadcn::MenuItem::with_shortcut("Save", "Cmd+S"),
                egui_shadcn::MenuItem::separator(),
                egui_shadcn::MenuItem::label("Export as PDF"),
                egui_shadcn::MenuItem::full("Share...", None, false),
            ],
            |_idx| {},
        );
    }

    fn demo_command(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Command palette for searching and executing actions.")
            .show(ui);
        ui.add_space(12.0);

        if egui_shadcn::Button::new("Open Command Palette")
            .show(ui)
            .clicked()
        {
            self.command_open = true;
        }
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            egui_shadcn::Typography::small("Shortcut:").show(ui);
            egui_shadcn::Kbd::new("Cmd").show(ui);
            ui.label("+");
            egui_shadcn::Kbd::new("K").show(ui);
        });
    }

    // ── Feedback ──────────────────────────────────────────────────────

    fn demo_toast(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Temporary notifications in the bottom-right corner.")
            .show(ui);
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            if egui_shadcn::Button::new("Default Toast").show(ui).clicked() {
                let time = ui.ctx().input(|i| i.time);
                self.toast_state
                    .add("Event has been created", egui_shadcn::ToastVariant::Default, time);
            }
            if egui_shadcn::Button::new("Success")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui)
                .clicked()
            {
                let time = ui.ctx().input(|i| i.time);
                self.toast_state.add_with_description(
                    "Success",
                    "Changes saved successfully.",
                    egui_shadcn::ToastVariant::Success,
                    time,
                );
            }
            if egui_shadcn::Button::new("Error")
                .variant(egui_shadcn::ButtonVariant::Destructive)
                .show(ui)
                .clicked()
            {
                let time = ui.ctx().input(|i| i.time);
                self.toast_state.add_with_description(
                    "Error",
                    "Something went wrong.",
                    egui_shadcn::ToastVariant::Error,
                    time,
                );
            }
        });

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if egui_shadcn::Button::new("Warning")
                .variant(egui_shadcn::ButtonVariant::Secondary)
                .show(ui)
                .clicked()
            {
                let time = ui.ctx().input(|i| i.time);
                self.toast_state
                    .add("Low disk space", egui_shadcn::ToastVariant::Warning, time);
            }
            if egui_shadcn::Button::new("Info")
                .variant(egui_shadcn::ButtonVariant::Ghost)
                .show(ui)
                .clicked()
            {
                let time = ui.ctx().input(|i| i.time);
                self.toast_state.add_with_description(
                    "Info",
                    "Your trial ends in 3 days.",
                    egui_shadcn::ToastVariant::Info,
                    time,
                );
            }
        });
    }

    fn demo_carousel(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Navigate through a set of slides.").show(ui);
        ui.add_space(12.0);

        let slides = ["Slide 1: Welcome", "Slide 2: Features", "Slide 3: Pricing"];
        egui_shadcn::Carousel::new(slides.len()).show(
            ui,
            &mut self.carousel_idx,
            |ui, idx| {
                egui_shadcn::Card::new().show(ui, |ui| {
                    ui.set_min_height(80.0);
                    egui_shadcn::Typography::h4(slides[idx]).show(ui);
                    ui.add_space(4.0);
                    ui.label(format!("Content for slide {} goes here.", idx + 1));
                });
            },
        );
    }

    fn demo_calendar(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Month grid for selecting a date.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Calendar::new().show(
            ui,
            &mut self.calendar_year,
            &mut self.calendar_month,
            &mut self.calendar_day,
        );
        ui.add_space(4.0);
        if self.calendar_day > 0 {
            ui.label(format!(
                "Selected: {:04}-{:02}-{:02}",
                self.calendar_year, self.calendar_month, self.calendar_day
            ));
        }
    }

    // ── New Inputs ─────────────────────────────────────────────────

    fn demo_number_input(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Numeric input with drag, range, prefix/suffix.")
            .show(ui);
        ui.add_space(12.0);

        egui_shadcn::Typography::small("f64 with range and suffix").show(ui);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            egui_shadcn::NumberInput::new(&mut self.number_f64)
                .range(0.0..=100.0)
                .speed(0.5)
                .suffix("px")
                .width(100.0)
                .show(ui);
            ui.label(format!("{:.1}", self.number_f64));
        });

        ui.add_space(12.0);
        egui_shadcn::Typography::small("f32 with decimals and prefix").show(ui);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            egui_shadcn::NumberInput::f32(&mut self.number_f32)
                .decimals(2)
                .prefix("$")
                .width(80.0)
                .show(ui);
            ui.label(format!("{:.2}", self.number_f32));
        });

        ui.add_space(12.0);
        egui_shadcn::Typography::small("i32 integer").show(ui);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            egui_shadcn::NumberInput::i32(&mut self.number_i32)
                .range(0.0..=50.0)
                .width(60.0)
                .show(ui);
            ui.label(format!("{}", self.number_i32));
        });
    }

    fn demo_input_group(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Input with prefix text and suffix addons.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Typography::small("With prefix").show(ui);
        ui.add_space(4.0);
        egui_shadcn::InputGroup::show(
            ui,
            &mut self.input_group_text,
            "example.com",
            Some("https://"),
            None::<fn(&mut egui::Ui)>,
        );

        ui.add_space(12.0);
        egui_shadcn::Typography::small("With prefix and suffix button").show(ui);
        ui.add_space(4.0);
        egui_shadcn::InputGroup::show(
            ui,
            &mut self.input_group_text,
            "Search...",
            None,
            Some(|ui: &mut egui::Ui| {
                egui_shadcn::Button::icon_only(egui_shadcn::LucideIcon::Search)
                    .variant(egui_shadcn::ButtonVariant::Ghost)
                    .size(egui_shadcn::ComponentSize::Sm)
                    .show(ui);
            }),
        );
    }

    // ── New Containers ───────────────────────────────────────────

    fn demo_icon_tabs(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Icon-based tabs with tooltips.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::IconTabs::new(vec![
            egui_shadcn::TabEntry::Icon {
                icon: egui_shadcn::LucideIcon::House,
                tooltip: "Home".to_owned(),
            },
            egui_shadcn::TabEntry::Icon {
                icon: egui_shadcn::LucideIcon::Settings,
                tooltip: "Settings".to_owned(),
            },
            egui_shadcn::TabEntry::Icon {
                icon: egui_shadcn::LucideIcon::CircleUser,
                tooltip: "Profile".to_owned(),
            },
            egui_shadcn::TabEntry::Icon {
                icon: egui_shadcn::LucideIcon::Bell,
                tooltip: "Notifications".to_owned(),
            },
        ])
        .show(ui, &mut self.icon_tabs_idx, |ui, idx| match idx {
            0 => { ui.label("Home content"); }
            1 => { ui.label("Settings content"); }
            2 => { ui.label("Profile content"); }
            _ => { ui.label("Notifications content"); }
        });
    }

    fn demo_item(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("List item container with variant styles.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::Typography::small("Default (no border)").show(ui);
        ui.add_space(4.0);
        egui_shadcn::Item::new().show(ui, |ui| {
            ui.horizontal(|ui| {
                egui_shadcn::Avatar::new("JD").show(ui);
                ui.vertical(|ui| {
                    ui.label("John Doe");
                    egui_shadcn::Typography::muted("john@example.com").show(ui);
                });
            });
        });

        ui.add_space(8.0);
        egui_shadcn::Typography::small("Outline variant").show(ui);
        ui.add_space(4.0);
        egui_shadcn::Item::new()
            .variant(egui_shadcn::ItemVariant::Outline)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    egui_shadcn::Avatar::new("AB").show(ui);
                    ui.vertical(|ui| {
                        ui.label("Alice Brown");
                        egui_shadcn::Typography::muted("alice@example.com").show(ui);
                    });
                });
            });
    }

    // ── Data Visualization ───────────────────────────────────────

    fn demo_area_chart(&self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Smooth stacked area chart with Catmull-Rom curves.")
            .show(ui);
        ui.add_space(12.0);

        let labels: Vec<String> = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        egui_shadcn::Typography::small("Single series").show(ui);
        ui.add_space(4.0);
        egui_shadcn::AreaChart::new(labels.clone())
            .series(egui_shadcn::AreaSeries {
                values: vec![20.0, 45.0, 35.0, 60.0, 50.0, 75.0],
                color: egui::Color32::from_rgb(99, 102, 241), // indigo
            })
            .height(180.0)
            .show(ui);

        ui.add_space(16.0);
        egui_shadcn::Typography::small("Stacked series").show(ui);
        ui.add_space(4.0);
        egui_shadcn::AreaChart::new(labels)
            .series(egui_shadcn::AreaSeries {
                values: vec![30.0, 40.0, 35.0, 50.0, 49.0, 60.0],
                color: egui::Color32::from_rgb(99, 102, 241),
            })
            .series(egui_shadcn::AreaSeries {
                values: vec![20.0, 30.0, 45.0, 25.0, 35.0, 40.0],
                color: egui::Color32::from_rgb(16, 185, 129), // emerald
            })
            .stacked()
            .height(200.0)
            .show(ui);
    }

    // ── Form Layout ──────────────────────────────────────────────

    fn demo_field_group(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Form layout utilities: FieldGroup, FieldSet, FieldLegend, FieldDescription.")
            .show(ui);
        ui.add_space(12.0);

        egui_shadcn::FieldGroup::show(ui, |ui| {
            egui_shadcn::FieldSet::show(ui, "Personal Information", |ui| {
                egui_shadcn::Label::new("First Name").show(ui);
                egui_shadcn::Input::new(&mut self.flex_first_name)
                    .placeholder("John")
                    .desired_width(300.0)
                    .show(ui);
                egui_shadcn::FieldDescription::show(ui, "Your legal first name.");

                ui.add_space(8.0);

                egui_shadcn::Label::new("Last Name").show(ui);
                egui_shadcn::Input::new(&mut self.flex_last_name)
                    .placeholder("Doe")
                    .desired_width(300.0)
                    .show(ui);
            });

            egui_shadcn::FieldSet::show(ui, "Contact", |ui| {
                egui_shadcn::Label::new("Email").show(ui);
                egui_shadcn::Input::new(&mut self.flex_email)
                    .placeholder("john@example.com")
                    .desired_width(300.0)
                    .show(ui);
                egui_shadcn::FieldDescription::show(ui, "We will never share your email.");
            });
        });
    }

    fn demo_date_picker(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Typography::muted("Date picker with popover calendar.").show(ui);
        ui.add_space(12.0);

        egui_shadcn::DatePicker::new()
            .placeholder("Pick a date")
            .show(ui, &mut self.date_picker_state);
        ui.add_space(4.0);
        if self.date_picker_state.is_set() {
            ui.label(format!("Selected: {}", self.date_picker_state.format()));
        }
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 750.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Component Dashboard \u{2014} egui-shadcn",
        options,
        Box::new(|cc| {
            egui_shadcn::setup_fonts(&cc.egui_ctx);
            Ok(Box::new(DashboardApp::default()))
        }),
    )
}
