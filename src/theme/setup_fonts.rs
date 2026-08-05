//! Loads the Geist font family as the default proportional font.

/// Configures egui to use the Geist font for proportional text.
///
/// Call this once during app setup, e.g. in the `CreationContext` callback:
///
/// ```no_run
/// # struct MyApp;
/// # impl eframe::App for MyApp {
/// #     fn ui(&mut self, _: &mut egui::Ui, _: &mut eframe::Frame) {}
/// # }
/// eframe::run_native("app", Default::default(), Box::new(|cc| {
///     egui_shadcn::setup_fonts(&cc.egui_ctx);
///     Ok(Box::new(MyApp))
/// }));
/// ```
pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Geist-Regular".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/Geist-Regular.ttf"
        ))),
    );

    fonts.font_data.insert(
        "Geist-Bold".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/Geist-Bold.ttf"
        ))),
    );

    // Insert Geist as the first proportional font (highest priority)
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Geist-Regular".to_owned());

    // Add bold variant fallback
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(1, "Geist-Bold".to_owned());

    // Register explicit bold font family for strong/bold text
    fonts
        .families
        .insert(egui::FontFamily::Name("bold".into()), vec!["Geist-Bold".to_owned()]);

    ctx.set_fonts(fonts);
}
