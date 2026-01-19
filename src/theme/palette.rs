//! Theme palettes

use crate::ThemeColor;

/// Windows Fluent light theme palette
///
/// Source: https://storybooks.fluentui.dev/react/?path=/docs/theme-colors--docs
pub const FLUENT_LIGHT: ThemePalette = ThemePalette {
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
pub const FLUENT_DARK: ThemePalette = ThemePalette {
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
pub const AQUA_LIGHT: ThemePalette = ThemePalette {
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
pub const AQUA_DARK: ThemePalette = ThemePalette {
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
pub const ADWAITA_LIGHT: ThemePalette = ThemePalette {
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
pub const ADWAITA_DARK: ThemePalette = ThemePalette {
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
pub const BREEZE_LIGHT: ThemePalette = ThemePalette {
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
pub const BREEZE_DARK: ThemePalette = ThemePalette {
    background: ThemeColor::from_rgb8(20, 22, 24),
    foreground: ThemeColor::from_rgb8(252, 252, 252),
    accent: ThemeColor::from_rgb8(61, 174, 233),
    success: ThemeColor::from_rgb8(39, 174, 96),
    warning: ThemeColor::from_rgb8(246, 116, 0),
    danger: ThemeColor::from_rgb8(218, 68, 83),
};

/// Theme Palette
#[derive(Debug, Clone, Copy, PartialEq)]
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
