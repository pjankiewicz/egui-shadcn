//! Rainbow / Spectrum theme constructors (light and dark) featuring balanced chromatic prism tokens.
//!
//! Attribution & Contribution:
//! - Design System Tokens: shadcn/ui (https://ui.shadcn.com) by shadcn (MIT License).
//! - Color Palette: Tailwind CSS Purple, Cyan, Emerald, Fuchsia & Rose scales (https://tailwindcss.com) by Tailwind Labs (MIT License).

/// Creates a dark theme based on prismatic deep indigo space with multi-hued spectral accents.
pub fn rainbow_dark() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(15, 12, 28),                // Deep space indigo (#0f0c1c)
        foreground: egui::Color32::from_rgb(245, 243, 255),           // Prism white (#f5f3ff)
        card: egui::Color32::from_rgb(24, 19, 43),                    // Violet-indigo glass (#18132b)
        card_foreground: egui::Color32::from_rgb(245, 243, 255),
        popover: egui::Color32::from_rgb(24, 19, 43),
        popover_foreground: egui::Color32::from_rgb(245, 243, 255),
        primary: egui::Color32::from_rgb(168, 85, 247),               // Spectrum Purple (purple-500: #a855f7)
        primary_foreground: egui::Color32::from_rgb(255, 255, 255),   // Pure white
        secondary: egui::Color32::from_rgb(6, 182, 212),               // Prism Cyan (cyan-500: #06b6d4)
        secondary_foreground: egui::Color32::from_rgb(255, 255, 255),
        muted: egui::Color32::from_rgb(40, 31, 70),                   // Muted indigo (#281f46)
        muted_foreground: egui::Color32::from_rgb(192, 132, 252),     // Purple-400 (#c084fc)
        accent: egui::Color32::from_rgb(16, 185, 129),                  // Prism Emerald (#10b981)
        accent_foreground: egui::Color32::from_rgb(255, 255, 255),
        destructive: egui::Color32::from_rgb(244, 63, 94),           // Sunset Rose (#f43f5e)
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(62, 47, 105),               // Spectrum border (#3e2f69)
        input: egui::Color32::from_rgb(62, 47, 105),
        ring: egui::Color32::from_rgb(236, 72, 153),                 // Prism Magenta ring (#ec4899)
        radius: 10.0,
    }
}

/// Alias for `rainbow_dark()`.
pub fn rainbow() -> super::shadcn_theme::ShadcnTheme {
    rainbow_dark()
}

/// Creates a soft eye-friendly light theme based on a muted lavender mist background.
pub fn rainbow_light() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(238, 234, 245),           // Soft lavender mist (#eeeaf5)
        foreground: egui::Color32::from_rgb(40, 14, 80),              // Deep purple text
        card: egui::Color32::from_rgb(244, 240, 250),                 // Muted card
        card_foreground: egui::Color32::from_rgb(40, 14, 80),
        popover: egui::Color32::from_rgb(244, 240, 250),
        popover_foreground: egui::Color32::from_rgb(40, 14, 80),
        primary: egui::Color32::from_rgb(147, 51, 234),               // Purple-600 (#9333ea)
        primary_foreground: egui::Color32::from_rgb(255, 255, 255),   // Pure white
        secondary: egui::Color32::from_rgb(222, 226, 245),            // Soft indigo-cyan tint
        secondary_foreground: egui::Color32::from_rgb(60, 50, 180),
        muted: egui::Color32::from_rgb(228, 220, 242),
        muted_foreground: egui::Color32::from_rgb(115, 30, 190),
        accent: egui::Color32::from_rgb(215, 235, 242),
        accent_foreground: egui::Color32::from_rgb(12, 100, 130),
        destructive: egui::Color32::from_rgb(225, 29, 72),
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(208, 200, 226),               // Soft border
        input: egui::Color32::from_rgb(208, 200, 226),
        ring: egui::Color32::from_rgb(217, 70, 239),
        radius: 10.0,
    }
}
