//! Theme definitions
mod palette;

#[doc(inline)]
pub use palette::ThemePalette;

/// Theme scheme
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ThemeScheme {
    /// Light mode
    Light,
    /// Dark mode
    #[default]
    Dark,
}

/// Theme contrast
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ThemeContrast {
    /// Normal contrast
    #[default]
    Normal,
    /// High contrast
    High,
}

/// Theme color
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ThemeColor {
    /// Red component (0.0 - 1.0)
    pub red: f32,
    /// Green component (0.0 - 1.0)
    pub green: f32,
    /// Blue component (0.0 - 1.0)
    pub blue: f32,
}

impl ThemeColor {
    /// White color (#FFFFFF)
    pub const WHITE: Self = Self {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };

    /// Black color (#000000)
    pub const BLACK: Self = Self {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    /// Create a color from RGB 8-bit values.
    pub const fn from_rgb8(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red: red as f32 / 255.0,
            green: green as f32 / 255.0,
            blue: blue as f32 / 255.0,
        }
    }

    /// Check if a color is dark.
    pub fn is_dark(&self) -> bool {
        // Conversion from: https://en.wikipedia.org/wiki/Oklab_color_space#Conversions_between_color_spaces

        // Convert RGB to LMS
        let l = 0.41222146 * self.red + 0.53633255 * self.green + 0.051445995 * self.blue;
        let m = 0.2119035 * self.red + 0.6806995 * self.green + 0.10739696 * self.blue;
        let s = 0.08830246 * self.red + 0.28171885 * self.green + 0.6299787 * self.blue;

        // Convert LMS to Oklab perceptual lightness
        let oklab_l = 0.21045426 * l.cbrt() + 0.7936178 * m.cbrt() - 0.004072047 * s.cbrt();

        // A color is considered dark if its Oklab perceptual lightness is below half the scale
        oklab_l < 0.5
    }
}

/// Theme kind
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ThemeKind {
    /// Microsoft Windows
    Windows,
    /// Apple MacOS
    MacOS,
    /// GTK (GNOME)
    #[default]
    Gtk,
    /// Qt (KDE)
    Qt,
}
