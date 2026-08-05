//! Theme Gallery Demo showcasing all available egui-shadcn themes (Default, Violet, Sky, Obsidian, Pink, Terracotta, Rainbow, Heatmap, Cyber Aurora, Nostalgia).

use egui_shadcn::{
    Alert, AlertVariant, Badge, BadgeVariant, Button, ButtonVariant, Card, Checkbox, ComponentSize,
    Input, Progress, Radio, SelectValue, ShadcnTheme, ShadcnThemeExt, Slider, Switch, Tabs, Toggle,
    ToggleVariant, Typography,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectedPalette {
    Default,
    Violet,
    Sky,
    Obsidian,
    Pink,
    Terracotta,
    Rainbow,
    Heatmap,
    #[default]
    CyberAurora,
    Nostalgia,
}

impl std::fmt::Display for SelectedPalette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl SelectedPalette {
    pub fn all() -> &'static [SelectedPalette] {
        &[
            SelectedPalette::Default,
            SelectedPalette::Violet,
            SelectedPalette::Sky,
            SelectedPalette::Obsidian,
            SelectedPalette::Pink,
            SelectedPalette::Terracotta,
            SelectedPalette::Rainbow,
            SelectedPalette::Heatmap,
            SelectedPalette::CyberAurora,
            SelectedPalette::Nostalgia,
        ]
    }

    pub fn name(self) -> &'static str {
        match self {
            SelectedPalette::Default => "Nova (Default)",
            SelectedPalette::Violet => "Violet / Zinc",
            SelectedPalette::Sky => "Sky Blue / Cerulean",
            SelectedPalette::Obsidian => "Obsidian / Jet Glass",
            SelectedPalette::Pink => "Hot Pink / Fuchsia",
            SelectedPalette::Terracotta => "Terracotta (Brown-Red)",
            SelectedPalette::Rainbow => "Rainbow Spectrum",
            SelectedPalette::Heatmap => "Heatmap Thermal",
            SelectedPalette::CyberAurora => "Cyber Aurora (Ultra Creative)",
            SelectedPalette::Nostalgia => "Childhood Nostalgia",
        }
    }

    pub fn to_shadcn_theme(self, dark: bool) -> ShadcnTheme {
        match (self, dark) {
            (SelectedPalette::Default, false) => egui_shadcn::theme::shadcn_theme_light::light(),
            (SelectedPalette::Default, true) => egui_shadcn::theme::shadcn_theme_dark::dark(),
            (SelectedPalette::Violet, false) => egui_shadcn::theme::shadcn_theme_violet::violet_light(),
            (SelectedPalette::Violet, true) => egui_shadcn::theme::shadcn_theme_violet::violet_dark(),
            (SelectedPalette::Sky, false) => egui_shadcn::theme::shadcn_theme_sky::sky_light(),
            (SelectedPalette::Sky, true) => egui_shadcn::theme::shadcn_theme_sky::sky_dark(),
            (SelectedPalette::Obsidian, false) => egui_shadcn::theme::shadcn_theme_obsidian::obsidian_light(),
            (SelectedPalette::Obsidian, true) => egui_shadcn::theme::shadcn_theme_obsidian::obsidian_dark(),
            (SelectedPalette::Pink, false) => egui_shadcn::theme::shadcn_theme_pink::pink_light(),
            (SelectedPalette::Pink, true) => egui_shadcn::theme::shadcn_theme_pink::pink_dark(),
            (SelectedPalette::Terracotta, false) => egui_shadcn::theme::shadcn_theme_terracotta::terracotta_light(),
            (SelectedPalette::Terracotta, true) => egui_shadcn::theme::shadcn_theme_terracotta::terracotta_dark(),
            (SelectedPalette::Rainbow, false) => egui_shadcn::theme::shadcn_theme_rainbow::rainbow_light(),
            (SelectedPalette::Rainbow, true) => egui_shadcn::theme::shadcn_theme_rainbow::rainbow_dark(),
            (SelectedPalette::Heatmap, false) => egui_shadcn::theme::shadcn_theme_heatmap::heatmap_light(),
            (SelectedPalette::Heatmap, true) => egui_shadcn::theme::shadcn_theme_heatmap::heatmap_dark(),
            (SelectedPalette::CyberAurora, false) => egui_shadcn::theme::shadcn_theme_cyber_aurora::cyber_aurora_light(),
            (SelectedPalette::CyberAurora, true) => egui_shadcn::theme::shadcn_theme_cyber_aurora::cyber_aurora_dark(),
            (SelectedPalette::Nostalgia, false) => egui_shadcn::theme::shadcn_theme_nostalgia::nostalgia_light(),
            (SelectedPalette::Nostalgia, true) => egui_shadcn::theme::shadcn_theme_nostalgia::nostalgia_dark(),
        }
    }
}

struct ThemeGalleryApp {
    selected_palette: SelectedPalette,
    dark_mode: bool,
    checkbox_val: bool,
    switch_val: bool,
    radio_idx: usize,
    slider_val: f64,
    progress_val: f32,
    input_text: String,
    toggle_bold: bool,
    toggle_italic: bool,
    active_tab: usize,
}

impl Default for ThemeGalleryApp {
    fn default() -> Self {
        Self {
            selected_palette: SelectedPalette::CyberAurora,
            dark_mode: true,
            checkbox_val: true,
            switch_val: true,
            radio_idx: 0,
            slider_val: 72.0,
            progress_val: 0.65,
            input_text: "egui-shadcn theme gallery".to_owned(),
            toggle_bold: true,
            toggle_italic: false,
            active_tab: 0,
        }
    }
}

impl eframe::App for ThemeGalleryApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let theme = self.selected_palette.to_shadcn_theme(self.dark_mode);
        ui.ctx().set_shadcn_theme(theme.clone());

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme.background))
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.add_space(24.0);
                        ui.vertical(|ui| {
                            self.draw_top_bar(ui, &theme);
                            ui.add_space(20.0);
                            self.draw_color_token_preview(ui, &theme);
                            ui.add_space(20.0);
                            self.draw_widget_samples(ui, &theme);
                            ui.add_space(32.0);
                        });
                        ui.add_space(24.0);
                    });
                });
            });
    }
}

impl ThemeGalleryApp {
    fn draw_top_bar(&mut self, ui: &mut egui::Ui, theme: &ShadcnTheme) {
        ui.horizontal(|ui| {
            ui.heading(
                egui::RichText::new("🎨 egui-shadcn Theme Gallery")
                    .size(24.0)
                    .color(theme.foreground),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(Switch::new(&mut self.dark_mode).label("Dark Mode"))
                    .changed()
                {
                    ui.ctx().request_repaint();
                }

                ui.add_space(16.0);

                ui.add(
                    SelectValue::new(&mut self.selected_palette, SelectedPalette::all())
                        .width(260.0),
                );


                ui.label(egui::RichText::new("Theme Palette: ").color(theme.muted_foreground));
            });
        });
    }

    fn draw_color_token_preview(&self, ui: &mut egui::Ui, theme: &ShadcnTheme) {
        Card::new().show(ui, |ui| {
            ui.heading(
                egui::RichText::new("Active Palette Color Tokens")
                    .size(16.0)
                    .color(theme.foreground),
            );
            Typography::muted(format!(
                "Currently inspecting {} ({}) — Radius: {:.1}px",
                self.selected_palette.name(),
                if self.dark_mode { "Dark Mode" } else { "Light Mode" },
                theme.radius
            ))
            .show(ui);

            ui.add_space(12.0);
            let tokens = [
                ("Background", theme.background),
                ("Foreground", theme.foreground),
                ("Card", theme.card),
                ("Primary", theme.primary),
                ("Primary Text", theme.primary_foreground),
                ("Secondary", theme.secondary),
                ("Muted", theme.muted),
                ("Muted Text", theme.muted_foreground),
                ("Accent", theme.accent),
                ("Destructive", theme.destructive),
                ("Border", theme.border),
                ("Ring", theme.ring),
            ];

            ui.horizontal_wrapped(|ui| {
                for (label, color) in tokens {
                    ui.vertical(|ui| {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(72.0, 36.0),
                            egui::Sense::hover(),
                        );
                        ui.painter().rect(
                            rect,
                            theme.radius.min(6.0),
                            color,
                            egui::Stroke::new(1.0, theme.border),
                            egui::StrokeKind::Outside,
                        );

                        ui.label(
                            egui::RichText::new(label)
                                .size(10.0)
                                .color(theme.muted_foreground),
                        );
                    });
                    ui.add_space(8.0);
                }
            });
        });
    }

    fn draw_widget_samples(&mut self, ui: &mut egui::Ui, theme: &ShadcnTheme) {
        ui.horizontal(|ui| {
            // Left Column
            ui.vertical(|ui| {
                ui.set_width(390.0);

                // Buttons
                Card::new().show(ui, |ui| {
                    ui.heading(egui::RichText::new("Button Variants & Sizes").size(15.0).color(theme.foreground));
                    ui.add_space(8.0);

                    ui.horizontal_wrapped(|ui| {
                        Button::new("Primary").show(ui);
                        Button::new("Secondary")
                            .variant(ButtonVariant::Secondary)
                            .show(ui);
                        Button::new("Outline")
                            .variant(ButtonVariant::Outline)
                            .show(ui);
                        Button::new("Destructive")
                            .variant(ButtonVariant::Destructive)
                            .show(ui);
                        Button::new("Ghost")
                            .variant(ButtonVariant::Ghost)
                            .show(ui);
                        Button::new("Link")
                            .variant(ButtonVariant::Link)
                            .show(ui);
                    });

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        Button::new("Small")
                            .size(ComponentSize::Sm)
                            .show(ui);
                        Button::new("Default").show(ui);
                        Button::new("Large")
                            .size(ComponentSize::Lg)
                            .show(ui);
                    });
                });

                ui.add_space(16.0);

                // Badges & Alerts
                Card::new().show(ui, |ui| {
                    ui.heading(egui::RichText::new("Badges & Alerts").size(15.0).color(theme.foreground));
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        Badge::new("Default").show(ui);
                        Badge::new("Secondary")
                            .variant(BadgeVariant::Secondary)
                            .show(ui);
                        Badge::new("Outline")
                            .variant(BadgeVariant::Outline)
                            .show(ui);
                        Badge::new("Destructive")
                            .variant(BadgeVariant::Destructive)
                            .show(ui);
                    });

                    ui.add_space(12.0);
                    Alert::new()
                        .title("Theme Activated")
                        .show(ui, |ui| {
                            ui.label(format!("Showing palette {}", self.selected_palette.name()));
                        });

                    ui.add_space(8.0);
                    Alert::new()
                        .title("Destructive Alert State")
                        .variant(AlertVariant::Destructive)
                        .show(ui, |ui| {
                            ui.label("This alert highlights error and critical warning states.");
                        });
                });
            });

            ui.add_space(20.0);

            // Right Column
            ui.vertical(|ui| {
                ui.set_width(390.0);

                // Form Inputs & Controls
                Card::new().show(ui, |ui| {
                    ui.heading(egui::RichText::new("Form Controls & Inputs").size(15.0).color(theme.foreground));
                    ui.add_space(8.0);

                    ui.add(Input::new(&mut self.input_text).placeholder("Enter text..."));
                    ui.add_space(12.0);

                    ui.add(Checkbox::new(&mut self.checkbox_val).label("Enable option"));
                    ui.add_space(8.0);

                    ui.add(Switch::new(&mut self.switch_val).label("Toggle state switch"));
                    ui.add_space(12.0);

                    Typography::muted("Radio selection group:").show(ui);
                    ui.add_space(4.0);
                    let radios = ["Option Alpha", "Option Beta", "Option Gamma"];
                    for (idx, label) in radios.iter().enumerate() {
                        let mut selected = self.radio_idx == idx;
                        if ui.add(Radio::new(&mut selected).label(*label)).clicked() {
                            self.radio_idx = idx;
                        }
                    }

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.add(Toggle::new(
                            &mut self.toggle_bold,
                            egui::RichText::new("Bold").font(egui::FontId::new(
                                14.0,
                                egui::FontFamily::Name("bold".into()),
                            )),
                        ));

                        ui.add(
                            Toggle::new(
                                &mut self.toggle_italic,
                                egui::RichText::new("Italic").italics(),
                            )
                            .variant(ToggleVariant::Outline),
                        );


                    });
                });

                ui.add_space(16.0);

                // Sliders & Progress & Tabs
                Card::new().show(ui, |ui| {
                    ui.heading(egui::RichText::new("Sliders, Progress & Tabs").size(15.0).color(theme.foreground));
                    ui.add_space(8.0);

                    ui.add(Slider::new(&mut self.slider_val, 0.0..=100.0));
                    Typography::muted(format!("Slider value: {:.0}%", self.slider_val)).show(ui);

                    ui.add_space(12.0);
                    ui.add(Progress::new(self.progress_val));
                    Typography::muted(format!("Progress status: {:.0}%", self.progress_val * 100.0)).show(ui);

                    ui.add_space(16.0);
                    Tabs::new(vec![
                        "Account".to_owned(),
                        "Security".to_owned(),
                        "Appearance".to_owned(),
                    ])
                    .show(ui, &mut self.active_tab, |ui, idx| match idx {
                        0 => {
                            ui.label("Account preferences and profile details.");
                        }
                        1 => {
                            ui.label("Security keys and authentication settings.");
                        }
                        _ => {
                            ui.label("Appearance customization & theme choices.");
                        }
                    });
                });
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([880.0, 920.0])
            .with_title("egui-shadcn Theme Gallery"),
        ..Default::default()
    };
    eframe::run_native(
        "egui-shadcn Theme Gallery",
        options,
        Box::new(|cc| {
            egui_shadcn::setup_fonts(&cc.egui_ctx);
            Ok(Box::new(ThemeGalleryApp::default()))
        }),
    )
}

