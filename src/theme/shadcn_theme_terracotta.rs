//! Terracotta / Brown-Red theme constructors (light and dark) featuring warm mahogany, sienna, and burnt copper.
//!
//! Attribution & Contribution:
//! - Design System Tokens: shadcn/ui (https://ui.shadcn.com) by shadcn (MIT License).
//! - Color Palette: Tailwind CSS Orange, Amber & Stone color scales (https://tailwindcss.com) by Tailwind Labs (MIT License).

/// Creates a dark theme based on dark mahogany espresso with warm terracotta rust accents.
pub fn terracotta_dark() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(24, 14, 12),                // Dark mahogany espresso (#180e0c)
        foreground: egui::Color32::from_rgb(250, 242, 238),           // Warm cream (#faf2ee)
        card: egui::Color32::from_rgb(36, 21, 18),                    // Dark sienna card (#241512)
        card_foreground: egui::Color32::from_rgb(250, 242, 238),
        popover: egui::Color32::from_rgb(36, 21, 18),
        popover_foreground: egui::Color32::from_rgb(250, 242, 238),
        primary: egui::Color32::from_rgb(194, 65, 12),               // Terracotta rust (orange-700: #c2410c)
        primary_foreground: egui::Color32::from_rgb(255, 247, 237),   // Orange-50 (#fff7ed)
        secondary: egui::Color32::from_rgb(58, 33, 27),               // Dark clay (#3a211b)
        secondary_foreground: egui::Color32::from_rgb(250, 242, 238),
        muted: egui::Color32::from_rgb(58, 33, 27),
        muted_foreground: egui::Color32::from_rgb(214, 140, 118),     // Warm ochre (#d68c76)
        accent: egui::Color32::from_rgb(76, 42, 34),                  // Copper clay (#4c2a22)
        accent_foreground: egui::Color32::from_rgb(255, 237, 213),
        destructive: egui::Color32::from_rgb(185, 28, 28),           // Dark crimson (#b91c1c)
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(64, 38, 31),               // Deep sienna border (#40261f)
        input: egui::Color32::from_rgb(64, 38, 31),
        ring: egui::Color32::from_rgb(234, 88, 12),                 // Vibrant orange rust ring (#ea580c)
        radius: 8.0,
    }
}

/// Alias for `terracotta_dark()`.
pub fn terracotta() -> super::shadcn_theme::ShadcnTheme {
    terracotta_dark()
}

/// Creates a soft eye-friendly light theme based on warm terracotta clay mist.
pub fn terracotta_light() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(244, 234, 228),           // Soft warm clay mist (#f4eae4)
        foreground: egui::Color32::from_rgb(55, 18, 10),              // Dark mahogany
        card: egui::Color32::from_rgb(248, 240, 235),                 // Muted clay card
        card_foreground: egui::Color32::from_rgb(55, 18, 10),
        popover: egui::Color32::from_rgb(248, 240, 235),
        popover_foreground: egui::Color32::from_rgb(55, 18, 10),
        primary: egui::Color32::from_rgb(194, 65, 12),               // Terracotta (#c2410c)
        primary_foreground: egui::Color32::from_rgb(255, 255, 255),   // Pure white
        secondary: egui::Color32::from_rgb(238, 220, 210),            // Muted terracotta tint
        secondary_foreground: egui::Color32::from_rgb(115, 38, 16),
        muted: egui::Color32::from_rgb(238, 220, 210),
        muted_foreground: egui::Color32::from_rgb(140, 65, 45),
        accent: egui::Color32::from_rgb(230, 210, 198),
        accent_foreground: egui::Color32::from_rgb(115, 38, 16),
        destructive: egui::Color32::from_rgb(185, 28, 28),
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(220, 196, 182),               // Soft border
        input: egui::Color32::from_rgb(220, 196, 182),
        ring: egui::Color32::from_rgb(194, 65, 12),
        radius: 8.0,
    }
}
