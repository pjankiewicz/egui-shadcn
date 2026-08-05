//! Violet theme constructors (light and dark) matching shadcn/ui violet/zinc theme tokens.
//!
//! Attribution & Contribution:
//! - Design System & Tokens: shadcn/ui (https://ui.shadcn.com) by shadcn (MIT License).
//! - Color Palette: Tailwind CSS Zinc & Violet color scales (https://tailwindcss.com) by Tailwind Labs (MIT License).

/// Creates a dark theme with a violet primary color palette (matching shadcn/ui violet / zinc dark theme).
pub fn violet_dark() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(9, 9, 11),                // zinc-950: hsl(240 10% 3.9%)
        foreground: egui::Color32::from_rgb(250, 250, 250),           // zinc-50: hsl(0 0% 98%)
        card: egui::Color32::from_rgb(24, 24, 27),                    // zinc-900: hsl(240 5.9% 10%)
        card_foreground: egui::Color32::from_rgb(250, 250, 250),      // zinc-50
        popover: egui::Color32::from_rgb(24, 24, 27),                 // zinc-900
        popover_foreground: egui::Color32::from_rgb(250, 250, 250),   // zinc-50
        primary: egui::Color32::from_rgb(124, 58, 237),               // violet-600: hsl(263.4 70% 50.4%)
        primary_foreground: egui::Color32::from_rgb(248, 250, 252),   // slate-50: hsl(210 20% 98%)
        secondary: egui::Color32::from_rgb(39, 39, 42),               // zinc-800: hsl(240 3.7% 15.9%)
        secondary_foreground: egui::Color32::from_rgb(250, 250, 250), // zinc-50
        muted: egui::Color32::from_rgb(39, 39, 42),                   // zinc-800: hsl(240 3.7% 15.9%)
        muted_foreground: egui::Color32::from_rgb(161, 161, 170),     // zinc-400: hsl(240 5% 64.9%)
        accent: egui::Color32::from_rgb(39, 39, 42),                  // zinc-800: hsl(240 3.7% 15.9%)
        accent_foreground: egui::Color32::from_rgb(250, 250, 250),    // zinc-50
        destructive: egui::Color32::from_rgb(239, 68, 68),           // red-500
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(39, 39, 42),               // zinc-800: hsl(240 3.7% 15.9%)
        input: egui::Color32::from_rgb(39, 39, 42),                // zinc-800: hsl(240 3.7% 15.9%)
        ring: egui::Color32::from_rgb(167, 139, 250),                 // violet-400: hsl(263 70% 75%)
        radius: 6.0,
    }
}

/// Alias for `violet_dark()`.
pub fn violet() -> super::shadcn_theme::ShadcnTheme {
    violet_dark()
}

/// Creates a soft eye-friendly light theme with a violet primary color palette.
pub fn violet_light() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(235, 235, 242),           // Soft lavender grey (#ebebf2)
        foreground: egui::Color32::from_rgb(20, 20, 28),              // Dark slate text
        card: egui::Color32::from_rgb(243, 243, 248),                 // Muted card
        card_foreground: egui::Color32::from_rgb(20, 20, 28),
        popover: egui::Color32::from_rgb(243, 243, 248),
        popover_foreground: egui::Color32::from_rgb(20, 20, 28),
        primary: egui::Color32::from_rgb(124, 58, 237),               // Violet-600
        primary_foreground: egui::Color32::from_rgb(255, 255, 255),
        secondary: egui::Color32::from_rgb(222, 222, 232),            // Muted lavender tint
        secondary_foreground: egui::Color32::from_rgb(24, 24, 34),
        muted: egui::Color32::from_rgb(222, 222, 232),
        muted_foreground: egui::Color32::from_rgb(95, 95, 110),
        accent: egui::Color32::from_rgb(214, 214, 228),
        accent_foreground: egui::Color32::from_rgb(24, 24, 34),
        destructive: egui::Color32::from_rgb(225, 29, 72),
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(205, 205, 218),               // Soft border
        input: egui::Color32::from_rgb(205, 205, 218),
        ring: egui::Color32::from_rgb(124, 58, 237),
        radius: 6.0,
    }
}
