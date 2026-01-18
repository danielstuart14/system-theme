//! Theme definitions
mod palette;

use std::fmt::Debug;

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
#[derive(PartialEq, Clone, Copy)]
pub struct ThemeColor {
    /// Red component (0.0 - 1.0)
    pub red: f32,
    /// Green component (0.0 - 1.0)
    pub green: f32,
    /// Blue component (0.0 - 1.0)
    pub blue: f32,
}

impl Debug for ThemeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{:02X}{:02X}{:02X}",
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8
        )
    }
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
}

/// Theme kind
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ThemeKind {
    /// Microsoft Windows
    #[cfg_attr(target_os = "windows", default)]
    Windows,
    /// Apple MacOS
    #[cfg_attr(target_os = "macos", default)]
    MacOS,
    /// GTK (GNOME)
    #[cfg_attr(not(any(target_os = "windows", target_os = "macos")), default)]
    Gtk,
    /// Qt (KDE)
    Qt,
}
