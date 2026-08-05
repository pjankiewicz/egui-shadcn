//! Heatmap / Thermal theme constructors (light and dark) featuring molten magma black, solar amber, and fiery neon orange.
//!
//! Attribution & Contribution:
//! - Design System Tokens: shadcn/ui (https://ui.shadcn.com) by shadcn (MIT License).
//! - Color Palette: Tailwind CSS Amber, Orange & Red color scales (https://tailwindcss.com) by Tailwind Labs (MIT License).

/// Creates a dark theme based on molten magma void black with solar amber and fiery orange thermal accents.
pub fn heatmap_dark() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(10, 5, 5),                 // Deep thermal magma black (#0a0505)
        foreground: egui::Color32::from_rgb(254, 243, 199),           // Solar white-yellow (amber-50: #fef3c7)
        card: egui::Color32::from_rgb(22, 10, 10),                    // Infrared void card (#160a0a)
        card_foreground: egui::Color32::from_rgb(254, 243, 199),
        popover: egui::Color32::from_rgb(22, 10, 10),
        popover_foreground: egui::Color32::from_rgb(254, 243, 199),
        primary: egui::Color32::from_rgb(245, 158, 11),               // Molten Amber (amber-500: #f59e0b)
        primary_foreground: egui::Color32::from_rgb(24, 10, 0),       // Thermal core black
        secondary: egui::Color32::from_rgb(45, 18, 12),               // Fiery hot orange-red (#2d120c)
        secondary_foreground: egui::Color32::from_rgb(254, 243, 199),
        muted: egui::Color32::from_rgb(45, 18, 12),
        muted_foreground: egui::Color32::from_rgb(251, 191, 36),     // Amber-400 (#fbbf24)
        accent: egui::Color32::from_rgb(70, 24, 14),                  // Solar flare accent (#46180e)
        accent_foreground: egui::Color32::from_rgb(253, 230, 138),    // Amber-200 (#fde68a)
        destructive: egui::Color32::from_rgb(239, 68, 68),           // Inferno Red (#ef4444)
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(58, 22, 16),               // Thermal border (#3a1610)
        input: egui::Color32::from_rgb(58, 22, 16),
        ring: egui::Color32::from_rgb(249, 115, 22),                 // Neon Fiery Orange ring (#f97316)
        radius: 6.0,
    }
}

/// Alias for `heatmap_dark()`.
pub fn heatmap() -> super::shadcn_theme::ShadcnTheme {
    heatmap_dark()
}

/// Creates a soft eye-friendly light theme based on warm amber sand background.
pub fn heatmap_light() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(244, 238, 222),           // Soft warm amber sand (#f4eede)
        foreground: egui::Color32::from_rgb(60, 22, 3),               // Deep thermal amber
        card: egui::Color32::from_rgb(248, 243, 230),                 // Muted amber card
        card_foreground: egui::Color32::from_rgb(60, 22, 3),
        popover: egui::Color32::from_rgb(248, 243, 230),
        popover_foreground: egui::Color32::from_rgb(60, 22, 3),
        primary: egui::Color32::from_rgb(217, 119, 6),               // Amber-600 (#d97706)
        primary_foreground: egui::Color32::from_rgb(255, 255, 255),   // Pure white
        secondary: egui::Color32::from_rgb(238, 218, 195),            // Muted amber tint
        secondary_foreground: egui::Color32::from_rgb(140, 45, 12),
        muted: egui::Color32::from_rgb(238, 224, 202),
        muted_foreground: egui::Color32::from_rgb(160, 75, 8),
        accent: egui::Color32::from_rgb(232, 210, 185),
        accent_foreground: egui::Color32::from_rgb(140, 45, 12),
        destructive: egui::Color32::from_rgb(220, 38, 38),
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(222, 198, 168),               // Soft border
        input: egui::Color32::from_rgb(222, 198, 168),
        ring: egui::Color32::from_rgb(217, 119, 6),
        radius: 6.0,
    }
}
