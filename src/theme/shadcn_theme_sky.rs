//! Sky / Light Blue theme constructors (light and dark) featuring cerulean & ocean slate tones.
//!
//! Attribution & Contribution:
//! - Design System Tokens: shadcn/ui (https://ui.shadcn.com) by shadcn (MIT License).
//! - Color Palette: Tailwind CSS Sky & Slate color scales (https://tailwindcss.com) by Tailwind Labs (MIT License).

/// Creates a dark theme based on a deep ocean slate background with vibrant sky-blue accents.
pub fn sky_dark() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(11, 17, 26),                // Deep ocean slate (#0b111a)
        foreground: egui::Color32::from_rgb(240, 246, 252),           // Crisp ice white (#f0f6fc)
        card: egui::Color32::from_rgb(19, 27, 41),                    // Sleek slate navy card (#131b29)
        card_foreground: egui::Color32::from_rgb(240, 246, 252),
        popover: egui::Color32::from_rgb(19, 27, 41),
        popover_foreground: egui::Color32::from_rgb(240, 246, 252),
        primary: egui::Color32::from_rgb(14, 165, 233),               // Electric sky blue (sky-500: #0ea5e9)
        primary_foreground: egui::Color32::from_rgb(255, 255, 255),   // Pure white
        secondary: egui::Color32::from_rgb(30, 41, 59),               // Slate-800 (#1e293b)
        secondary_foreground: egui::Color32::from_rgb(240, 246, 252),
        muted: egui::Color32::from_rgb(30, 41, 59),                   // Slate-800
        muted_foreground: egui::Color32::from_rgb(148, 163, 184),     // Slate-400 (#94a3b8)
        accent: egui::Color32::from_rgb(30, 58, 86),                  // Subtle icy ocean accent tint (#1e3a56)
        accent_foreground: egui::Color32::from_rgb(224, 242, 254),    // Sky-100 (#e0f2fe)
        destructive: egui::Color32::from_rgb(244, 63, 94),           // Rose-500 (#f43f5e)
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(30, 45, 66),               // Deep blue slate border (#1e2d42)
        input: egui::Color32::from_rgb(30, 45, 66),                // Deep blue slate input border
        ring: egui::Color32::from_rgb(56, 189, 248),                 // Glowing cyan ring (sky-400: #38bdf8)
        radius: 8.0,
    }
}

/// Alias for `sky_dark()`.
pub fn sky() -> super::shadcn_theme::ShadcnTheme {
    sky_dark()
}

/// Creates a soft eye-friendly light theme based on an icy slate mist background with cerulean elements.
pub fn sky_light() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(232, 238, 244),           // Soft ice slate mist (#e8eef4)
        foreground: egui::Color32::from_rgb(15, 23, 42),              // Deep slate-900 (#0f172a)
        card: egui::Color32::from_rgb(240, 245, 250),                 // Muted ice card
        card_foreground: egui::Color32::from_rgb(15, 23, 42),
        popover: egui::Color32::from_rgb(240, 245, 250),
        popover_foreground: egui::Color32::from_rgb(15, 23, 42),
        primary: egui::Color32::from_rgb(2, 132, 199),               // Rich cerulean (sky-600: #0284c7)
        primary_foreground: egui::Color32::from_rgb(255, 255, 255),   // Pure white
        secondary: egui::Color32::from_rgb(215, 228, 240),            // Muted sky blue tint
        secondary_foreground: egui::Color32::from_rgb(3, 105, 161),    // Deep sky blue text
        muted: egui::Color32::from_rgb(222, 232, 242),
        muted_foreground: egui::Color32::from_rgb(80, 100, 125),
        accent: egui::Color32::from_rgb(208, 224, 238),
        accent_foreground: egui::Color32::from_rgb(3, 105, 161),
        destructive: egui::Color32::from_rgb(225, 29, 72),
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(198, 214, 230),               // Soft border
        input: egui::Color32::from_rgb(198, 214, 230),
        ring: egui::Color32::from_rgb(14, 165, 233),
        radius: 8.0,
    }
}
