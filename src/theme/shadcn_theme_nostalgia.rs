//! Nostalgia / Childhood Memories theme constructors (light and dark) featuring storybook parchment, pastel crayons, and cozy bedtime dusk.
//!
//! Attribution & Contribution:
//! - Original Concept: "Childhood Nostalgia" storybook & crayon design palette created for egui-shadcn.
//! - Design System Tokens: shadcn/ui (https://ui.shadcn.com) by shadcn (MIT License).
//! - Color Scales: Tailwind CSS Amber, Orange, Violet, Sky & Rose color scales (https://tailwindcss.com) by Tailwind Labs (MIT License).

/// Creates a dark theme evoking nostalgic childhood memories: a cozy twilight fort with warm creamsicle orange, pastel lavender, and night-light yellow accents.
pub fn nostalgia_dark() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(20, 18, 30),                // Cozy twilight dusk (#14121e)
        foreground: egui::Color32::from_rgb(250, 245, 235),           // Soft parchment (#faf5eb)
        card: egui::Color32::from_rgb(32, 28, 46),                    // Night fort card (#201c2e)
        card_foreground: egui::Color32::from_rgb(250, 245, 235),
        popover: egui::Color32::from_rgb(32, 28, 46),
        popover_foreground: egui::Color32::from_rgb(250, 245, 235),
        primary: egui::Color32::from_rgb(251, 146, 60),               // Creamsicle Orange (orange-400: #fb923c)
        primary_foreground: egui::Color32::from_rgb(30, 20, 10),       // Warm core
        secondary: egui::Color32::from_rgb(52, 44, 74),               // Cozy grape-violet (#342c4a)
        secondary_foreground: egui::Color32::from_rgb(250, 245, 235),
        muted: egui::Color32::from_rgb(52, 44, 74),
        muted_foreground: egui::Color32::from_rgb(196, 181, 253),     // Soft pastel lavender (violet-300: #c4b5fd)
        accent: egui::Color32::from_rgb(56, 189, 248),                  // Playful sky blue (sky-400: #38bdf8)
        accent_foreground: egui::Color32::from_rgb(15, 23, 42),
        destructive: egui::Color32::from_rgb(248, 113, 113),           // Bubblegum Coral (red-400: #f87171)
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(60, 50, 85),               // Soft dusk border (#3c3255)
        input: egui::Color32::from_rgb(60, 50, 85),
        ring: egui::Color32::from_rgb(253, 224, 71),                 // Night-light butter yellow (yellow-300: #fde047)
        radius: 12.0,
    }
}

/// Alias for `nostalgia_dark()`.
pub fn nostalgia() -> super::shadcn_theme::ShadcnTheme {
    nostalgia_dark()
}

/// Creates a soft eye-friendly light theme evoking nostalgic childhood memories: warm storybook parchment paper, butter yellow, sky blue, and creamsicle orange.
pub fn nostalgia_light() -> super::shadcn_theme::ShadcnTheme {
    super::shadcn_theme::ShadcnTheme {
        background: egui::Color32::from_rgb(244, 238, 226),           // Soft warm storybook parchment paper (#f4eee2)
        foreground: egui::Color32::from_rgb(45, 35, 55),              // Warm storybook ink
        card: egui::Color32::from_rgb(248, 243, 234),                 // Muted parchment card
        card_foreground: egui::Color32::from_rgb(45, 35, 55),
        popover: egui::Color32::from_rgb(248, 243, 234),
        popover_foreground: egui::Color32::from_rgb(45, 35, 55),
        primary: egui::Color32::from_rgb(249, 115, 22),               // Creamsicle Orange (#f97316)
        primary_foreground: egui::Color32::from_rgb(255, 255, 255),   // Pure white
        secondary: egui::Color32::from_rgb(238, 222, 185),            // Soft butter yellow tint
        secondary_foreground: egui::Color32::from_rgb(130, 55, 10),
        muted: egui::Color32::from_rgb(232, 218, 242),                // Muted pastel lavender
        muted_foreground: egui::Color32::from_rgb(110, 25, 180),
        accent: egui::Color32::from_rgb(212, 228, 242),               // Muted pastel sky blue
        accent_foreground: egui::Color32::from_rgb(2, 90, 140),
        destructive: egui::Color32::from_rgb(239, 68, 68),
        destructive_foreground: egui::Color32::from_rgb(255, 255, 255),
        border: egui::Color32::from_rgb(226, 210, 192),               // Soft border
        input: egui::Color32::from_rgb(226, 210, 192),
        ring: egui::Color32::from_rgb(249, 115, 22),
        radius: 12.0,
    }
}
