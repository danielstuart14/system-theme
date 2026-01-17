//! Theme palettes

use crate::ThemeKind;

use super::{ThemeColor, ThemeContrast, ThemeScheme};

/// Windows Fluent light theme palette
///
/// Source: https://storybooks.fluentui.dev/react/?path=/docs/theme-colors--docs
const FLUENT_LIGHT: ThemePalette = ThemePalette {
    background: ThemeColor::from_rgb8(250, 250, 250),
    foreground: ThemeColor::from_rgb8(36, 36, 36),
    accent: ThemeColor::from_rgb8(15, 108, 189),
    success: ThemeColor::from_rgb8(14, 112, 14),
    warning: ThemeColor::from_rgb8(188, 75, 9),
    danger: ThemeColor::from_rgb8(177, 14, 28),
};

/// Windows Fluent dark theme palette
///
/// Source: https://storybooks.fluentui.dev/react/?path=/docs/theme-colors--docs
const FLUENT_DARK: ThemePalette = ThemePalette {
    background: ThemeColor::from_rgb8(31, 31, 31),
    foreground: ThemeColor::from_rgb8(255, 255, 255),
    accent: ThemeColor::from_rgb8(71, 158, 245),
    success: ThemeColor::from_rgb8(84, 176, 84),
    warning: ThemeColor::from_rgb8(250, 160, 107),
    danger: ThemeColor::from_rgb8(220, 98, 109),
};

/// Apple Aqua light theme palette
///
/// Source: https://developer.apple.com/design/human-interface-guidelines/color
const AQUA_LIGHT: ThemePalette = ThemePalette {
    background: ThemeColor::from_rgb8(229, 229, 234),
    foreground: ThemeColor::from_rgb8(28, 28, 30),
    accent: ThemeColor::from_rgb8(0, 136, 255),
    success: ThemeColor::from_rgb8(52, 199, 89),
    warning: ThemeColor::from_rgb8(255, 141, 40),
    danger: ThemeColor::from_rgb8(255, 56, 60),
};

/// Apple Aqua dark theme palette
///
/// Source: https://developer.apple.com/design/human-interface-guidelines/color
const AQUA_DARK: ThemePalette = ThemePalette {
    background: ThemeColor::from_rgb8(44, 44, 46),
    foreground: ThemeColor::from_rgb8(242, 242, 247),
    accent: ThemeColor::from_rgb8(0, 145, 255),
    success: ThemeColor::from_rgb8(48, 209, 88),
    warning: ThemeColor::from_rgb8(255, 146, 48),
    danger: ThemeColor::from_rgb8(255, 66, 69),
};

/// GNOME Adwaita light theme palette
///
/// Source: https://github.com/FedoraQt/QGnomePlatform/blob/master/src/color-schemes/Adwaita.colors
const ADWAITA_LIGHT: ThemePalette = ThemePalette {
    background: ThemeColor::from_rgb8(246, 245, 244),
    foreground: ThemeColor::from_rgb8(25, 25, 25),
    accent: ThemeColor::from_rgb8(61, 174, 233),
    success: ThemeColor::from_rgb8(39, 174, 96),
    warning: ThemeColor::from_rgb8(246, 116, 0),
    danger: ThemeColor::from_rgb8(218, 68, 83),
};

/// GNOME Adwaita dark theme palette
///
/// Source: https://github.com/FedoraQt/QGnomePlatform/blob/master/src/color-schemes/AdwaitaDark.colors
const ADWAITA_DARK: ThemePalette = ThemePalette {
    background: ThemeColor::from_rgb8(45, 45, 45),
    foreground: ThemeColor::from_rgb8(255, 255, 255),
    accent: ThemeColor::from_rgb8(61, 174, 233),
    success: ThemeColor::from_rgb8(39, 174, 96),
    warning: ThemeColor::from_rgb8(246, 116, 0),
    danger: ThemeColor::from_rgb8(218, 68, 83),
};

/// KDE Breeze light theme palette
///
/// Source: https://github.com/KDE/breeze/blob/master/colors/BreezeLight.colors
const BREEZE_LIGHT: ThemePalette = ThemePalette {
    background: ThemeColor::from_rgb8(255, 255, 255),
    foreground: ThemeColor::from_rgb8(35, 38, 41),
    accent: ThemeColor::from_rgb8(61, 174, 233),
    success: ThemeColor::from_rgb8(39, 174, 96),
    warning: ThemeColor::from_rgb8(246, 116, 0),
    danger: ThemeColor::from_rgb8(218, 68, 83),
};

/// KDE Breeze dark theme palette
///
/// Source: https://github.com/KDE/breeze/blob/master/colors/BreezeDark.colors
const BREEZE_DARK: ThemePalette = ThemePalette {
    background: ThemeColor::from_rgb8(20, 22, 24),
    foreground: ThemeColor::from_rgb8(252, 252, 252),
    accent: ThemeColor::from_rgb8(61, 174, 233),
    success: ThemeColor::from_rgb8(39, 174, 96),
    warning: ThemeColor::from_rgb8(246, 116, 0),
    danger: ThemeColor::from_rgb8(218, 68, 83),
};

/// Theme Palette
pub struct ThemePalette {
    /// Background color
    pub background: ThemeColor,
    /// Foreground color
    pub foreground: ThemeColor,
    /// Accent color
    pub accent: ThemeColor,
    /// Success color
    pub success: ThemeColor,
    /// Warning color
    pub warning: ThemeColor,
    /// Danger color
    pub danger: ThemeColor,
}

impl ThemePalette {
    /// Get the system palette for the given theme kind, scheme, and contrast.
    pub fn system_palette(kind: ThemeKind, scheme: ThemeScheme, contrast: ThemeContrast) -> Self {
        let mut palette = match kind {
            ThemeKind::Windows => match scheme {
                ThemeScheme::Light => FLUENT_LIGHT,
                ThemeScheme::Dark => FLUENT_DARK,
            },
            ThemeKind::MacOS => match scheme {
                ThemeScheme::Light => AQUA_LIGHT,
                ThemeScheme::Dark => AQUA_DARK,
            },
            ThemeKind::Gtk => match scheme {
                ThemeScheme::Light => ADWAITA_LIGHT,
                ThemeScheme::Dark => ADWAITA_DARK,
            },
            ThemeKind::Qt => match scheme {
                ThemeScheme::Light => BREEZE_LIGHT,
                ThemeScheme::Dark => BREEZE_DARK,
            },
        };

        // Change background/foreground colors if high contrast
        if contrast == ThemeContrast::High {
            match scheme {
                ThemeScheme::Light => {
                    palette.background = ThemeColor::WHITE;
                    palette.foreground = ThemeColor::BLACK;
                }
                ThemeScheme::Dark => {
                    palette.background = ThemeColor::BLACK;
                    palette.foreground = ThemeColor::WHITE;
                }
            }
        }

        palette
    }
}
