//! Pink theme constructors (light and dark) featuring hot pink, fuchsia, and deep plum tokens.
//!
//! Attribution & Contribution:
//! - Design System Tokens: shadcn/ui (https://ui.shadcn.com) by shadcn (MIT License).
//! - Color Palette: Tailwind CSS Pink & Rose color scales (https://tailwindcss.com) by Tailwind Labs (MIT License).

/// Creates a dark theme based on a deep plum background with hot pink & fuchsia primary elements.
pub fn pink_dark() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(18, 10, 15),                // Deep velvet plum-black (#120a0f)
        foreground: egui::Color32::from_rgb(253, 242, 248),           // Pink-50 (#fdf2f8)
        card: egui::Color32::from_rgb(28, 16, 24),                    // Dark plum card (#1c1018)
        card_foreground: egui::Color32::from_rgb(253, 242, 248),
        popover: egui::Color32::from_rgb(28, 16, 24),
        popover_foreground: egui::Color32::from_rgb(253, 242, 248),
        primary: egui::Color32::from_rgb(236, 72, 153),               // Hot Pink (pink-500: #ec4899)
        primary_foreground: egui::Color32::from_rgb(255, 255, 255),   // Pure white
        secondary: egui::Color32::from_rgb(48, 26, 40),               // Rose-950 (#301a28)
        secondary_foreground: egui::Color32::from_rgb(253, 242, 248),
        muted: egui::Color32::from_rgb(48, 26, 40),
        muted_foreground: egui::Color32::from_rgb(244, 114, 182),     // Pink-400 (#f472b6)
        accent: egui::Color32::from_rgb(64, 30, 52),                  // Magenta accent tint (#401e34)
        accent_foreground: egui::Color32::from_rgb(253, 242, 248),
        destructive: egui::Color32::from_rgb(225, 29, 72),           // Rose-600 (#e11d48)
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(52, 28, 44),               // Deep rose border (#341c2c)
        input: egui::Color32::from_rgb(52, 28, 44),
        ring: egui::Color32::from_rgb(244, 114, 182),                 // Pink-400 ring (#f472b6)
        radius: 8.0,
    }
}

/// Alias for `pink_dark()`.
pub fn pink() -> super::shadcn_theme::ShadcnTheme {
    pink_dark()
}

/// Creates a soft eye-friendly light theme based on a muted blush rose background.
pub fn pink_light() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(244, 230, 238),           // Soft muted blush rose (#f4e6ee)
        foreground: egui::Color32::from_rgb(64, 12, 40),              // Dark fuchsia text
        card: egui::Color32::from_rgb(248, 237, 243),                 // Muted blush card
        card_foreground: egui::Color32::from_rgb(64, 12, 40),
        popover: egui::Color32::from_rgb(248, 237, 243),
        popover_foreground: egui::Color32::from_rgb(64, 12, 40),
        primary: egui::Color32::from_rgb(219, 39, 119),               // Vibrant Pink-600 (#db2777)
        primary_foreground: egui::Color32::from_rgb(255, 255, 255),   // Pure white
        secondary: egui::Color32::from_rgb(238, 218, 230),            // Muted pink tint
        secondary_foreground: egui::Color32::from_rgb(140, 20, 70),
        muted: egui::Color32::from_rgb(238, 218, 230),
        muted_foreground: egui::Color32::from_rgb(160, 24, 80),
        accent: egui::Color32::from_rgb(230, 206, 222),
        accent_foreground: egui::Color32::from_rgb(140, 20, 70),
        destructive: egui::Color32::from_rgb(225, 29, 72),
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(222, 192, 212),               // Soft border
        input: egui::Color32::from_rgb(222, 192, 212),
        ring: egui::Color32::from_rgb(219, 39, 119),
        radius: 8.0,
    }
}
