//! Demo app showcasing all egui-shadcn widgets.

struct DemoApp {
    dark_mode: bool,
    checkbox_val: bool,
    switch_val: bool,
    radio_a: bool,
    radio_b: bool,
    radio_c: bool,
    toggle_bold: bool,
    toggle_italic: bool,
    slider_val: f64,
    input_text: String,
    selected_fruit: Option<String>,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            dark_mode: false,
            checkbox_val: false,
            switch_val: false,
            radio_a: true,
            radio_b: false,
            radio_c: false,
            toggle_bold: false,
            toggle_italic: false,
            slider_val: 50.0,
            input_text: String::new(),
            selected_fruit: None,
        }
    }
}

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Apply theme
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
                    ui.horizontal(|ui| {
                        ui.add_space(24.0);
                        ui.vertical(|ui| {
                            self.draw_content(ui);
                        });
                    });
                });
            });
    }
}

impl DemoApp {
    fn draw_content(&mut self, ui: &mut egui::Ui) {
        ui.heading("egui-shadcn Demo");
        ui.add_space(16.0);

        // Theme toggle
        ui.horizontal(|ui| {
            ui.label("Dark mode:");
            ui.add(egui_shadcn::Switch::new(&mut self.dark_mode));
        });

        ui.add_space(24.0);

        // --- Buttons ---
        ui.label("Buttons");
        ui.add_space(8.0);
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

        ui.add_space(8.0);
        ui.label("Button Sizes");
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

        ui.add_space(24.0);

        // --- Checkbox ---
        ui.label("Checkbox");
        ui.add_space(8.0);
        ui.add(egui_shadcn::Checkbox::new(&mut self.checkbox_val).label("Accept terms"));

        ui.add_space(24.0);

        // --- Switch ---
        ui.label("Switch");
        ui.add_space(8.0);
        ui.add(egui_shadcn::Switch::new(&mut self.switch_val).label("Airplane mode"));

        ui.add_space(24.0);

        // --- Radio ---
        ui.label("Radio");
        ui.add_space(8.0);
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

        ui.add_space(24.0);

        // --- Toggle ---
        ui.label("Toggle");
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.add(egui_shadcn::Toggle::new(&mut self.toggle_bold, "B"));
            ui.add(
                egui_shadcn::Toggle::new(&mut self.toggle_italic, "I")
                    .variant(egui_shadcn::ToggleVariant::Outline),
            );
        });

        ui.add_space(24.0);

        // --- Slider ---
        ui.label("Slider");
        ui.add_space(8.0);
        ui.add(egui_shadcn::Slider::new(&mut self.slider_val, 0.0..=100.0));
        ui.label(format!("Value: {:.0}", self.slider_val));

        ui.add_space(24.0);

        // --- Input ---
        ui.label("Input");
        ui.add_space(8.0);
        ui.add(egui_shadcn::Input::new(&mut self.input_text).placeholder("Type something..."));

        ui.add_space(24.0);

        // --- Select ---
        ui.label("Select");
        ui.add_space(8.0);
        let fruits = vec![
            "Apple".to_owned(),
            "Banana".to_owned(),
            "Cherry".to_owned(),
            "Grape".to_owned(),
        ];
        ui.add(
            egui_shadcn::Select::new(&mut self.selected_fruit, &fruits)
                .placeholder("Pick a fruit..."),
        );
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "egui-shadcn demo",
        options,
        Box::new(|_cc| Ok(Box::new(DemoApp::default()))),
    )
}
