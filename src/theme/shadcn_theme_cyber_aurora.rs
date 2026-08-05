//! Cyber Aurora theme constructors (light and dark) featuring abyssal synthwave void, bioluminescent mint cyan, and radioactive fuchsia accents.
//!
//! Attribution & Contribution:
//! - Original Concept: "Cyber Aurora" bioluminescent synthwave design palette created for egui-shadcn.
//! - Design System Tokens: shadcn/ui (https://ui.shadcn.com) by shadcn (MIT License).
//! - Color Scales: Tailwind CSS Cyan, Teal, Indigo & Fuchsia color scales (https://tailwindcss.com) by Tailwind Labs (MIT License).

/// Creates an ultra-creative dark theme combining abyssal synthwave indigo-black with bioluminescent neon cyan & radioactive pink accents.
pub fn cyber_aurora_dark() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(6, 9, 24),                // Abyssal synthwave void (#060918)
        foreground: egui::Color32::from_rgb(224, 255, 255),           // Electric ice mint white (#e0ffff)
        card: egui::Color32::from_rgb(13, 18, 41),                    // Bioluminescent glass (#0d1229)
        card_foreground: egui::Color32::from_rgb(224, 255, 255),
        popover: egui::Color32::from_rgb(13, 18, 41),
        popover_foreground: egui::Color32::from_rgb(224, 255, 255),
        primary: egui::Color32::from_rgb(34, 211, 238),               // Neon Mint Cyan (cyan-400: #22d3ee)
        primary_foreground: egui::Color32::from_rgb(2, 24, 38),       // Deep abyssal core
        secondary: egui::Color32::from_rgb(28, 37, 77),               // Cyber navy (#1c254d)
        secondary_foreground: egui::Color32::from_rgb(224, 255, 255),
        muted: egui::Color32::from_rgb(28, 37, 77),
        muted_foreground: egui::Color32::from_rgb(129, 140, 248),     // Electric Indigo-400 (#818cf8)
        accent: egui::Color32::from_rgb(45, 34, 89),                  // Synth-fuchsia tint (#2d2259)
        accent_foreground: egui::Color32::from_rgb(244, 114, 182),    // Radioactive Pink (#f472b6)
        destructive: egui::Color32::from_rgb(244, 63, 94),           // Neon Crimson (#f43f5e)
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(38, 51, 102),               // Holographic glass stroke (#263366)
        input: egui::Color32::from_rgb(38, 51, 102),
        ring: egui::Color32::from_rgb(192, 132, 252),                 // Neon Hologram Violet Ring (#c084fc)
        radius: 12.0,
    }
}

/// Alias for `cyber_aurora_dark()`.
pub fn cyber_aurora() -> super::shadcn_theme::ShadcnTheme {
    cyber_aurora_dark()
}

/// Creates a soft eye-friendly light theme based on frosted mint aurora mist with electric teal & indigo elements.
pub fn cyber_aurora_light() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(230, 242, 240),           // Soft frosted mint aurora (#e0f2f0)
        foreground: egui::Color32::from_rgb(15, 23, 42),              // Abyssal slate
        card: egui::Color32::from_rgb(238, 248, 246),                 // Muted mint card
        card_foreground: egui::Color32::from_rgb(15, 23, 42),
        popover: egui::Color32::from_rgb(238, 248, 246),
        popover_foreground: egui::Color32::from_rgb(15, 23, 42),
        primary: egui::Color32::from_rgb(13, 148, 136),               // Electric Teal-600 (#0d9488)
        primary_foreground: egui::Color32::from_rgb(255, 255, 255),   // Pure white
        secondary: egui::Color32::from_rgb(212, 235, 230),            // Muted teal tint
        secondary_foreground: egui::Color32::from_rgb(15, 118, 110),
        muted: egui::Color32::from_rgb(218, 226, 242),                // Muted indigo tint
        muted_foreground: egui::Color32::from_rgb(60, 50, 190),
        accent: egui::Color32::from_rgb(235, 218, 230),               // Muted fuchsia tint
        accent_foreground: egui::Color32::from_rgb(170, 20, 80),
        destructive: egui::Color32::from_rgb(225, 29, 72),
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(195, 222, 216),               // Soft border
        input: egui::Color32::from_rgb(195, 222, 216),
        ring: egui::Color32::from_rgb(20, 184, 166),
        radius: 12.0,
    }
}
