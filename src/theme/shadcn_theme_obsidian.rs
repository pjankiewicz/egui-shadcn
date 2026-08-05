//! Obsidian theme constructors (light and dark) featuring pitch volcanic glass & silver platinum tokens.
//!
//! Attribution & Contribution:
//! - Design System Tokens: shadcn/ui (https://ui.shadcn.com) by shadcn (MIT License).
//! - Color Palette: Tailwind CSS Neutral & Slate scales (https://tailwindcss.com) by Tailwind Labs (MIT License).

/// Creates a dark theme based on pitch volcanic obsidian black with metallic silver-white accents.
pub fn obsidian_dark() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(5, 5, 5),                 // True volcanic obsidian black (#050505)
        foreground: egui::Color32::from_rgb(245, 245, 245),           // Pure silver white (#f5f5f5)
        card: egui::Color32::from_rgb(14, 14, 14),                    // Polished jet slate (#0e0e0e)
        card_foreground: egui::Color32::from_rgb(245, 245, 245),
        popover: egui::Color32::from_rgb(14, 14, 14),
        popover_foreground: egui::Color32::from_rgb(245, 245, 245),
        primary: egui::Color32::from_rgb(226, 232, 240),               // Burnished silver platinum (#e2e8f0)
        primary_foreground: egui::Color32::from_rgb(10, 10, 10),       // Obsidian core black
        secondary: egui::Color32::from_rgb(28, 28, 30),               // Dark charcoal (#1c1c1e)
        secondary_foreground: egui::Color32::from_rgb(245, 245, 245),
        muted: egui::Color32::from_rgb(28, 28, 30),
        muted_foreground: egui::Color32::from_rgb(140, 140, 145),     // Ash gray (#8c8c91)
        accent: egui::Color32::from_rgb(40, 40, 44),                  // Metallic charcoal (#28282c)
        accent_foreground: egui::Color32::from_rgb(255, 255, 255),
        destructive: egui::Color32::from_rgb(220, 38, 38),           // Red-600 (#dc2626)
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(34, 34, 38),               // Subtle obsidian edge (#222226)
        input: egui::Color32::from_rgb(34, 34, 38),
        ring: egui::Color32::from_rgb(160, 160, 168),                 // Platinum halo ring (#a0a0a8)
        radius: 4.0,
    }
}

/// Alias for `obsidian_dark()`.
pub fn obsidian() -> super::shadcn_theme::ShadcnTheme {
    obsidian_dark()
}

/// Creates a soft eye-friendly light theme based on smoky quartz grey with sharp obsidian accents.
pub fn obsidian_light() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(235, 235, 238),           // Smoky quartz grey (#ebebee)
        foreground: egui::Color32::from_rgb(14, 14, 18),              // Obsidian core
        card: egui::Color32::from_rgb(242, 242, 246),                 // Muted quartz card
        card_foreground: egui::Color32::from_rgb(14, 14, 18),
        popover: egui::Color32::from_rgb(242, 242, 246),
        popover_foreground: egui::Color32::from_rgb(14, 14, 18),
        primary: egui::Color32::from_rgb(24, 24, 28),                 // Deep pitch obsidian (#18181c)
        primary_foreground: egui::Color32::from_rgb(255, 255, 255),   // Pure white
        secondary: egui::Color32::from_rgb(222, 222, 228),
        secondary_foreground: egui::Color32::from_rgb(24, 24, 28),
        muted: egui::Color32::from_rgb(225, 225, 230),
        muted_foreground: egui::Color32::from_rgb(90, 90, 100),
        accent: egui::Color32::from_rgb(215, 215, 222),
        accent_foreground: egui::Color32::from_rgb(14, 14, 18),
        destructive: egui::Color32::from_rgb(220, 38, 38),
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(202, 202, 210),
        input: egui::Color32::from_rgb(202, 202, 210),
        ring: egui::Color32::from_rgb(24, 24, 28),
        radius: 4.0,
    }
}
