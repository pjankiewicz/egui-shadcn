//! Full shadcn/ui homepage demo replica with 4-column grid layout.

struct ShadcnDemo {
    // Theme
    dark_mode: bool,

    // Column 1: Payment form
    name: String,
    card_number: String,
    cvv: String,
    selected_month: Option<String>,
    selected_year: Option<String>,
    billing_same: bool,
    comments: String,

    // Column 2
    slider_low: f64,
    search_text: String,
    url_text: String,

    // Column 3
    prefix_text: String,
    infra_choice: String,
    gpu_count: String,
    wallpaper_tinting: bool,

    // Column 4
    prompt_text: String,
    terms_accepted: bool,
    hear_social: bool,
    hear_search: bool,
    hear_referral: bool,
    hear_other: bool,
}

impl Default for ShadcnDemo {
    fn default() -> Self {
        Self {
            dark_mode: true,
            name: String::new(),
            card_number: String::new(),
            cvv: String::new(),
            selected_month: None,
            selected_year: None,
            billing_same: true,
            comments: String::new(),
            slider_low: 200.0,
            search_text: String::new(),
            url_text: String::new(),
            prefix_text: String::new(),
            infra_choice: "Kubernetes".to_owned(),
            gpu_count: "1".to_owned(),
            wallpaper_tinting: true,
            prompt_text: String::new(),
            terms_accepted: false,
            hear_social: true,
            hear_search: false,
            hear_referral: false,
            hear_other: false,
        }
    }
}

impl eframe::App for ShadcnDemo {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let theme = if self.dark_mode {
            egui_shadcn::theme::shadcn_theme_dark::dark()
        } else {
            egui_shadcn::theme::shadcn_theme_light::light()
        };
        egui_shadcn::ShadcnThemeExt::set_shadcn_theme(ui.ctx(), theme.clone());

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme.background))
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(24.0);

                    // Header with dark mode toggle
                    ui.horizontal(|ui| {
                        ui.add_space(24.0);
                        let galley = ui.painter().layout_no_wrap(
                            "Dark mode".to_owned(),
                            egui::FontId::proportional(14.0),
                            theme.foreground,
                        );
                        let (r, _) =
                            ui.allocate_exact_size(galley.size(), egui::Sense::hover());
                        ui.painter().galley(r.min, galley, theme.foreground);
                        ui.add(egui_shadcn::Switch::new(&mut self.dark_mode));
                    });
                    ui.add_space(24.0);

                    // 4-column grid
                    let margin = 24.0;
                    let gap = 16.0;
                    let available = ui.available_width() - margin * 2.0 - gap * 3.0;
                    let col_w = (available / 4.0).max(200.0);

                    ui.horizontal(|ui| {
                        ui.add_space(margin);

                        // Column 1
                        ui.vertical(|ui| {
                            ui.set_width(col_w);
                            self.column_1(ui);
                        });
                        ui.add_space(gap);

                        // Column 2
                        ui.vertical(|ui| {
                            ui.set_width(col_w);
                            self.column_2(ui);
                        });
                        ui.add_space(gap);

                        // Column 3
                        ui.vertical(|ui| {
                            ui.set_width(col_w);
                            self.column_3(ui);
                        });
                        ui.add_space(gap);

                        // Column 4
                        ui.vertical(|ui| {
                            ui.set_width(col_w);
                            self.column_4(ui);
                        });
                    });

                    ui.add_space(32.0);
                });
            });
    }
}

impl ShadcnDemo {
    fn column_1(&mut self, ui: &mut egui::Ui) {
        self.field_demo(ui);
    }

    fn column_2(&mut self, ui: &mut egui::Ui) {
        self.empty_avatar_group(ui);
        ui.add_space(16.0);
        self.spinner_badge(ui);
        ui.add_space(16.0);
        self.button_group_input_group(ui);
        ui.add_space(16.0);
        self.field_slider(ui);
        ui.add_space(16.0);
        self.input_group_demo(ui);
    }

    fn column_3(&mut self, ui: &mut egui::Ui) {
        self.input_group_button_example(ui);
        ui.add_space(16.0);
        self.item_demo(ui);
        ui.add_space(16.0);
        egui_shadcn::Separator::horizontal()
            .text("Appearance Settings")
            .show(ui);
        ui.add_space(16.0);
        self.appearance_settings(ui);
    }

    fn column_4(&mut self, ui: &mut egui::Ui) {
        self.notion_prompt_form(ui);
        ui.add_space(16.0);
        self.button_group_demo(ui);
        ui.add_space(16.0);
        self.field_checkbox(ui);
        ui.add_space(16.0);
        self.button_group_nested_and_popover(ui);
        ui.add_space(16.0);
        self.field_hear(ui);
        ui.add_space(16.0);
        self.spinner_empty(ui);
    }

    // ── Column 1: FieldDemo ──────────────────────────────────────────

    fn field_demo(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Card::new().show(ui, |ui| {
            egui_shadcn::FieldGroup::show(ui, |ui| {
                // Payment Method fieldset
                egui_shadcn::FieldSet::show(ui, "Payment Method", |ui| {
                    egui_shadcn::FieldDescription::show(
                        ui,
                        "All transactions are secure and encrypted",
                    );

                    egui_shadcn::Label::new("Name on Card").show(ui);
                    egui_shadcn::Input::new(&mut self.name)
                        .placeholder("John Doe")
                        .desired_width(ui.available_width())
                        .show(ui);

                    // Card Number + CVV row
                    ui.horizontal(|ui| {
                        let w = ui.available_width();
                        ui.vertical(|ui| {
                            ui.set_width(w * 0.63);
                            egui_shadcn::Label::new("Card Number").show(ui);
                            egui_shadcn::Input::new(&mut self.card_number)
                                .placeholder("1234 5678 9012 3456")
                                .desired_width(ui.available_width())
                                .show(ui);
                            egui_shadcn::FieldDescription::show(
                                ui,
                                "Enter your 16-digit number.",
                            );
                        });
                        ui.vertical(|ui| {
                            egui_shadcn::Label::new("CVV").show(ui);
                            egui_shadcn::Input::new(&mut self.cvv)
                                .placeholder("123")
                                .desired_width(ui.available_width())
                                .show(ui);
                        });
                    });

                    // Month + Year row
                    ui.horizontal(|ui| {
                        let months = vec![
                            "01".to_owned(),
                            "02".to_owned(),
                            "03".to_owned(),
                            "04".to_owned(),
                            "05".to_owned(),
                            "06".to_owned(),
                            "07".to_owned(),
                            "08".to_owned(),
                            "09".to_owned(),
                            "10".to_owned(),
                            "11".to_owned(),
                            "12".to_owned(),
                        ];
                        let half = (ui.available_width() - 8.0) / 2.0;
                        ui.add(
                            egui_shadcn::Select::new(&mut self.selected_month, &months)
                                .placeholder("MM")
                                .width(half),
                        );
                        let years = vec![
                            "2024".to_owned(),
                            "2025".to_owned(),
                            "2026".to_owned(),
                            "2027".to_owned(),
                            "2028".to_owned(),
                            "2029".to_owned(),
                        ];
                        ui.add(
                            egui_shadcn::Select::new(&mut self.selected_year, &years)
                                .placeholder("YYYY")
                                .width(half),
                        );
                    });
                });

                egui_shadcn::Separator::horizontal().show(ui);

                // Billing Address fieldset
                egui_shadcn::FieldSet::show(ui, "Billing Address", |ui| {
                    egui_shadcn::FieldDescription::show(
                        ui,
                        "The billing address associated with your payment method",
                    );
                    ui.add(
                        egui_shadcn::Checkbox::new(&mut self.billing_same)
                            .label("Same as shipping address"),
                    );
                });

                egui_shadcn::Separator::horizontal().show(ui);

                // Comments
                egui_shadcn::Label::new("Comments").show(ui);
                egui_shadcn::Textarea::new(&mut self.comments)
                    .placeholder("Add any additional comments")
                    .desired_width(ui.available_width())
                    .show(ui);

                // Action buttons
                ui.horizontal(|ui| {
                    egui_shadcn::Button::new("Submit").show(ui);
                    ui.add_space(8.0);
                    egui_shadcn::Button::new("Cancel")
                        .variant(egui_shadcn::ButtonVariant::Outline)
                        .show(ui);
                });
            });
        });
    }

    // ── Column 2: EmptyAvatarGroup ───────────────────────────────────

    fn empty_avatar_group(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Empty::show(ui, |ui| {
            ui.horizontal(|ui| {
                egui_shadcn::Avatar::new("CN").show(ui);
                egui_shadcn::Avatar::new("LR").show(ui);
                egui_shadcn::Avatar::new("ER").show(ui);
            });
            ui.add_space(8.0);
            egui_shadcn::Label::new("No Team Members").show(ui);
            egui_shadcn::FieldDescription::show(
                ui,
                "Invite your team to collaborate on this project.",
            );
            ui.add_space(8.0);
            egui_shadcn::Button::new("Invite Members")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .size(egui_shadcn::ComponentSize::Sm)
                .show(ui);
        });
    }

    // ── Column 2: SpinnerBadge ───────────────────────────────────────

    fn spinner_badge(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            egui_shadcn::Badge::new("Syncing")
                .variant(egui_shadcn::BadgeVariant::Default)
                .show(ui);
            egui_shadcn::Badge::new("Updating")
                .variant(egui_shadcn::BadgeVariant::Secondary)
                .show(ui);
            egui_shadcn::Badge::new("Loading")
                .variant(egui_shadcn::BadgeVariant::Outline)
                .show(ui);
        });
    }

    // ── Column 2: ButtonGroupInputGroup ──────────────────────────────

    fn button_group_input_group(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::ButtonGroup::show(ui, |ui| {
            egui_shadcn::Button::new("Day")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui);
            egui_shadcn::Button::new("Week")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui);
            egui_shadcn::Button::new("Month")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui);
        });
    }

    // ── Column 2: FieldSlider ────────────────────────────────────────

    fn field_slider(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Label::new("Price Range").show(ui);
        egui_shadcn::FieldDescription::show(
            ui,
            &format!("Set your budget range (${:.0}).", self.slider_low),
        );
        ui.add_space(8.0);
        egui_shadcn::Slider::new(&mut self.slider_low, 0.0..=1000.0)
            .step(10.0)
            .width(ui.available_width())
            .show(ui);
    }

    // ── Column 2: InputGroupDemo ─────────────────────────────────────

    fn input_group_demo(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::InputGroup::show(
            ui,
            &mut self.search_text,
            "Search...",
            Some("\u{1F50D}"),
            None::<fn(&mut egui::Ui)>,
        );
        ui.add_space(8.0);
        egui_shadcn::InputGroup::show(
            ui,
            &mut self.url_text,
            "example.com",
            Some("https://"),
            None::<fn(&mut egui::Ui)>,
        );
    }

    // ── Column 3: InputGroupButtonExample ────────────────────────────

    fn input_group_button_example(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::InputGroup::show(
            ui,
            &mut self.prefix_text,
            "Enter value...",
            Some("$"),
            Some(|ui: &mut egui::Ui| {
                egui_shadcn::Button::new("Go")
                    .size(egui_shadcn::ComponentSize::Xs)
                    .show(ui);
            }),
        );
    }

    // ── Column 3: ItemDemo ───────────────────────────────────────────

    fn item_demo(&self, ui: &mut egui::Ui) {
        egui_shadcn::Item::new()
            .variant(egui_shadcn::ItemVariant::Outline)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        egui_shadcn::Label::new("Two-factor authentication").show(ui);
                        egui_shadcn::FieldDescription::show(
                            ui,
                            "Verify via email or phone number.",
                        );
                    });
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            egui_shadcn::Button::new("Enable")
                                .variant(egui_shadcn::ButtonVariant::Outline)
                                .size(egui_shadcn::ComponentSize::Sm)
                                .show(ui);
                        },
                    );
                });
            });

        ui.add_space(8.0);

        egui_shadcn::Item::new()
            .variant(egui_shadcn::ItemVariant::Outline)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let theme =
                        egui_shadcn::ShadcnThemeExt::shadcn_theme(ui.ctx());
                    let galley = ui.painter().layout_no_wrap(
                        "\u{2713}".to_owned(),
                        egui::FontId::proportional(14.0),
                        theme.primary,
                    );
                    let (r, _) =
                        ui.allocate_exact_size(galley.size(), egui::Sense::hover());
                    ui.painter().galley(r.min, galley, theme.primary);
                    egui_shadcn::Label::new("Your profile has been verified.").show(ui);
                });
            });
    }

    // ── Column 3: AppearanceSettings ─────────────────────────────────

    fn appearance_settings(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Card::new().show(ui, |ui| {
            egui_shadcn::FieldGroup::show(ui, |ui| {
                // Infrastructure
                egui_shadcn::FieldSet::show(ui, "Compute Environment", |ui| {
                    egui_shadcn::FieldDescription::show(
                        ui,
                        "Select the compute environment for your cluster.",
                    );
                    ui.add_space(4.0);
                    let options =
                        ["Kubernetes".to_owned(), "Virtual Machine".to_owned()];
                    ui.add(egui_shadcn::RadioGroup::new(
                        &mut self.infra_choice,
                        &options,
                    ));
                });

                egui_shadcn::Separator::horizontal().show(ui);

                // GPU Count
                egui_shadcn::Label::new("Number of GPUs").show(ui);
                egui_shadcn::FieldDescription::show(
                    ui,
                    "You can add more later.",
                );
                ui.add_space(4.0);
                egui_shadcn::Input::new(&mut self.gpu_count)
                    .desired_width(60.0)
                    .show(ui);

                egui_shadcn::Separator::horizontal().show(ui);

                // Wallpaper Tinting
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        egui_shadcn::Label::new("Wallpaper Tinting").show(ui);
                        egui_shadcn::FieldDescription::show(
                            ui,
                            "Allow the wallpaper to be tinted.",
                        );
                    });
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.add(egui_shadcn::Switch::new(
                                &mut self.wallpaper_tinting,
                            ));
                        },
                    );
                });
            });
        });
    }

    // ── Column 4: NotionPromptForm ───────────────────────────────────

    fn notion_prompt_form(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Card::new().show(ui, |ui| {
            egui_shadcn::Textarea::new(&mut self.prompt_text)
                .placeholder("Ask, search, or make anything...")
                .desired_width(ui.available_width())
                .min_height(80.0)
                .show(ui);
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                egui_shadcn::Button::new("Send").show(ui);
                ui.add_space(4.0);
                egui_shadcn::Button::new("Clear")
                    .variant(egui_shadcn::ButtonVariant::Ghost)
                    .show(ui);
            });
        });
    }

    // ── Column 4: ButtonGroupDemo ────────────────────────────────────

    fn button_group_demo(&self, ui: &mut egui::Ui) {
        egui_shadcn::ButtonGroup::show(ui, |ui| {
            egui_shadcn::Button::new("Archive")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui);
            egui_shadcn::Button::new("Report")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui);
            egui_shadcn::Button::new("Snooze")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .show(ui);
        });
    }

    // ── Column 4: FieldCheckbox ──────────────────────────────────────

    fn field_checkbox(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui_shadcn::Checkbox::new(&mut self.terms_accepted)
                .label("I agree to the terms and conditions"),
        );
    }

    // ── Column 4: ButtonGroupNested + ButtonGroupPopover side-by-side

    fn button_group_nested_and_popover(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            egui_shadcn::ButtonGroup::show(ui, |ui| {
                egui_shadcn::Button::new("1")
                    .variant(egui_shadcn::ButtonVariant::Outline)
                    .size(egui_shadcn::ComponentSize::Sm)
                    .show(ui);
                egui_shadcn::Button::new("2")
                    .variant(egui_shadcn::ButtonVariant::Outline)
                    .size(egui_shadcn::ComponentSize::Sm)
                    .show(ui);
                egui_shadcn::Button::new("3")
                    .variant(egui_shadcn::ButtonVariant::Outline)
                    .size(egui_shadcn::ComponentSize::Sm)
                    .show(ui);
            });
            ui.add_space(8.0);
            egui_shadcn::ButtonGroup::show(ui, |ui| {
                egui_shadcn::Button::new("Copilot")
                    .variant(egui_shadcn::ButtonVariant::Outline)
                    .size(egui_shadcn::ComponentSize::Sm)
                    .show(ui);
                egui_shadcn::Button::new("\u{25BE}")
                    .variant(egui_shadcn::ButtonVariant::Outline)
                    .size(egui_shadcn::ComponentSize::Sm)
                    .show(ui);
            });
        });
    }

    // ── Column 4: FieldHear ──────────────────────────────────────────

    fn field_hear(&mut self, ui: &mut egui::Ui) {
        egui_shadcn::Card::new().show(ui, |ui| {
            egui_shadcn::FieldLegend::show(ui, "How did you hear about us?");
            egui_shadcn::FieldDescription::show(
                ui,
                "Select the option that best describes how you heard about us.",
            );
            ui.add_space(8.0);
            ui.add(
                egui_shadcn::Checkbox::new(&mut self.hear_social)
                    .label("Social Media"),
            );
            ui.add(
                egui_shadcn::Checkbox::new(&mut self.hear_search)
                    .label("Search Engine"),
            );
            ui.add(
                egui_shadcn::Checkbox::new(&mut self.hear_referral)
                    .label("Referral"),
            );
            ui.add(
                egui_shadcn::Checkbox::new(&mut self.hear_other).label("Other"),
            );
        });
    }

    // ── Column 4: SpinnerEmpty ───────────────────────────────────────

    fn spinner_empty(&self, ui: &mut egui::Ui) {
        egui_shadcn::Empty::show(ui, |ui| {
            egui_shadcn::Spinner::new().size(24.0).show(ui);
            ui.add_space(8.0);
            egui_shadcn::Label::new("Processing your request").show(ui);
            egui_shadcn::FieldDescription::show(
                ui,
                "Please wait while we process your request.",
            );
            ui.add_space(8.0);
            egui_shadcn::Button::new("Cancel")
                .variant(egui_shadcn::ButtonVariant::Outline)
                .size(egui_shadcn::ComponentSize::Sm)
                .show(ui);
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 900.0]),
        ..Default::default()
    };
    eframe::run_native(
        "shadcn/ui Demo \u{2014} egui-shadcn",
        options,
        Box::new(|_cc| Ok(Box::new(ShadcnDemo::default()))),
    )
}
